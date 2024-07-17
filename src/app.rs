use super::Request;
use crate::components::{
    default_block, tabs, AddressBarComponent, EnvironmentsComponent, RequestTabComponent,
    RequestsComponent, ResponseTabComponent,
};
use crate::keys::keys::{
    is_navigation, is_quit, transform, Event as AppEvent, CLOSE_COLLECTIONS, CLOSE_ENVIRONMENTS,
    NAV_DOWN, NAV_LEFT, NAV_RIGHT, NAV_UP, OPEN_COLLECTIONS, OPEN_ENVIRONMENTS,
};
use crate::main_windows::{key_registry, ChangeEvent};
use crate::request::HttpVerb;
use crate::{
    collection::Collection, components::error_popup, environments::Environment,
    main_windows::MainWindows,
};
use crate::{environments, layout};
use crossterm::event::{self, Event, KeyEvent};
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
};
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use ratatui::{widgets, Frame, Terminal};
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::{Response, ResponseBuilderExt};
use serde_json::{self};
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
    io::{BufRead, BufReader, Read, Write},
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
pub struct App<'a> {
    client: reqwest::Client,
    requests: Vec<super::request::Request>,
    main_window: MainWindows,

    req_tabs: RequestTabComponent<'static>,
    resp_tabs: ResponseTabComponent,
    address_bar: AddressBarComponent,
    requests_component: RequestsComponent,
    environments_component: EnvironmentsComponent,

    current_request_idx: usize,
    error_pop_up: (bool, Option<Error>),
    all_envs: Vec<Environment>,
    temp_envs: Option<environments::TempEnv>,
    current_env_idx: usize, // index of active environments
    data_directory: String,
    collections: Collection<'a>,
    regex_replacer: regex::Regex,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut all_envs = match fs::File::open(format!("{}/{}", DATA_DIRECTORY, ENV_PATH)) {
            Ok(f) => {
                if f.metadata().unwrap().is_dir() {
                    let mut result = Vec::<Environment>::new();
                    for entry in fs::read_dir(format!("{}/{}", DATA_DIRECTORY, ENV_PATH)).unwrap() {
                        let entry = entry.unwrap();
                        match entry.path().extension() {
                            Some(ext) => {
                                if ext == "env" {
                                    result.push(
                                        serde_json::from_reader(
                                            fs::File::open(entry.path()).unwrap(),
                                        )
                                        .unwrap(),
                                    );
                                };
                            }
                            None => (),
                        };
                    }
                    result
                } else {
                    let mut reader = BufReader::new(f);
                    let mut buffer = Vec::new();
                    reader.read_to_end(&mut buffer).unwrap();
                    serde_json::from_str(from_utf8(&buffer).unwrap()).unwrap()
                }
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => (),
                    _ => (),
                };
                Vec::new()
            }
        };
        if all_envs.len() == 0 {
            all_envs.push(Environment::new("default".to_string()));
        }
        let mut requests = vec![super::request::Request::new()];
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
            data_directory: DATA_DIRECTORY.to_string(),
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
            environments_component: EnvironmentsComponent::new(),
        }
    }
    pub async fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> () {
        loop {
            term.draw(|f| self.ui(f)).unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                let even = transform(key);
                if is_quit(&even) {
                    panic!("quit");
                }
                if is_navigation(&even) {
                    self.navigation(&even);
                    continue;
                }
                match self.main_window {
                    MainWindows::Main => {
                        if matches!(&even, OPEN_COLLECTIONS) {
                            self.main_window = MainWindows::Collections;
                            continue;
                        };
                        if matches!(&even, OPEN_ENVIRONMENTS) {
                            self.main_window = MainWindows::Environments;
                            match self.all_envs.len() {
                                0 => {
                                    self.temp_envs = Some(environments::TempEnv::new());
                                }
                                _ => {
                                    self.temp_envs =
                                        Some(self.all_envs[self.current_env_idx].clone().into())
                                }
                            }
                            continue;
                        }
                    }
                    MainWindows::Environments => {
                        if &even == CLOSE_ENVIRONMENTS {
                            self.main_window = MainWindows::Main;
                            if let Some(modified_env) =
                                self.temp_envs.as_ref().unwrap().get_modified()
                            {
                                App::save_env(modified_env).unwrap();
                            }
                            continue;
                        };
                        if let Some(temp) = &mut self.temp_envs {
                            temp.update(&even);
                            continue;
                        }
                    }
                    MainWindows::Collections => {
                        if &even == CLOSE_COLLECTIONS {
                            self.main_window = MainWindows::Main;
                            continue;
                        };
                        if let Some(paths) = self.collections.update(&even) {
                            self.main_window = MainWindows::Main;
                            if let Some(path) = paths.last() {
                                match fs::metadata(path.clone()) {
                                    Ok(f) => {
                                        if f.is_file() {
                                            self.requests.push(
                                                serde_json::from_reader(
                                                    fs::File::open(path.clone()).unwrap(),
                                                )
                                                .unwrap(),
                                            );
                                        }
                                        if f.is_dir() {
                                            for entry in fs::read_dir(path.clone()).unwrap() {
                                                let entry = entry.unwrap();
                                                match entry.path().extension() {
                                                    Some(ext) => {
                                                        if ext == "rph" {
                                                            self.requests.push(
                                                                serde_json::from_reader(
                                                                    fs::File::open(entry.path())
                                                                        .unwrap(),
                                                                )
                                                                .unwrap(),
                                                            );
                                                        }
                                                    }
                                                    None => continue,
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        self.error_pop_up =
                                            (true, Some(Error::FileOperationsErr(e)));
                                    }
                                };
                            }
                        };
                    }
                    _ => (),
                };
                match key_registry(&even, &self.main_window) {
                    ChangeEvent::ChangeRequestTab => {
                        self.req_tabs.update_inner_focus();
                        continue;
                    }
                    ChangeEvent::ChangeResponseTab => {
                        self.resp_tabs.update_inner_focus();
                        continue;
                    }
                    ChangeEvent::SaveRequest => {
                        self.save_current_req().unwrap();
                        continue;
                    }
                    ChangeEvent::NewRequest => {
                        self.new_request();
                        continue;
                    }
                    ChangeEvent::PreRequest => {
                        self.pre_req();
                        continue;
                    }
                    ChangeEvent::NextRequest => {
                        self.next_req();
                        continue;
                    }
                    ChangeEvent::CallRequest => {
                        match self.call_request().await {
                            Ok(_) => {}
                            Err(e) => {
                                self.error_pop_up = (true, Some(e));
                            }
                        }
                        continue;
                    }
                    ChangeEvent::NoChange => (),
                }
                if self.req_tabs.is_focused() {
                    self.req_tabs
                        .update(&mut self.requests[self.current_request_idx], even);
                    continue;
                }
                if self.address_bar.is_focused() {
                    self.address_bar
                        .update(&mut self.requests[self.current_request_idx], &even);
                    continue;
                }
                if self.resp_tabs.is_focused() {
                    self.resp_tabs
                        .update(&mut self.requests[self.current_request_idx], &even);
                    continue;
                }
                if self.requests_component.is_focused() {
                    self.requests_component.update(
                        &mut self.requests,
                        &mut self.current_request_idx,
                        &mut self.all_envs,
                        &mut self.current_env_idx,
                        &even,
                    );
                    continue;
                }
            }
        }
    }
    fn ui(&mut self, f: &mut Frame) {
        let lay = layout::AppLayout::new(f.size());
        let t = tabs(
            self.get_req_names()
                .into_iter()
                .map(|s| Span::from(s))
                .collect(),
            "requests",
            self.current_request_idx,
            false,
        )
        .block(default_block("requests", false));

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
                temp.draw(f, f.size());
            }
        }
        if self.error_pop_up.0 {
            error_popup(f, &self.error_pop_up.1.as_ref().unwrap(), f.size());
            self.error_pop_up.0 = false;
        }
    }

    pub fn get_req_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for r in &self.requests {
            result.push({ r.name() });
        }
        result
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
        let mut headers = HeaderMap::try_from(&self.replace_envs(current_request.handle_headers()))
            .unwrap_or(HeaderMap::new());
        let mut body = None;
        params = self.replace_envs(current_request.handle_params());
        addr = self.replace_envs(current_request.address().to_string());
        body = current_request.handle_json_body()?;
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
                let mut r = self.client.post(addr).query(&params).headers(headers);
                if let Some(b) = body {
                    r = r.json(&b)
                };
                resp = r.send().await.map_err(|e| Error::ReqwestErr(e))?;
            }
            HttpVerb::PUT => {
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
    pub fn save_env(env: Environment) -> Result<(), Error> {
        let path = format!("{}/{}", DATA_DIRECTORY, ENV_PATH);
        match fs::metadata(path.clone()) {
            Ok(f) => {
                if f.is_dir() {
                    match fs::File::create(format!("{}/{}.env", path, env.name)) {
                        Ok(mut f) => {
                            let to_write = serde_json::to_vec(&env).unwrap();
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
                    self.req_tabs.lose_focus();
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
                    self.req_tabs.lose_focus();
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
