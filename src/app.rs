use crate::{
    collection::{Action, Collection},
    components::error_popup,
    environments::{self, Environment, TempEnv},
    layout,
    main_windows::{key_registry, ChangeEvent, MainWindows},
    request::HttpVerb,
    trace_dbg,
};
use crate::{
    components::{
        AddressBarComponent, RequestTabComponent, RequestsComponent, ResponseTabComponent,
    },
    keys::keys::{
        is_navigation, is_quit, transform, Event as AppEvent, CLOSE_COLLECTIONS, NAV_DOWN,
        NAV_LEFT, NAV_RIGHT, NAV_UP, OPEN_COLLECTIONS, OPEN_ENVIRONMENTS,
    },
};

use crossterm::event::{self, Event};
use ratatui::{backend::Backend, Frame, Terminal};
use regex::Regex;
use reqwest::{header::HeaderMap, Response};
use serde_json::{self};
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
    io::{BufReader, Error as ioError, Read, Write},
    str::from_utf8,
};

const ENV_PATH: &str = "environments";
const COLLECTION_PATH: &str = "collections";
const DATA_DIRECTORY: &str = "/home/babak/.config/restopher";
const START_ENV_TOKEN: &str = "{{";
const END_ENV_TOKEN: &str = "}}";

#[derive(Debug)]
pub enum Error {
    NoRequestErr(usize),
    ReqwestErr(reqwest::Error),
    JsonErr(serde_json::Error),
    HeaderIsNotString,
    FileOperationsErr(std::io::Error),
    ParamIsNotString,
}
impl Error {
    fn from_reqwest(e: reqwest::Error) -> Self {
        Error::ReqwestErr(e)
    }
    fn from_serde(e: serde_json::Error) -> Self {
        Error::JsonErr(e)
    }
    fn to_string(&self) -> String {
        match self {
            Error::NoRequestErr(_) => "no request error".to_string(),
            Error::ReqwestErr(e) => e.to_string(),
            Error::JsonErr(e) => e.to_string(),
            Error::HeaderIsNotString => "header is not string".to_string(),
            Error::FileOperationsErr(e) => e.to_string(),
            Error::ParamIsNotString => "param is not string".to_string(),
        }
    }
}

impl From<ioError> for Error {
    fn from(e: ioError) -> Self {
        Error::FileOperationsErr(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonErr(e)
    }
}

pub struct App<'a> {
    client: reqwest::Client,
    requests: Vec<super::request::Request>,
    main_window: MainWindows,

    req_tabs: RequestTabComponent<'static>,
    resp_tabs: ResponseTabComponent,
    address_bar: AddressBarComponent,
    requests_component: RequestsComponent,

    current_request_idx: usize,
    error_pop_up: (bool, Option<Error>),

