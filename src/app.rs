use crate::components::{
    self, default_block, tabs, RequestController, ADDRESS, BODY, HEADERS, PARAMS, VERB,
};
use crate::keys::keys::{
    is_navigation, is_quit, transform, Event as AppEvent, NAV_DOWN, NAV_LEFT, NAV_RIGHT, NAV_UP,
};
use crate::main_windows::key_registry;
use crate::utils::app_state::{Section, State, StateItem, REQUESTS};
use crate::{
    components::{error_popup, HttpVerb, ReqBundle, ReqTabs, RespTabs},
    environments::{Environment, KV},
};
use crate::{environments, layout};
use crossterm::event::{self, Event, KeyEvent};
use ratatui::{backend::Backend, layout::{Constraint, Direction, Layout}};
use ratatui::{style::{Color, Style}, text::Span};
use ratatui::widgets::{StatefulWidget, Widget};
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
use tree::{stateful_tree::StatefulTree, Node, TreeItem, TreeState};

const ENV_PATH: &str = "envs";
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
    state: State,
    client: reqwest::Client,
    req_controller: RequestController,

    current_request_idx: usize,
    requests: Vec<ReqBundle>,
    temp_header_param_idx: usize,
    req_tabs: ReqTabs<'a>,
    resp_tabs: RespTabs<'a>,
    error_pop_up: (bool, Option<Error>),
    show_environments: bool,
    all_envs: Vec<Environment>,
    temp_envs: Option<environments::TempEnv>,
    current_env_idx: Option<usize>, // index of active environments
    data_directory: String,
    collections: Option<StatefulTree>,
    has_new_req_name: bool,
    has_new_collection: bool,
    collection_or_name: String,
    regex_replacer: regex::Regex,
}
impl<'a> App<'a> {
    pub fn new() -> Self {
        let rc = RequestController::new();
        let all_envs = match fs::File::open(format!("{}/{}", DATA_DIRECTORY, ENV_PATH)) {
            Ok(f) => {
                let mut reader = BufReader::new(f);
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).unwrap();
                serde_json::from_str(from_utf8(&buffer).unwrap()).unwrap()
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => (),
                    _ => (),
                };
                Vec::new()
            }
        };
        let mut requests = vec![ReqBundle::new()];
        App {
            state: State::default(),
            client: reqwest::Client::new(),
            req_controller: rc,
            requests,
            current_request_idx: 0,
            req_tabs: ReqTabs::new(),
            resp_tabs: RespTabs::new(),
            error_pop_up: (false, None),
            temp_header_param_idx: 0,
            current_env_idx: None,
            show_environments: false,
            all_envs,
            temp_envs: None,
            data_directory: DATA_DIRECTORY.to_string(),
            regex_replacer: Regex::new(&format!(
                "{}.*{}",
                regex::escape(START_ENV_TOKEN),
                regex::escape(END_ENV_TOKEN)
            ))
            .unwrap(),
            collections: None,
            has_new_collection: false,
            has_new_req_name: false,
            collection_or_name: "".to_string(),
        }
    }
    pub async fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> () {
        loop {
            term.draw(|f| self.ui(f)).unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                let even = transform(key, &mut self.state);
                if is_quit(&even) {
                    return;
                }
                if is_navigation(&even) {
                    navigation(&even, &mut self.state);
                    continue;
                }
                match key_registry(&even, &mut self.state) {
                    crate::main_windows::ChangeEvent::ChangeRequestTab => {
                        self.change_request_tab();
                        continue;
                    }
                    crate::main_windows::ChangeEvent::ChangeResponseTab => {
                        self.change_response_tab();
                        continue;
                    }
                    crate::main_windows::ChangeEvent::SaveRequest => {
                        self.save_request();
                        continue;
                    }
                    crate::main_windows::ChangeEvent::NewRequest => {
                        self.new_request();
                        continue;
                    }
                    crate::main_windows::ChangeEvent::PreRequest => {
                        self.pre_req();
                        continue;
                    }
                    crate::main_windows::ChangeEvent::NextRequest => {
                        self.next_req();
                        continue;
                    }
                    crate::main_windows::ChangeEvent::CallRequest => {
                        match self.call_request().await {
                            Ok(_) => {}
                            Err(e) => {
                                self.error_pop_up = (true, Some(e));
                            }
                        }
                        continue;
                    }
                    crate::main_windows::ChangeEvent::NoChange => (),
                }
                match *self.state.last().main_windows() {
                    crate::main_windows::MainWindows::Main => {
                        self.req_controller.handle(
                            even,
                            &mut self.requests[self.current_request_idx],
                            &self.state,
                        );
                    }
                    _ => (),
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
        )
        .block(default_block("requests"));
        RequestController::render(
            f,
            &self.requests[self.current_request_idx],
            lay.request,
            &self.state,
        );
        f.render_widget(self.render_request_tabs(), lay.req_tabs);
        f.render_widget(self.render_response_tabs(), lay.resp_tabs);
        f.render_widget(t, lay.requests);
        if self.error_pop_up.0 {
            error_popup(f, &self.error_pop_up.1.as_ref().unwrap(), f.size());
            self.error_pop_up.0 = false;
        }
    }

    fn render_response_tabs(&self) -> impl Widget + 'a {
        components::tabs(
            self.resp_tabs
                .resp_tabs
                .iter()
                .map(|t| Span::from(t.to_string()))
                .collect(),
            "Request data tabs",
            self.resp_tabs.active_idx(),
        )
    }
    fn render_request_tabs(&self) -> impl Widget + 'a {
        components::tabs(
            self.req_tabs
                .req_tabs
                .iter()
                .map(|t| Span::from(t.to_string()))
                .collect(),
            "Request data tabs",
            self.req_tabs.active_idx(),
        )
    }
    pub fn get_req_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for r in &self.requests {
            result.push({ r.name() });
        }
        result
    }
    pub fn new_request(&mut self) {
        self.requests.push(ReqBundle::new());
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
        current_request.set_response_body(resp.text().await.map_err(|e| Error::ReqwestErr(e))?);
        Ok(())
    }
    pub fn change_request_tab(&mut self) {
        self.req_tabs.next();
        self.state.push(StateItem::new(
            self.state.last().main_windows_clone(),
            self.req_tabs.active().to_section(),
        ));
    }
    pub fn change_response_tab(&mut self) {
        self.resp_tabs.next();
        self.state.push(StateItem::new(
            self.state.last().main_windows_clone(),
            self.resp_tabs.active().to_section(),
        ));
    }
    pub fn save_request(&mut self) {
        //self.requests.push()
    }
    pub fn initiate_temp_envs(&mut self) {
        self.temp_envs = Some(environments::TempEnv::new(self.all_envs.clone()));
    }
    pub fn clear_temp_envs(&mut self) -> Result<(), Error> {
        if let Some(idx) = &mut self.current_env_idx {
            let mut found: bool = false;
            let name = &self.all_envs[*idx].name;
            if let Some(temp) = &self.temp_envs {
                for item in temp.temp_envs.iter().enumerate() {
                    if item.1.name == *name {
                        self.current_env_idx = Some(item.0);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                self.current_env_idx = None;
            }
        }
        if let Some(te) = &self.temp_envs {
            if te.changed {
                self.all_envs = te.temp_envs.clone();
            }
        }
        self.temp_envs = None;

        let mut env_file = fs::File::create(format!("{}/{}", DATA_DIRECTORY, ENV_PATH))
            .map_err(|e| Error::FileOperationsErr(e))?;

        if self.all_envs.len() > 0 {
            env_file
                .write_all(
                    serde_json::to_string(&self.all_envs.clone())
                        .map_err(|e| Error::JsonErr(e))?
                        .as_bytes(),
                )
                .map_err(|e| Error::FileOperationsErr(e))?;
        }
        Ok(())
    }
    pub fn new_environment(&mut self, name: String) {
        if let Some(te) = &mut self.temp_envs {
            te.temp_envs.push(environments::Environment::new(name));
            te.with_name_insertion = false;
            te.changed = true;
        }
    }
    pub fn change_active_env_panel(&mut self) {
        if let Some(temp) = &mut self.temp_envs {
            temp.change_environment_subsection()
        }
    }
    pub fn has_new_env_name(&self) -> bool {
        match &self.temp_envs {
            Some(t) => t.with_name_insertion,
            None => false,
        }
    }
    pub fn has_new_env_kv(&self) -> bool {
        match &self.temp_envs {
            Some(t) => t.with_kv_insertion,
            None => false,
        }
    }
    pub fn change_active_env_kv(&mut self) {
        match &mut self.temp_envs {
            Some(t) => t.kv_insertion.is_key_active = !t.kv_insertion.is_key_active,
            None => (),
        }
    }
    pub fn add_to_env_kv(&mut self, name: String, key: String, value: String) {
        if name == "".to_string() || key == "".to_string() || value == "".to_string() {
            return;
        }
        match &mut self.temp_envs {
            Some(t) => {
                for item in t.temp_envs.iter_mut() {
                    if item.name == name {
                        item.envs.insert(key.clone(), value.clone());
                        item.envs_to_show.push([key, value]);
                        break;
                    }
                }
                t.kv_insertion.key = "".to_string();
                t.kv_insertion.value = "".to_string();
                t.with_kv_insertion = false;
                t.kv_insertion.is_key_active = true;
                t.changed = true;
            }
            None => (),
        }
    }
    fn replace_envs<T>(&self, to_replace: T) -> T
    where
        T: Clone + EnvReplacer,
    {
        match self.current_env_idx {
            Some(idx) => to_replace.replace_env(&self.regex_replacer, &self.all_envs[idx].envs),
            None => to_replace,
        }
    }
    pub fn next_env(&mut self) {
        if self.all_envs.len() > 0 {
            match self.current_env_idx {
                None => self.current_env_idx = Some(0),
                Some(mut x) => {
                    x = x + 1;
                    self.current_env_idx = Some(x);
                    if x == self.all_envs.len() {
                        self.current_env_idx = Some(0);
                    }
                }
            }
        }
    }
    pub fn pre_env(&mut self) {
        if self.all_envs.len() > 0 {
            match self.current_env_idx {
                None => self.current_env_idx = Some(0),
                Some(mut x) => {
                    if x > 0 {
                        x = x - 1;
                    }
                    self.current_env_idx = Some(x);
                    if x == self.all_envs.len() {
                        self.current_env_idx = Some(0);
                    }
                }
            }
        }
    }
    pub fn deselect_env(&mut self) {
        self.current_env_idx = None;
    }
    //   pub fn add_to_req_body(&mut self, c: char) {
    //       if let Some(req) = self.current_request_as_mut() {
    //           req.add_to_req_body(c);
    //       }
    //   }
    //   pub fn remove_from_req_body(&mut self) {
    //       if let Some(req) = self.current_request_as_mut() {
    //           req.remove_from_req_body();
    //       }
    //   }
    //   pub fn req_body(&self) -> Body {
    //       if let Some(req) = self.current_request() {
    //           return req.body.clone();
    //       };
    //       Body::default()
    //   }
    //   pub fn change_body_kind(&mut self) {
    //       if let Some(req) = &mut self.current_request_as_mut() {
    //           req.body.kind = req.body.kind.change();
    //       }
    //   }
    //   pub fn save_current_req(&mut self) -> Result<(), Error> {
    //       let mut name = String::new();
    //       if let Some(req) = self.current_request() {
    //           name = req.name.clone();
    //           if let Some(cols) = &self.collections {
    //               let path: String = cols.get_node().ok_or::<Error>(Error::NoRequestErr(3))?.path;
    //               match fs::metadata(path.clone()) {
    //                   Ok(f) => {
    //                       if f.is_dir() {
    //                           match fs::File::create(format!("{}/{}.rph", path, name)) {
    //                               Ok(mut f) => {
    //                                   let to_write = serde_json::to_vec(req).unwrap();
    //                                   f.write(&to_write).unwrap();
    //                                   self.reload_collections();
    //                               }
    //                               Err(e) => return Err(Error::FileOperationsErr(e)),
    //                           };
    //                       }
    //                   }
    //                   Err(e) => return Err(Error::FileOperationsErr(e)),
    //               };
    //           }
    //       } else {
    //           return Err(Error::NoRequestErr(1));
    //       };
    //       Ok(())
    //   }
    //   pub fn open_collections(&mut self) {
    //       self.selected_main_window = MainWindows::CollectionScr;
    //       self.set_collections();
    //   }
    pub fn reload_collections(&mut self) {
        let mut current_state = TreeState::default();
        if let Some(cols) = &self.collections {
            current_state = cols.get_state().clone()
        }
        let cols = self.create_tree(
            Node::new(
                format!("{}/{}", DATA_DIRECTORY, COLLECTION_PATH),
                COLLECTION_PATH.to_string(),
            ),
            0,
        );
        self.collections = Some(StatefulTree::with_items_and_state(
            vec![cols.unwrap()],
            current_state,
        ));
    }
    pub fn new_collection(&mut self) {
        self.has_new_collection = true
    }
    pub fn set_collections(&mut self) {
        let cols = self.create_tree(
            Node::new(
                format!("{}/{}", DATA_DIRECTORY, COLLECTION_PATH),
                COLLECTION_PATH.to_string(),
            ),
            0,
        );
        self.collections = Some(StatefulTree::with_items(vec![cols.unwrap()]));
    }
    //    pub fn open_request_from_collection(&mut self) -> Result<(), Error> {
    //        if let Some(cols) = &self.collections {
    //            let path = cols.get_node().ok_or::<Error>(Error::NoRequestErr(3))?.path;
    //            if path == "".to_string() {
    //                return Err(Error::NoRequestErr(4));
    //            }
    //            match fs::metadata(path.clone()) {
    //                Ok(f) => {
    //                    if f.is_file() {
    //                        self.add_to_requests_by_path(path.clone())?
    //                    };
    //                    if f.is_dir() {
    //                        for entry in fs::read_dir(path.clone()).unwrap() {
    //                            let entry = entry.unwrap();
    //                            match entry.path().extension() {
    //                                Some(ext) => {
    //                                    if ext == "rph" {
    //                                        self.add_to_requests_by_path(
    //                                            entry.path().to_string_lossy().to_string(),
    //                                        )?
    //                                    }
    //                                }
    //                                None => continue,
    //                            }
    //                        }
    //                    }
    //                }
    //                Err(e) => return Err(Error::FileOperationsErr(e)),
    //            };
    //        };
    //        Ok(())
    //    }
    //    fn add_to_requests_by_path(&mut self, path: String) -> Result<(), Error> {
    //        match fs::File::open(path) {
    //            Ok(f) => {
    //                let mut reader = BufReader::new(f);
    //                let mut buffer = Vec::new();
    //                reader.read_to_end(&mut buffer).unwrap();
    //                let new_req = serde_json::from_str(from_utf8(&buffer).unwrap()).unwrap();
    //                if let Some(req) = &mut self.requests {
    //                    req.push(new_req);
    //                };
    //                Ok(())
    //            }
    //            Err(e) => return Err(Error::FileOperationsErr(e)),
    //        }
    //    }
    //    pub fn close_collections(&mut self) {
    //        self.selected_main_window = MainWindows::RequestScr;
    //        self.collections = None;
    //    }
    //    pub fn has_new_collection(&self) -> bool {
    //        self.has_new_collection
    //    }
    //    pub fn has_new_req_name(&self) -> bool {
    //        self.has_new_req_name
    //    }
    //    pub fn insert_collection_or_name(&mut self) {
    //        if self.has_new_req_name {
    //            let req_name = self.collection_or_name.clone();
    //            if let Some(req) = &mut self.current_request_as_mut() {
    //                req.name = req_name;
    //                self.has_new_req_name = false;
    //                self.collection_or_name = "".to_string();
    //            }
    //        }
    //        if self.has_new_collection {
    //            if let Some(cols) = &self.collections {
    //                let path = cols.get_node().unwrap().path;
    //                if path == "".to_string() {
    //                    return;
    //                }
    //                fs::create_dir(format!("{}/{}", path, self.collection_or_name.clone())).unwrap();
    //                self.has_new_collection = false;
    //                self.collection_or_name = "".to_string();
    //                self.reload_collections();
    //            }
    //        }
    //    }
    pub fn add_to_collection_or_name_string(&mut self, x: char) {
        if self.has_new_collection || self.has_new_req_name {
            self.collection_or_name.push(x);
        }
    }
    pub fn remove_from_collection_or_name_string(&mut self) {
        if self.has_new_collection || self.has_new_req_name {
            self.collection_or_name.pop();
        }
    }
    pub fn close_new_req_or_collection(&mut self) {
        self.has_new_req_name = false;
        self.has_new_collection = false;
        self.collection_or_name = "".to_string();
    }
    //    pub fn current_req_has_name(&self) -> bool {
    //        if let Some(req) = self.current_request() {
    //            if req.name == "".to_string() {
    //                return false;
    //            }
    //            return true;
    //        }
    //        false
    //    }
    fn create_tree(&mut self, node: Node, mut depth: usize) -> Option<TreeItem> {
        let mut result = TreeItem::new_leaf(node.clone());
        if depth > 10 || !fs::metadata(node.path.clone()).unwrap().is_dir() {
            if !node.to_show.ends_with(".rph") {
                return None;
            };
            return Some(result);
        }
        for entry in fs::read_dir(node.path.clone()).unwrap() {
            let ent = entry.unwrap();
            let f_name = ent.file_name().to_str().unwrap().to_string().clone();
            let f_path = ent.path().to_string_lossy().to_string();
            let new_path = Node::new(f_path, f_name);
            depth += 1;
            if let Some(r) = self.create_tree(new_path, depth) {
                result.add_child(r);
            }
        }
        Some(result)
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

fn navigation(e: &AppEvent, s: &mut State) {
    let mut last_state = s.last();
    match e {
        NAV_UP => {
            match last_state.sub() {
                HEADERS | PARAMS | BODY => {
                    s.push(StateItem::new(s.last().main_windows().clone(), ADDRESS));
                }
                REQUESTS => s.push(StateItem::new(s.last().main_windows().clone(), VERB)),
                _ => (),
                //Windows::Address => self.selected_window = Windows::ReqNames,
                //Windows::Response => self.selected_window = Windows::RequestData,
                //Windows::Verb => self.selected_window = Windows::ReqNames,
                //Windows::RequestData => self.selected_window = Windows::Address,
                //Windows::EnvSelection => self.selected_window = Windows::ReqNames,
                //Windows::ReqNames => self.selected_window = Windows::Response,
            }
        }
        NAV_DOWN => {
            match last_state.sub() {
                REQUESTS => {
                    s.push(StateItem::new(s.last().main_windows().clone(), VERB));
                }
                VERB | ADDRESS => s.push(StateItem::new(s.last().main_windows().clone(), BODY)),
                _ => (),
                //Windows::Address => self.selected_window = Windows::RequestData,
                //Windows::Response => self.selected_window = Windows::ReqNames,
                //Windows::Verb => self.selected_window = Windows::RequestData,
                //Windows::RequestData => self.selected_window = Windows::Response,
                //Windows::EnvSelection => self.selected_window = Windows::RequestData,
                //Windows::ReqNames => self.selected_window = Windows::Address,
            }
        }
        NAV_LEFT => {
            match last_state.sub() {
                ADDRESS => s.push(StateItem::new(s.last().main_windows().clone(), VERB)),
                _ => (),
                //Windows::Address => (),
                //Windows::Response => (),
                //Windows::Verb => self.selected_window = Windows::EnvSelection,
                //Windows::RequestData => (),
                //Windows::EnvSelection => self.selected_window = Windows::Address,
                //Windows::ReqNames => (),
            }
        }
        NAV_RIGHT => {
            match last_state.sub() {
                VERB => s.push(StateItem::new(s.last().main_windows().clone(), ADDRESS)),
                _ => (),
                //Windows::Address => self.selected_window = Windows::EnvSelection,
                //Windows::Response => (),
                //Windows::Verb => (),
                //Windows::RequestData => (),
                //Windows::EnvSelection => self.selected_window = Windows::Verb,
                //Windows::ReqNames => (),
            }
        }
        _ => (),
    };
}
