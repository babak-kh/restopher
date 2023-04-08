use reqwest::{
    get,
    header::{self, HeaderMap},
};
use serde_json::{self, Serializer};
use std::collections::HashMap;
use std::convert::TryInto;

use crate::{
    request::{HttpVerb, Request},
    response::Response,
};
#[derive(Debug)]
pub enum Windows {
    Address,
    Response,
    Verb,
}
#[derive(Debug)]
enum ResponseTabs {
    Body,
    Headers,
}
#[derive(Debug)]
enum RequestTabs {
    Body,
    Headers,
    Params,
}

#[derive(Debug)]
pub enum Error {
    NoRequestErr(usize),
}

pub struct App {
    pub selected_window: Windows,
    client: reqwest::Client,
    current_request_idx: usize,
    requests: Option<Vec<Request>>,
}
impl App {
    pub fn new() -> Self {
        App {
            requests: Some(vec![Request::new()]),
            client: reqwest::Client::new(),
            current_request_idx: 0,
            selected_window: Windows::Address,
        }
    }
    pub fn up(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Response,
            Windows::Response => self.selected_window = Windows::Address,
            Windows::Verb => (),
        };
    }
    pub fn down(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Response,
            Windows::Response => self.selected_window = Windows::Address,
            Windows::Verb => self.selected_window = Windows::Response,
        };
    }
    pub fn right(&mut self) {
        match self.selected_window {
            Windows::Address => (),
            Windows::Response => (),
            Windows::Verb => self.selected_window = Windows::Address,
        };
    }
    pub fn left(&mut self) {
        match self.selected_window {
            Windows::Address => self.selected_window = Windows::Verb,
            Windows::Response => (),
            Windows::Verb => (),
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
                return res.body.clone().unwrap_or("".to_string())
            };
        }
        "".to_string()
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
}
