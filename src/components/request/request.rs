use std::collections::HashMap;
use std::default;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::utils::text_box::TextBox;

use super::response::Response;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HttpVerb {
    GET,
    POST,
    PUT,
    DELETE,
}
impl HttpVerb {
    pub fn to_string(&self) -> String {
        match self {
            HttpVerb::GET => "GET".to_string(),
            HttpVerb::POST => "POST".to_string(),
            HttpVerb::DELETE => "DELETE".to_string(),
            HttpVerb::PUT => "PUT".to_string(),
        }
    }
    pub fn down(&self) -> Self {
        match self {
            HttpVerb::GET => HttpVerb::POST,
            HttpVerb::POST => HttpVerb::PUT,
            HttpVerb::PUT => HttpVerb::DELETE,
            HttpVerb::DELETE => HttpVerb::GET,
        }
    }
    pub fn up(&self) -> Self {
        match self {
            HttpVerb::GET => HttpVerb::DELETE,
            HttpVerb::POST => HttpVerb::GET,
            HttpVerb::PUT => HttpVerb::POST,
            HttpVerb::DELETE => HttpVerb::PUT,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyKind {
    JSON,
    TEXT,
}
impl BodyKind {
    pub fn to_string(&self) -> String {
        match self {
            BodyKind::JSON => "JSON".to_string(),
            BodyKind::TEXT => "Text".to_string(),
        }
    }
    pub fn change(&self) -> Self {
        match self {
            BodyKind::JSON => BodyKind::TEXT,
            BodyKind::TEXT => BodyKind::JSON,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub kind: BodyKind,
    pub payload: Option<String>,
}
impl Body {
    pub fn default() -> Self {
        Body {
            kind: BodyKind::JSON,
            payload: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub headers: Option<Vec<(String, String, bool)>>,
    pub params: Option<Vec<(String, String, bool)>>,
    pub body: Body,
    pub address: Address,
    pub verb: HttpVerb,
    pub response: Option<Response>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            name: "".to_string(),
            headers: None,
            params: None,
            body: Body {
                kind: BodyKind::JSON,
                payload: None,
            },
            address: Address {
                uri: TextBox::new(),
            },
            verb: HttpVerb::GET,
            response: None,
        }
    }
    pub fn handle_headers(&self) -> HashMap<String, String> {
        self.headers
            .clone()
            .unwrap_or(vec![("".to_string(), "".to_string(), false)])
            .iter()
            .filter(|item| item.2)
            .map(|item| (item.0.clone(), item.1.clone()))
            .collect::<HashMap<String, String>>()
    }
    pub fn handle_params(&self) -> HashMap<String, String> {
        let h = self
            .params
            .clone()
            .unwrap_or(vec![("".to_string(), "".to_string(), false)])
            .iter()
            .filter(|item| item.2)
            .map(|item| (item.0.clone(), item.1.clone()))
            .collect::<HashMap<String, String>>();
        h
    }
    pub fn handle_json_body(&self) -> Result<Option<serde_json::Value>, crate::app::Error> {
        match &self.body.payload {
            Some(data) => {
                serde_json::from_str(&*data.clone()).map_err(|e| crate::app::Error::JsonErr(e))
            }
            None => Ok(None),
        }
    }
    pub fn add_to_req_body(&mut self, c: char) {
        match &mut self.body.payload {
            Some(s) => s.push(c),
            None => self.body.payload = Some(c.to_string()),
        }
    }
    pub fn remove_from_req_body(&mut self) {
        match &mut self.body.payload {
            Some(s) => {
                s.pop();
                if s.len() == 0 {
                    self.body.payload = None;
                }
            }
            None => (),
        }
    }
    pub fn add_to_header(&mut self, key: String, value: String, active: bool) {
        if let Some(ref mut h) = self.headers {
            h.push((key, value, active));
        } else {
            self.headers = Some(vec![(key, value, active)])
        }
    }
    pub fn add_to_param(&mut self, key: String, value: String, active: bool) {
        if let Some(ref mut h) = self.params {
            h.push((key, value, active));
        } else {
            self.headers = Some(vec![(key, value, active)])
        }
    }
    pub fn delete_header(&mut self, idx: usize) {
        if let Some(h) = &mut self.headers {
            h.remove(idx);
        }
    }
    pub fn delete_param(&mut self, idx: usize) {
        if let Some(h) = &mut self.params {
            h.remove(idx);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub uri: TextBox,
}
impl Address {
    pub fn to_string(&self) -> String {
        self.uri.to_string()
    }
    pub fn pop(&mut self) {
        self.uri.pop();
    }
    pub fn add(&mut self, c: char) {
        self.uri.push(c);
    }
}
