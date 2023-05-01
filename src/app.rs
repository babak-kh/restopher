use crate::environments;
use crate::tree_states::Node;
use crate::tree_states::StatefulTree;
use crate::{
    environments::{Environment, KV},
    request::Body,
};
use regex::Regex;
use reqwest::header::HeaderMap;
use serde_json::{self};
use std::ffi::{OsStr, OsString};
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
    hash::Hash,
    io::{self, BufRead, BufReader, Read, Write},
    str::{from_utf8, from_utf8_unchecked},
    string::FromUtf8Error,
};
use tui_tree_widget::TreeItem;
use walkdir::WalkDir;

use crate::{
    request::{self, HttpVerb, Request},
    response::{self, Response},
};

const ENV_PATH: &str = "envs";
const COLLECTION_PATH: &str = "collections";
const DATA_DIRECTORY: &str = "/home/babak/.config/restopher";
const START_ENV_TOKEN: &str = "{{";
const END_ENV_TOKEN: &str = "}}";

#[derive(Debug)]
pub enum MainWindows {
    RequestScr,
    EnvironmentScr,
    CollectionScr,
}

#[derive(Debug)]
pub enum Windows {
    ReqNames,
    Address,
    Response,
    RequestData,
    Verb,
    EnvSelection,
}
#[derive(Debug)]
pub enum ResponseTabs<'a> {
    Body(usize, &'a str),
    Headers(usize, &'a str),
}
impl<'a> ResponseTabs<'a> {
    pub fn split_at(&self) -> (&str, &str) {
        match self {
            ResponseTabs::Headers(_, name) | ResponseTabs::Body(_, name) => name.split_at(1),
        }
    }
}

#[derive(Debug)]
pub enum RequestTabs<'a> {
    Headers(usize, &'a str),
    Params(usize, &'a str),
    Body(usize, &'a str),
}
impl<'a> RequestTabs<'a> {
    pub fn split_at(&self) -> (&str, &str) {
        match self {
            RequestTabs::Headers(_, name)
            | RequestTabs::Params(_, name)
            | RequestTabs::Body(_, name) => name.split_at(1),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NoRequestErr(usize),
    ReqwestErr(reqwest::Error),
    JsonErr(serde_json::Error),
    HeaderIsNotString,
    FileOperationsErr(io::Error),
    ParamIsNotString,
}
impl Error {
    fn from_reqwest(e: reqwest::Error) -> Self {
        Error::ReqwestErr(e)
    }
    fn from_serde(e: serde_json::Error) -> Self {
        Error::JsonErr(e)
    }
}
#[derive(Debug)]
pub struct ReqTabs<'a> {
    pub req_tabs: Vec<&'a RequestTabs<'a>>,
    pub selected: &'a RequestTabs<'a>,
    pub selected_idx: usize,
}
#[derive(Debug)]
pub struct RespTabs<'a> {
    pub resp_tabs: Vec<&'a ResponseTabs<'a>>,
    pub selected: &'a ResponseTabs<'a>,
    pub selected_idx: usize,
}

pub struct App<'a> {
    pub selected_main_window: MainWindows,
    pub selected_window: Windows,
    client: reqwest::Client,
    pub current_request_idx: usize,
    requests: Option<Vec<Request>>,
    pub temp_header_param_idx: usize,
    pub req_tabs: ReqTabs<'a>,
    pub resp_tabs: RespTabs<'a>,
    pub error_pop_up: (bool, Option<Error>),
    pub show_environments: bool,
    pub all_envs: Vec<Environment>,
    pub temp_envs: Option<environments::TempEnv>,
    pub current_env_idx: Option<usize>, // index of active environments
    pub data_directory: String,
    pub collections: Option<StatefulTree<'a>>,
    regex_replacer: regex::Regex,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let req_tabs = vec![
            &RequestTabs::Headers(0, "Headers"),
            &RequestTabs::Body(1, "Body"),
            &RequestTabs::Params(2, "Params"),
        ];
        let resp_tabs = vec![
            &ResponseTabs::Headers(0, "Headers"),
            &ResponseTabs::Body(1, "Body"),
        ];
        let all_envs = match fs::File::open(format!("{}/{}", DATA_DIRECTORY, ENV_PATH)) {
            Ok(f) => {
                let mut reader = BufReader::new(f);
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).unwrap();
                serde_json::from_str(from_utf8(&buffer).unwrap()).unwrap()
            }
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => (),
                    _ => (),
                };
                Vec::new()
            }
        };
        App {
            requests: Some(vec![Request::new()]),
            client: reqwest::Client::new(),
            current_request_idx: 0,
            selected_window: Windows::Address,
            req_tabs: ReqTabs {
                selected: req_tabs[0],
                req_tabs,
                selected_idx: 0,
            },
            resp_tabs: RespTabs {
                selected: resp_tabs[0],
                resp_tabs,
                selected_idx: 0,
            },
            error_pop_up: (false, None),
            temp_header_param_idx: 0,
            current_env_idx: None,
            show_environments: false,
            all_envs,
            selected_main_window: MainWindows::RequestScr,
            temp_envs: None,
            data_directory: DATA_DIRECTORY.to_string(),
            regex_replacer: Regex::new(&format!(
                "{}.*{}",
                regex::escape(START_ENV_TOKEN),
                regex::escape(END_ENV_TOKEN)
            ))
            .unwrap(),
            collections: None,
        }
    }
    pub fn get_req_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(req) = &self.requests {
            for r in req {
                result.push({
                    if r.name == "".to_string() {
                        let mut n = r.address.uri.clone();
                        if n.len() >= 10 {
                            n = n[0..9].to_string();
                        };
                        n
                    } else {
                        r.name.clone()
                    }
                });
            }
        };
        result
    }
    pub fn has_new_header(&self) -> bool {
        if let Some(x) = self.current_request() {
            match x.new_header {
                Some(_) => return true,
                None => return false,
            }
        }
        return false;
    }
    pub fn has_new_param(&self) -> bool {
        if let Some(x) = self.current_request() {
            match x.new_param {
                Some(_) => return true,
                None => return false,
            }
        }
        return false;
    }
    pub fn new_headers(&self) -> [String; 2] {
        if let Some(req) = self.current_request() {
            if let Some(h) = &req.new_header {
                return [h.key.text.clone(), h.value.text.clone()];
            } else {
                return ["".to_string(), "".to_string()];
            };
        };
        ["".to_string(), "".to_string()]
    }
    pub fn initiate_new_header(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            req.new_header = Some(request::KV::new());
        }
    }
    pub fn remove_new_header(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            req.new_header = None;
        }
    }
    pub fn new_param(&self) -> [String; 2] {
        if let Some(req) = self.current_request() {
            if let Some(h) = &req.new_param {
                return [h.key.text.clone(), h.value.text.clone()];
            } else {
                return ["".to_string(), "".to_string()];
            };
        };
        ["".to_string(), "".to_string()]
    }
    pub fn initiate_new_param(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            req.new_param = Some(request::KV::new());
        }
    }
    pub fn remove_new_param(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            req.new_param = None;
        }
    }
    pub fn up(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::ReqNames,
            Windows::Response => self.selected_window = Windows::RequestData,
            Windows::Verb => self.selected_window = Windows::ReqNames,
            Windows::RequestData => self.selected_window = Windows::Address,
            Windows::EnvSelection => self.selected_window = Windows::ReqNames,
            Windows::ReqNames => self.selected_window = Windows::Response,
        };
    }
    pub fn down(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::RequestData,
            Windows::Response => self.selected_window = Windows::ReqNames,
            Windows::Verb => self.selected_window = Windows::RequestData,
            Windows::RequestData => self.selected_window = Windows::Response,
            Windows::EnvSelection => self.selected_window = Windows::RequestData,
            Windows::ReqNames => self.selected_window = Windows::Address,
        };
    }
    pub fn right(&mut self) {
        match self.selected_window {
            Windows::Address => (),
            Windows::Response => (),
            Windows::Verb => self.selected_window = Windows::EnvSelection,
            Windows::RequestData => (),
            Windows::EnvSelection => self.selected_window = Windows::Address,
            Windows::ReqNames => (),
        };
    }
    pub fn left(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::EnvSelection,
            Windows::Response => (),
            Windows::Verb => (),
            Windows::RequestData => (),
            Windows::EnvSelection => self.selected_window = Windows::Verb,
            Windows::ReqNames => (),
        };
    }
    fn current_request_as_mut(&mut self) -> Option<&mut Request> {
        if let Some(req) = &mut self.requests {
            return Some(&mut req[self.current_request_idx]);
        };
        None
    }
    fn current_request(&self) -> Option<&Request> {
        if let Some(req) = &self.requests {
            return Some(&req[self.current_request_idx]);
        };
        None
    }
    pub fn address(&self) -> Option<String> {
        if let Some(req) = self.current_request() {
            return Some(req.address.to_string());
        };
        None
    }
    pub fn add_header(&mut self, k: String, v: String) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(ref mut headers) = req.headers {
                headers.push((k, v, true));
                return;
            }
            let mut h: Vec<(String, String, bool)> = Vec::new();
            h.push((k, v, true));
            req.headers = Some(h);
        }
    }
    pub fn add_header_key(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(ref mut headers) = req.new_header {
                if headers.key.text == "".to_string() || headers.value.text == "".to_string() {
                    return;
                }
                if let Some(ref mut h) = req.headers {
                    h.push((headers.key.text.clone(), headers.value.text.clone(), true));
                    return;
                }
                let mut h: Vec<(String, String, bool)> = Vec::new();
                h.push((headers.key.text.clone(), headers.value.text.clone(), true));
                req.headers = Some(h);
            }
        }
    }
    pub fn add_header_value(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(ref mut headers) = req.new_header {
                if headers.key.text == "".to_string() || headers.value.text == "".to_string() {
                    return;
                }
                if let Some(ref mut h) = req.headers {
                    h.push((headers.key.text.clone(), headers.value.text.clone(), true));
                    return;
                }
                let mut h: Vec<(String, String, bool)> = Vec::new();
                h.push((headers.key.text.clone(), headers.value.text.clone(), true));
                req.headers = Some(h);
            }
        }
    }
    pub fn add_param_key(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(ref mut params) = req.new_param {
                if params.key.text == "".to_string() || params.value.text == "".to_string() {
                    return;
                }
                if let Some(ref mut h) = req.params {
                    h.push((params.key.text.clone(), params.value.text.clone(), true));
                    return;
                }
                let mut h: Vec<(String, String, bool)> = Vec::new();
                h.push((params.key.text.clone(), params.value.text.clone(), true));
                req.params = Some(h);
            }
        }
    }
    pub fn add_param_value(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(ref mut params) = req.new_param {
                if params.key.text == "".to_string() || params.value.text == "".to_string() {
                    return;
                }
                if let Some(ref mut h) = req.params {
                    h.push((params.key.text.clone(), params.value.text.clone(), true));
                    return;
                }
                let mut h: Vec<(String, String, bool)> = Vec::new();
                h.push((params.key.text.clone(), params.value.text.clone(), true));
                req.params = Some(h);
            }
        }
    }
    pub fn pop_address(&mut self) {
        if let Some(ref mut r) = self.current_request_as_mut() {
            r.address.pop();
        }
    }
    pub fn add_address(&mut self, c: char) {
        if let Some(r) = self.current_request_as_mut() {
            r.address.add(c);
        }
    }
    pub fn verb_up(&mut self) {
        if let Some(r) = self.current_request_as_mut() {
            r.verb = r.verb.up();
        }
    }
    pub fn verb_down(&mut self) {
        if let Some(r) = self.current_request_as_mut() {
            r.verb = r.verb.down();
        }
    }
    pub fn verb(&self) -> String {
        if let Some(r) = self.current_request() {
            return r.verb.to_string();
        }
        HttpVerb::GET.to_string()
    }
    pub fn response_body(&self) -> String {
        if let Some(r) = self.current_request() {
            if let Some(ref res) = r.response {
                return res.body.clone().unwrap_or("".to_string());
            };
        }
        "".to_string()
    }
    pub fn response_status_code(&self) -> i32 {
        if let Some(r) = self.current_request() {
            if let Some(ref res) = r.response {
                return res.status_code.clone();
            };
        }
        0
    }
    pub fn headers(&self) -> Option<Vec<(String, String, bool)>> {
        if let Some(req) = self.current_request() {
            return req.headers.clone().or(None);
        }
        None
    }
    pub fn params(&self) -> Option<Vec<(String, String, bool)>> {
        if let Some(req) = self.current_request() {
            return req.params.clone().or(None);
        }
        None
    }
    pub async fn call_request(&mut self) -> Result<String, Error> {
        let mut addr = String::new();
        let mut params = HashMap::new();
        let mut headers = HeaderMap::new();
        let mut body = None;

        if let Some(request) = &self.requests {
            let req = &request[self.current_request_idx];
            params = self.replace_envs(req.handle_params());
            addr = self.replace_envs(req.address.to_string());
            headers = (&self.replace_envs(req.handle_headers()))
                .try_into()
                .unwrap();
            body = req.handle_json_body()?;
        }
        if let Some(requests) = &mut self.requests {
            let req = &mut requests[self.current_request_idx];
            match req.verb {
                HttpVerb::GET => {
                    let r = self
                        .client
                        .get(addr)
                        .query(&params)
                        .headers(headers)
                        .send()
                        .await
                        .map_err(|e| Error::ReqwestErr(e))?;
                    req.response = Some(Response {
                        headers: Some(response::handle_response_headers(r.headers())?),
                        status_code: r.status().as_u16() as i32,
                        body: Some(r.text().await.map_err(|e| Error::ReqwestErr(e))?),
                    });
                    return Ok("done".to_string());
                }
                HttpVerb::POST => {
                    let mut r = self.client.post(addr).query(&params).headers(headers);
                    if let Some(b) = body {
                        r = r.json(&b)
                    };
                    let resp = r.send().await.map_err(|e| Error::ReqwestErr(e))?;
                    req.response = Some(Response {
                        headers: Some(response::handle_response_headers(resp.headers())?),
                        status_code: resp.status().as_u16() as i32,
                        body: Some(resp.text().await.map_err(|e| Error::from_reqwest(e))?),
                    });
                    return Ok("done".to_string());
                }
                HttpVerb::PUT => {
                    let mut r = self.client.put(addr).query(&params).headers(headers);
                    if let Some(b) = body {
                        r = r.json(&b)
                    };
                    let resp = r.send().await.map_err(|e| Error::ReqwestErr(e))?;
                    req.response = Some(Response {
                        headers: Some(response::handle_response_headers(resp.headers())?),
                        status_code: resp.status().as_u16() as i32,
                        body: Some(resp.text().await.unwrap()),
                    });
                    return Ok("done".to_string());
                }
                HttpVerb::DELETE => {
                    let r = self
                        .client
                        .get(addr)
                        .query(&params)
                        .headers(headers)
                        .send()
                        .await
                        .map_err(|e| Error::ReqwestErr(e))?;
                    req.response = Some(Response {
                        headers: Some(response::handle_response_headers(r.headers())?),
                        status_code: r.status().as_u16() as i32,
                        body: Some(r.text().await.unwrap()),
                    });
                    return Ok("done".to_string());
                }
            }
        }
        Err(Error::NoRequestErr(0))
    }
    pub fn change_request_tab(&mut self) {
        let mut idx: usize;
        match self.req_tabs.selected {
            RequestTabs::Headers(index, _)
            | RequestTabs::Params(index, _)
            | RequestTabs::Body(index, _) => idx = *index,
        }
        idx += 1;
        if idx == self.req_tabs.req_tabs.len() {
            idx = 0;
            self.req_tabs.selected = self.req_tabs.req_tabs[0]
        }
        self.req_tabs.selected_idx = idx;
        self.req_tabs.selected = self.req_tabs.req_tabs[idx]
    }
    pub fn change_response_tab(&mut self) {
        let mut idx: usize;
        match self.resp_tabs.selected {
            ResponseTabs::Headers(index, _) | ResponseTabs::Body(index, _) => idx = *index,
        }
        idx += 1;
        if idx == self.resp_tabs.resp_tabs.len() {
            idx = 0;
            self.resp_tabs.selected = self.resp_tabs.resp_tabs[0]
        }
        self.resp_tabs.selected_idx = idx;
        self.resp_tabs.selected = self.resp_tabs.resp_tabs[idx]
    }
    pub fn add_to_kv(&mut self, ch: char) {
        match self.req_tabs.selected {
            RequestTabs::Headers(_, _) => {
                if let Some(req) = self.current_request_as_mut() {
                    if let Some(h) = &mut req.new_header {
                        h.add_to_active(ch);
                    }
                }
            }
            RequestTabs::Params(_, _) => {
                if let Some(req) = self.current_request_as_mut() {
                    if let Some(h) = &mut req.new_param {
                        h.add_to_active(ch);
                    }
                }
            }
            _ => (),
        }
    }
    pub fn remove_from_kv(&mut self) {
        match self.req_tabs.selected {
            RequestTabs::Headers(_, _) => {
                if let Some(req) = self.current_request_as_mut() {
                    if let Some(h) = &mut req.new_header {
                        h.remove_from_active();
                    }
                }
            }
            RequestTabs::Params(_, _) => {
                if let Some(req) = self.current_request_as_mut() {
                    if let Some(h) = &mut req.new_param {
                        h.remove_from_active();
                    }
                }
            }
            _ => (),
        }
    }
    pub fn change_active(&mut self) {
        match self.req_tabs.selected {
            RequestTabs::Headers(_, _) => {
                if let Some(req) = self.current_request_as_mut() {
                    if let Some(h) = &mut req.new_header {
                        h.change_active();
                    }
                }
            }
            RequestTabs::Params(_, _) => {
                if let Some(req) = self.current_request_as_mut() {
                    if let Some(h) = &mut req.new_param {
                        h.change_active();
                    }
                }
            }
            _ => (),
        }
    }
    pub fn is_key_active(&self) -> bool {
        match self.req_tabs.selected {
            RequestTabs::Headers(_, _) => {
                if let Some(req) = self.current_request() {
                    if let Some(h) = &req.new_header {
                        return h.is_key_active();
                    } else {
                        return false;
                    };
                };
            }
            RequestTabs::Params(_, _) => {
                if let Some(req) = self.current_request() {
                    if let Some(h) = &req.new_param {
                        return h.is_key_active();
                    } else {
                        return false;
                    };
                }
            }
            _ => (),
        }
        false
    }
    pub fn response_headers(&self) -> Option<HashMap<String, String>> {
        if let Some(req) = self.current_request() {
            if let Some(resp) = &req.response {
                return resp.headers();
            };
            return None;
        };
        None
    }
    pub fn delete_selected_header(&mut self) {
        let idx = self.temp_header_param_idx.clone();
        if let Some(req) = self.current_request_as_mut() {
            if let Some(headers) = &mut req.headers {
                if idx <= headers.len() - 1 {
                    headers.remove(idx);
                }
            }
        }
    }
    pub fn delete_selected_param(&mut self) {
        let idx = self.temp_header_param_idx.clone();
        if let Some(req) = self.current_request_as_mut() {
            if let Some(params) = &mut req.params {
                if idx <= params.len() - 1 {
                    params.remove(idx);
                }
            }
        }
    }
    pub fn increase_temp_idx(&mut self) {
        if let Some(req) = self.current_request() {
            if self.temp_header_param_idx
                <= std::cmp::max(
                    req.headers.clone().map_or(0, |v| v.len()),
                    req.params.clone().map_or(0, |v| v.len()),
                )
            {
                self.temp_header_param_idx += 1;
            }
        }
    }
    pub fn decrease_temp_idx(&mut self) {
        if self.temp_header_param_idx >= 1 {
            self.temp_header_param_idx -= 1;
        };
    }
    pub fn change_activity_selected_param(&mut self) {
        let idx = self.temp_header_param_idx.clone();
        if let Some(req) = self.current_request_as_mut() {
            if let Some(params) = &mut req.params {
                if idx <= params.len() - 1 {
                    params[idx].2 = !params[idx].2;
                }
            }
        }
    }
    pub fn change_activity_selected_header(&mut self) {
        let idx = self.temp_header_param_idx.clone();
        if let Some(req) = self.current_request_as_mut() {
            if let Some(headers) = &mut req.headers {
                if idx <= headers.len() - 1 {
                    headers[idx].2 = !headers[idx].2;
                }
            }
        }
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
    pub fn add_to_req_body(&mut self, c: char) {
        if let Some(req) = self.current_request_as_mut() {
            req.add_to_req_body(c);
        }
    }
    pub fn remove_from_req_body(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            req.remove_from_req_body();
        }
    }
    pub fn req_body(&self) -> Body {
        if let Some(req) = self.current_request() {
            return req.body.clone();
        };
        Body::default()
    }
    pub fn change_body_kind(&mut self) {
        if let Some(req) = &mut self.current_request_as_mut() {
            req.body.kind = req.body.kind.change();
        }
    }
    pub fn next_req(&mut self) {
        self.current_request_idx += 1;
        if let Some(req) = &self.requests {
            if self.current_request_idx >= req.len() {
                self.current_request_idx = 0;
            }
        }
    }
    pub fn pre_req(&mut self) {
        if let Some(req) = &self.requests {
            if self.current_request_idx == 0 {
                self.current_request_idx = req.len() - 1;
                return;
            };
            self.current_request_idx -= 1;
        }
    }
    pub fn new_request(&mut self) {
        if let Some(req) = &mut self.requests {
            req.push(request::Request::new());
            self.current_request_idx = req.len() - 1;
        };
    }
    pub fn save_current_req(&mut self) {
        self.selected_main_window = MainWindows::CollectionScr;
        self.set_collections();
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
    pub fn close_collections(&mut self) {
        self.selected_main_window = MainWindows::RequestScr;
        self.collections = None;
    }
    fn create_tree(&mut self, node: Node, mut depth: usize) -> Option<TreeItem<'static>> {
        let mut result = TreeItem::new_leaf(node.clone());
        if depth > 10 || !fs::metadata(node.path.clone()).unwrap().is_dir() {
            if !node.name.ends_with(".rph") {
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
            //            if idx == 0 {
            //                continue;
            //            }
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
