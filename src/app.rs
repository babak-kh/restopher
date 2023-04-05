use std::collections::HashMap;

use reqwest::{get, header::HeaderMap};

use crate::request::{HttpVerb, Request};
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

enum Error {
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
            requests: None,
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
    pub fn address(&self) -> Option<String> {
        if let Some(req) = self.requests {
            return Some(req[self.current_request_idx].address);
        };
        None
    }
    pub async fn call_request(&mut self) -> Result<String, Error> {
        if let Some(requests) = &self.requests {
            let req = &requests[self.current_request_idx];
            match req.verb {
                HttpVerb::GET => {
                    let r = self
                        .client
                        .get(&req.address)
                        .query(&req.params.as_ref().unwrap())
                        .headers(reqwest::header::HeaderMap::try_from(
                            &req.headers.as_ref().unwrap(),
                        ).unwrap());
                    r.send().await;
                }
                HttpVerb::POST => {
                    let r = self.client.post(&req.address);
                }
                HttpVerb::PUT => {}
                HttpVerb::DELETE => {}
            }
        }
        Err(Error::NoRequestErr(0))
    }
}