    all_envs: Vec<Environment>,
    temp_envs: Option<environments::TempEnv>,
    current_env_idx: usize, // index of active environments
    collections: Collection<'a>,
    regex_replacer: regex::Regex,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let all_envs = App::load_envs().unwrap();
        let requests = vec![super::request::Request::new()];
        let names = requests.iter().map(|r| r.name()).collect();
        let cols = Collection::default(format!("{}/{}", DATA_DIRECTORY, COLLECTION_PATH));
        App {
            client: reqwest::Client::new(),
            requests,
            current_request_idx: 0,
            error_pop_up: (false, None),
            current_env_idx: 0,
            all_envs,
            temp_envs: None,
            regex_replacer: Regex::new(&format!(
                "{}.*{}",
                regex::escape(START_ENV_TOKEN),
                regex::escape(END_ENV_TOKEN)
            ))
            .unwrap(),
            collections: cols,
            main_window: MainWindows::Main,

            req_tabs: RequestTabComponent::new(),
            resp_tabs: ResponseTabComponent::new(),
            address_bar: AddressBarComponent::new(),
            requests_component: RequestsComponent::new(names, 0),
        }
    }
    pub fn load_envs() -> Result<Vec<Environment>, Error> {
        let path = format!("{}/{}", DATA_DIRECTORY, ENV_PATH);
        let file = fs::File::open(&path);
        match file {
            Ok(f) => {
                if f.metadata()?.is_dir() {
                    let mut result = Vec::<Environment>::new();
                    for entry in fs::read_dir(path)? {
                        let entry = entry?;
                        match entry.path().extension() {
                            Some(ext) => {
                                if ext == "env" {
                                    result.push(serde_json::from_reader(fs::File::open(
                                        entry.path(),
                                    )?)?);
                                };
                            }
                            None => (),
                        };
                    }
                    Ok(result)
                } else {
                    let mut reader = BufReader::new(f);
                    let mut buffer = Vec::new();
                    reader.read_to_end(&mut buffer)?;
                    Ok(serde_json::from_str(from_utf8(&buffer).unwrap())?)
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(vec![Environment::new("default".to_string())]),
                _ => Err(Error::FileOperationsErr(e)),
            },
        }
    }
    pub fn reload_envs(&mut self) -> Result<(), Error> {
        self.all_envs = Self::load_envs()?;
        Ok(())
    }
    pub async fn run<B: Backend>(mut self, term: &mut Terminal<B>) -> () {
        term.draw(|f| self.ui(f)).unwrap();
        loop {
            match self.update().await {
                Ok(ss) => {
                    if let Some(s) = ss {
                        if s == "quit" {
                            break;
                        }
                    }
                }
                Err(e) => {
                    self.error_pop_up = (true, Some(e));
                }
            }
            term.draw(|f| self.ui(f)).unwrap();
        }
    }
    pub async fn update(&mut self) -> Result<Option<String>, Error> {
        if let Event::Key(key) = event::read().unwrap() {
            let even = transform(key);
            if is_quit(&even) {
                return Ok(Some("quit".to_string()));
            }
            if is_navigation(&even) && matches!(self.main_window, MainWindows::Main) {
                self.navigation(&even);
                return Ok(None);
            }
            match self.main_window {
                MainWindows::Main => {
                    if matches!(&even, OPEN_COLLECTIONS) {
                        self.main_window = MainWindows::Collections;
                        return Ok(None);
                    };
                    if matches!(&even, OPEN_ENVIRONMENTS) {
                        self.main_window = MainWindows::Environments;
                        self.temp_envs =
                            Some(TempEnv::new(self.all_envs.clone(), self.current_env_idx));
                        return Ok(None);
                    }
                }
                MainWindows::Environments => {
                    if let Some(temp) = &mut self.temp_envs {
                        let result = temp.update(&even);
                        if result.1 {
                            return Ok(None);
                        }
                        self.main_window = MainWindows::Main;
                        if let Some(modified_env) = result.0 {
                            App::save_env(modified_env).unwrap();
                            self.reload_envs()?;
                        }
                        return Ok(None);
                    }
                }
                MainWindows::Collections => {
                    if &even == CLOSE_COLLECTIONS {
                        self.main_window = MainWindows::Main;
                        return Ok(None);
                    };
                    if let Some((action, paths)) = self.collections.update(&even) {
                        self.main_window = MainWindows::Main;
                        trace_dbg!(level:tracing::Level::INFO, (&action, "aaaaaaaaaaa",&paths));
                        match action {
                            Action::Delete => self.delete_request(paths)?,
                            Action::Create => self.create_new_collection(paths)?,
                            Action::AddRequest => self.modify_collection(paths)?,
                        };
                        self.reload_collections()
                    }
                }
            };
            match key_registry(&even, &self.main_window) {
                ChangeEvent::ChangeRequestTab => {
                    self.req_tabs.update_inner_focus();
                    return Ok(None);
                }
                ChangeEvent::ChangeResponseTab => {
                    self.resp_tabs.update_inner_focus();
                    return Ok(None);
                }
                ChangeEvent::SaveRequest => {
                    self.save_current_req()?;
                    return Ok(None);
                }
                ChangeEvent::NewRequest => {
                    self.new_request();
                    return Ok(None);
                }
                ChangeEvent::PreRequest => {
                    self.pre_req();
                    return Ok(None);
                }
                ChangeEvent::NextRequest => {
                    self.next_req();
                    return Ok(None);
                }
                ChangeEvent::CallRequest => {
                    match self.call_request().await {
                        Ok(_) => {}
                        Err(e) => {
                            self.error_pop_up = (true, Some(e));
                        }
                    }
                    return Ok(None);
                }
                ChangeEvent::NoChange => (),
            }
            if self.req_tabs.is_focused() {
                self.req_tabs
                    .update(&mut self.requests[self.current_request_idx], even);
                return Ok(None);
            }
            if self.address_bar.is_focused() {
                self.address_bar
                    .update(&mut self.requests[self.current_request_idx], &even);
                return Ok(None);
            }
            if self.resp_tabs.is_focused() {
                self.resp_tabs
                    .update(&mut self.requests[self.current_request_idx], &even);
                return Ok(None);
            }
            if self.requests_component.is_focused() {
                self.requests_component.update(
                    &mut self.requests,
                    &mut self.current_request_idx,
                    &mut self.all_envs,
                    &mut self.current_env_idx,
                    &even,
                );
                return Ok(None);
            }
            return Ok(None);
        }
        return Ok(None);
    }
    fn reload_collections(&mut self) {
        self.collections = Collection::default(format!("{}/{}", DATA_DIRECTORY, COLLECTION_PATH));
    }
    fn ui(&mut self, f: &mut Frame) {
        let lay = layout::AppLayout::new(f.area());
        self.req_tabs
            .draw(f, &self.requests[self.current_request_idx], lay.request);
        self.resp_tabs
            .draw(f, &self.requests[self.current_request_idx], lay.response);
        self.address_bar.draw(
            f,
            &self.requests[self.current_request_idx],
            lay.address_verb,
        );
        self.requests_component.draw(
            f,
            self.requests.iter().map(|r| r.name().clone()).collect(),
            self.all_envs
                .get(self.current_env_idx)
                .map_or("-".to_string(), |e| e.name.clone()),
            self.current_request_idx,
            lay.requests,
        );
        if matches!(self.main_window, MainWindows::Collections) {
            self.collections.draw(f);
        }
        if matches!(self.main_window, MainWindows::Environments) {
            if let Some(temp) = &mut self.temp_envs {
                temp.draw(f, f.area());
            }
        }
        if self.error_pop_up.0 {
            error_popup(f, &self.error_pop_up.1.as_ref().unwrap(), f.area());
            self.error_pop_up.0 = false;
        }
    }
    pub fn new_request(&mut self) {
        self.requests.push(super::Request::new());
        self.current_request_idx = self.requests.len() - 1;
    }
    pub fn next_req(&mut self) {
        self.current_request_idx += 1;
        if self.current_request_idx >= self.requests.len() {
            self.current_request_idx = 0;
        }
    }
    pub fn pre_req(&mut self) {
        if self.current_request_idx == 0 {
            self.current_request_idx = self.requests.len() - 1;
            return;
        };
        self.current_request_idx -= 1;
    }
    pub async fn call_request(&mut self) -> Result<(), Error> {
        let current_request = &self.requests[self.current_request_idx];
        let mut addr = String::new();
        let mut params = HashMap::new();
        let headers = HeaderMap::try_from(&self.replace_envs(current_request.handle_headers()))
            .unwrap_or(HeaderMap::new());
        let mut body = None;
        params = self.replace_envs(current_request.handle_params());
        addr = self.replace_envs(current_request.address().to_string());
        let resp: Response;
        match current_request.verb() {
            HttpVerb::GET => {
                resp = self
                    .client
                    .get(addr)
                    .query(&params)
                    .headers(headers)
                    .send()
                    .await
                    .map_err(|e| Error::ReqwestErr(e))?;
            }
            HttpVerb::POST => {
                body = current_request.handle_json_body()?;
                let mut r = self.client.post(addr).query(&params).headers(headers);
                if let Some(b) = body {
                    r = r.json(&b)
                };
                resp = r.send().await.map_err(|e| Error::ReqwestErr(e))?;
            }
            HttpVerb::PUT => {
                body = current_request.handle_json_body()?;
                let mut r = self.client.put(addr).query(&params).headers(headers);
                if let Some(b) = body {
                    r = r.json(&b)
                };
                resp = r.send().await.map_err(|e| Error::ReqwestErr(e))?;
            }
            HttpVerb::DELETE => {
                resp = self
                    .client
                    .get(addr)
                    .query(&params)
                    .headers(headers)
                    .send()
                    .await
                    .map_err(|e| Error::ReqwestErr(e))?;
            }
        }
        let current_request = &mut self.requests[self.current_request_idx];
        current_request.set_response_status_code(resp.status().as_u16() as i32);
        current_request
            .set_response_headers(&resp.headers().clone())
            .unwrap();
        current_request.set_response_body(resp.text().await.map_err(|e| Error::ReqwestErr(e))?);
        Ok(())
    }
    fn replace_envs<T>(&self, to_replace: T) -> T
    where
        T: Clone + EnvReplacer,
    {
        to_replace.replace_env(
            &self.regex_replacer,
            &self.all_envs[self.current_env_idx].envs,
        )
    }
    pub fn save_env(environments: Vec<Environment>) -> Result<(), Error> {
        let path = format!("{}/{}", DATA_DIRECTORY, ENV_PATH);
        for env in environments.iter() {
            match fs::metadata(path.clone()) {
                Ok(f) => {
                    if f.is_dir() {
                        match fs::File::create(format!("{}/{}.env", path, env.name)) {
                            Ok(mut f) => {
                                let to_write = serde_json::to_vec(&env).unwrap();
                                f.write(&to_write).unwrap();
                            }
                            Err(e) => match fs::File::open(format!("{}/{}.env", path, env.name)) {
                                Ok(mut f) => {
                                    f.write(serde_json::to_vec(&env).unwrap().as_slice())
                                        .unwrap();
                                }
                                Err(e) => return Err(Error::FileOperationsErr(e)),
                            },
                        };
                    }
                }
                Err(e) => return Err(Error::FileOperationsErr(e)),
            }
        }
        Ok(())
    }
    pub fn save_current_req(&mut self) -> Result<(), Error> {
        let req = self
            .requests
            .get(self.current_request_idx)
            .ok_or(Error::NoRequestErr(2))?;
        let name = req.name().clone();
        let cols = &self.collections;
        let path: String = cols
            .get_selected()
            .get(0)
            .unwrap_or(&format!("{}/{}", DATA_DIRECTORY, COLLECTION_PATH))
            .to_string();
        trace_dbg!(level: tracing::Level::INFO, (&path, name.clone()));
        match fs::metadata(path.clone()) {
            Ok(f) => {
                if f.is_dir() {
                    match fs::File::create(format!("{}/{}.rph", path, name)) {
                        Ok(mut f) => {
                            let to_write = serde_json::to_vec(req).unwrap();
                            f.write(&to_write).unwrap();
                        }
                        Err(e) => return Err(Error::FileOperationsErr(e)),
                    };
                }
            }
            Err(e) => return Err(Error::FileOperationsErr(e)),
        };
        Ok(())
    }
    pub fn set_collections(&mut self) {
        self.collections = Collection::default(format!("{}/{}", DATA_DIRECTORY, COLLECTION_PATH));
    }
    fn navigation(&mut self, e: &AppEvent) {
        match e {
            NAV_UP => {
                if self.req_tabs.is_focused() {
                    self.req_tabs
                        .lose_focus(&mut self.requests[self.current_request_idx]);
                    self.address_bar.gain_focus();
                } else if self.address_bar.is_focused() {
                    self.address_bar.lose_focus();
                    self.requests_component.gain_focus();
                } else if self.resp_tabs.is_focused() {
                    self.resp_tabs.lose_focus();
                    self.req_tabs.gain_focus();
                } else if self.requests_component.is_focused() {
                    self.requests_component.lose_focus();
                    self.resp_tabs.gain_focus();
                }
            }
            NAV_DOWN => {
                if self.req_tabs.is_focused() {
                    self.req_tabs
                        .lose_focus(&mut self.requests[self.current_request_idx]);
                    self.resp_tabs.gain_focus();
                } else if self.address_bar.is_focused() {
                    self.address_bar.lose_focus();
                    self.req_tabs.gain_focus();
                } else if self.resp_tabs.is_focused() {
                    self.resp_tabs.lose_focus();
                    self.requests_component.gain_focus();
                } else if self.requests_component.is_focused() {
                    self.requests_component.lose_focus();
                    self.address_bar.gain_focus();
                }
            }
            NAV_LEFT => (),
            NAV_RIGHT => (),
            _ => (),
        }
    }
    fn delete_request(&mut self, paths: Vec<String>) -> Result<(), Error> {
        if let Some(path) = paths.last() {
            match fs::metadata(path.clone()) {
                Ok(f) => {
                    if f.is_file() {
                        fs::remove_file(path.clone())?;
                        return Ok(());
                    } else {
                        let mut is_empty = true;
                        for entry in fs::read_dir(path.clone())? {
                            let entry = entry?;
                            match entry.path().extension() {
                                Some(ext) => {
                                    if ext == "rph" {
                                        fs::remove_file(entry.path())?;
                                    }
                                }
                                None => {
                                    is_empty = false;
                                    continue;
                                }
                            };
                        }
                        if is_empty && *path != format!("{}/{}", DATA_DIRECTORY, COLLECTION_PATH) {
                            fs::remove_dir(path.clone())?;
                        }
                        return Ok(());
                    }
                }
                Err(e) => {
                    self.error_pop_up = (true, Some(Error::FileOperationsErr(e)));
                    return Ok(());
                }
            };
        }
        Err(Error::NoRequestErr(1))
    }
    fn modify_collection(&mut self, paths: Vec<String>) -> Result<(), Error> {
        if let Some(path) = paths.last() {
            match fs::metadata(path.clone()) {
                Ok(f) => {
                    if f.is_file() {
                        self.requests
                            .push(serde_json::from_reader(fs::File::open(path.clone())?)?);
                        return Ok(());
                    } else {
                        for entry in fs::read_dir(path.clone())? {
                            let entry = entry?;
                            match entry.path().extension() {
                                Some(ext) => {
                                    if ext == "rph" {
                                        self.requests.push(serde_json::from_reader(
                                            fs::File::open(entry.path())?,
                                        )?);
                                    }
                                }
                                None => continue,
                            };
                        }
                        return Ok(());
                    }
                }
                Err(e) => {
                    self.error_pop_up = (true, Some(Error::FileOperationsErr(e)));
                    return Ok(());
                }
            };
        }
        Err(Error::NoRequestErr(1))
    }

    fn create_new_collection(&mut self, paths: Vec<String>) -> Result<(), Error> {
        if let Some(path) = paths.last() {
            trace_dbg!(level: tracing::Level::INFO, ("in creating", &paths));
            fs::create_dir(path.clone())?;
            return Ok(());
        };
        trace_dbg!(level: tracing::Level::INFO, ("in NONE", &paths));
        return Err(Error::NoRequestErr(1));
    }
}

