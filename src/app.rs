use serde_json::{self, Serializer};
use std::collections::HashMap;

use crate::{
    request::{self, HttpVerb, Request},
    response::{self, Response},
};
#[derive(Debug)]
pub enum Windows {
    Address,
    Response,
    RequestData,
    Verb,
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
    pub selected_window: Windows,
    client: reqwest::Client,
    current_request_idx: usize,
    requests: Option<Vec<Request>>,
    pub temp_header_param_idx: usize,
    pub req_tabs: ReqTabs<'a>,
    pub resp_tabs: RespTabs<'a>,
    pub error_pop_up: (bool, Option<Error>),
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
        }
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
            Windows::Address => self.selected_window = Windows::Response,
            Windows::Response => self.selected_window = Windows::RequestData,
            Windows::Verb => (),
            Windows::RequestData => self.selected_window = Windows::Address,
        };
    }
    pub fn down(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::RequestData,
            Windows::Response => self.selected_window = Windows::Address,
            Windows::Verb => self.selected_window = Windows::RequestData,
            Windows::RequestData => self.selected_window = Windows::Response,
        };
    }
    pub fn right(&mut self) {
        match self.selected_window {
            Windows::Address => (),
            Windows::Response => (),
            Windows::Verb => self.selected_window = Windows::Address,
            Windows::RequestData => (),
        };
    }
    pub fn left(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Verb,
            Windows::Response => (),
            Windows::Verb => (),
            Windows::RequestData => (),
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
        if let Some(requests) = &mut self.requests {
            let req = &mut requests[self.current_request_idx];
            match req.verb {
                HttpVerb::GET => {
                    let r = self
                        .client
                        .get(&req.address.to_string())
                        .query(&req.handle_params())
                        .headers(req.handle_headers())
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
                    let r = self
                        .client
                        .post(&req.address.to_string())
                        .query(&req.handle_params())
                        .headers(req.handle_headers())
                        .json(&req.handle_json_body().map(|_| "".to_string())?)
                        .send()
                        .await
                        .map_err(|e| Error::ReqwestErr(e))?;
                    req.response = Some(Response {
                        headers: Some(response::handle_response_headers(r.headers())?),
                        status_code: r.status().as_u16() as i32,
                        body: Some(r.text().await.map_err(|e| Error::from_reqwest(e))?),
                    });
                    return Ok("done".to_string());
                }
                HttpVerb::PUT => {
                    let r = self
                        .client
                        .put(&req.address.to_string())
                        .query(&req.handle_params())
                        .headers(req.handle_headers())
                        .json(&req.handle_json_body().map(|_| "".to_string())?)
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
                HttpVerb::DELETE => {
                    let r = self
                        .client
                        .get(&req.address.to_string())
                        .query(&req.handle_params())
                        .headers(req.handle_headers())
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
}
