use reqwest::{
    get,
    header::{self, HeaderMap},
};
use serde_json::{self, Serializer};
use std::collections::HashMap;
use std::convert::TryInto;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

use crate::{
    request::{self, HttpVerb, Request},
    response::Response,
};
#[derive(Debug)]
pub enum Windows {
    Address,
    Response,
    RequestData,
    Verb,
}
#[derive(Debug)]
enum ResponseTabs {
    Body,
    Headers,
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
}
#[derive(Debug)]
pub struct ReqTabs<'a> {
    pub req_tabs: Vec<&'a RequestTabs<'a>>,
    pub selected: &'a RequestTabs<'a>,
    pub selected_idx: usize,
}
pub struct App<'a> {
    pub selected_window: Windows,
    client: reqwest::Client,
    current_request_idx: usize,
    requests: Option<Vec<Request>>,
    pub req_tabs: ReqTabs<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let req_tabs = vec![
            &RequestTabs::Headers(0, "Headers"),
            &RequestTabs::Body(1, "Body"),
            &RequestTabs::Params(2, "Params"),
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
    pub fn new_headers(&self) -> [String; 2] {
        if let Some(req) = self.current_request() {
            if let Some(h) = &req.new_header {
                return [h.key.text.clone(), h.value.text.clone()]
            } else {
                return ["bagh".to_string(), "".to_string()]
            };
        };
        ["baghoooo".to_string(), "".to_string()]
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
            Windows::RequestData => todo!(),
        };
    }
    pub fn left(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Verb,
            Windows::Response => (),
            Windows::Verb => (),
            Windows::RequestData => todo!(),
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
                headers.insert(k, v);
                return;
            }
            let mut h = HashMap::new();
            h.insert(k, v);
            req.headers = Some(h);
        }
    }
    pub fn add_header_key(&mut self) {
        let v = "".to_string();
        if let Some(req) = self.current_request_as_mut() {
            if let Some(ref mut headers) = req.new_header {
                if headers.key.text == "".to_string() {
                    return;
                }
                if let Some(ref mut h) = req.headers {
                    h.insert(headers.key.text.clone(), v);
                    return;
                }
                let mut h = HashMap::new();
                h.insert(headers.key.text.clone(), v);
                req.headers = Some(h);
            }
        }
    }
    pub fn add_header_value(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(ref mut headers) = req.new_header {
                if headers.key.text == "".to_string() {
                    return;
                }
                if let Some(ref mut h) = req.headers {
                    h.insert(headers.key.text.clone(), headers.value.text.clone());
                    return;
                }
                let mut h = HashMap::new();
                h.insert(headers.key.text.clone(), headers.value.text.clone());
                req.headers = Some(h);
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
    pub fn headers(&self) -> Option<HashMap<String, String>> {
        if let Some(req) = self.current_request() {
            return req.headers.clone().or(None);
        }
        None
    }
    pub async fn call_request(&mut self) -> Result<String, Error> {
        if let Some(requests) = &mut self.requests {
            let req = &mut requests[self.current_request_idx];
            let headers: HeaderMap = (&req.headers.clone().unwrap())
                .try_into()
                .expect("valid headers");
            match req.verb {
                HttpVerb::GET => {
                    let r = self
                        .client
                        .get(&req.address.to_string())
                        .query(&req.params.as_ref().unwrap())
                        .headers(headers)
                        .send()
                        .await
                        .unwrap();
                    let mut response_headers = HashMap::new();
                    for (key, value) in r.headers().iter() {
                        response_headers.insert(
                            key.as_str().to_string(),
                            value.to_str().unwrap().to_string(),
                        );
                    }
                    req.response = Some(Response {
                        headers: Some(response_headers),
                        status_code: r.status().as_u16() as i32,
                        body: Some(r.text().await.unwrap()),
                    });
                }
                HttpVerb::POST => {
                    let json_body: &serde_json::Value =
                        &serde_json::from_str(&req.body.clone().unwrap()).unwrap();
                    let headers: HeaderMap = (&req.headers.clone().unwrap())
                        .try_into()
                        .expect("invalid headers");
                    let r = self
                        .client
                        .post(&req.address.to_string())
                        .query(&req.params.as_ref().unwrap())
                        .headers(headers)
                        .json(json_body)
                        .send()
                        .await
                        .unwrap();
                    let mut response_headers = HashMap::new();
                    for (key, value) in r.headers().iter() {
                        response_headers.insert(
                            key.as_str().to_string(),
                            value.to_str().unwrap().to_string(),
                        );
                    }
                    req.response = Some(Response {
                        headers: Some(response_headers),
                        status_code: r.status().as_u16() as i32,
                        body: Some(r.text().await.unwrap()),
                    });
                }
                HttpVerb::PUT => {
                    let json_body: &serde_json::Value =
                        &serde_json::from_str(&req.body.clone().unwrap()).unwrap();
                    let headers: HeaderMap = (&req.headers.clone().unwrap())
                        .try_into()
                        .expect("invalid headers");
                    let r = self
                        .client
                        .put(&req.address.to_string())
                        .query(&req.params.as_ref().unwrap())
                        .headers(headers)
                        .json(json_body)
                        .send()
                        .await
                        .unwrap();
                    let mut response_headers = HashMap::new();
                    for (key, value) in r.headers().iter() {
                        response_headers.insert(
                            key.as_str().to_string(),
                            value.to_str().unwrap().to_string(),
                        );
                    }
                    req.response = Some(Response {
                        headers: Some(response_headers),
                        status_code: r.status().as_u16() as i32,
                        body: Some(r.text().await.unwrap()),
                    });
                }
                HttpVerb::DELETE => {
                    let r = self
                        .client
                        .get(&req.address.to_string())
                        .query(&req.params.as_ref().unwrap())
                        .headers(headers)
                        .send()
                        .await
                        .unwrap();
                    let mut response_headers = HashMap::new();
                    for (key, value) in r.headers().iter() {
                        response_headers.insert(
                            key.as_str().to_string(),
                            value.to_str().unwrap().to_string(),
                        );
                    }
                    req.response = Some(Response {
                        headers: Some(response_headers),
                        status_code: r.status().as_u16() as i32,
                        body: Some(r.text().await.unwrap()),
                    });
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
    }
    pub fn add_to_kv(&mut self, ch: char) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(h) = &mut req.new_header {
                h.add_to_active(ch);
            }
        }
    }
    pub fn remove_from_kv(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(h) = &mut req.new_header {
                h.remove_from_active();
            }
        }
    }
    pub fn change_active(&mut self) {
        if let Some(req) = self.current_request_as_mut() {
            if let Some(h) = &mut req.new_header {
                h.change_active();
            }
        }
    }
    pub fn is_key_active(&self) -> bool {
        if let Some(req) = self.current_request() {
            if let Some(h) = &req.new_header {
                return h.is_key_active();
            } else {
                false
            };
        }
        false
    }
}