trait EnvReplacer {
    fn replace_env(self, pattern: &Regex, replace_kvs: &HashMap<String, String>) -> Self
    where
        Self: Sized,
    {
        self
    }
}
impl EnvReplacer for String {
    fn replace_env(self, pattern: &Regex, replace_kvs: &HashMap<String, String>) -> Self {
        let mut result = self.clone();
        for (idx, matched) in pattern.captures_iter(&self).enumerate() {
            match replace_kvs.get(
                &matched[0]
                    .trim_end_matches(END_ENV_TOKEN)
                    .trim_start_matches(START_ENV_TOKEN)
                    .to_string(),
            ) {
                Some(s) => result = result.replacen(&matched[0], s, 1),
                None => (),
            };
        }
        result
    }
}

impl EnvReplacer for HashMap<String, String, RandomState> {
    fn replace_env(
        self,
        pattern: &Regex,
        replace_kvs: &HashMap<String, String, RandomState>,
    ) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for (key, value) in self.into_iter() {
            let mut new_key = key.clone();
            let mut new_value = value.clone();
            for (idx, matched) in pattern.captures_iter(&key).enumerate() {
                let to_match = &matched[0];
                match replace_kvs.get(
                    &to_match
                        .trim_end_matches(END_ENV_TOKEN)
                        .trim_start_matches(START_ENV_TOKEN)
                        .to_string(),
                ) {
                    Some(s) => {
                        new_key = key.clone().replacen(to_match, s, 1);
                    }
                    None => new_key = key.clone(),
                };
            }
            for (idx, matched) in pattern.captures_iter(&value).enumerate() {
                let to_match = &matched[0];
                match replace_kvs.get(
                    &to_match
                        .trim_end_matches(END_ENV_TOKEN)
                        .trim_start_matches(START_ENV_TOKEN)
                        .to_string(),
                ) {
                    Some(s) => {
                        new_value = value.clone().replacen(to_match, s, 1);
                    }
                    None => new_value = value.clone(),
                };
            }
            result.insert(new_key, new_value);
        }
        result
    }
}
