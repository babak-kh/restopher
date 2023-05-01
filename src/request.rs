use std::collections::HashMap;
use std::default;

use reqwest::header::HeaderMap;

use crate::response::Response;

#[derive(Debug)]
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
#[derive(Debug)]
pub struct KVElement {
    pub text: String,
    pub active: bool,
}
#[derive(Debug)]
pub struct KV {
    pub key: KVElement,
    pub value: KVElement,
}
impl KV {
    pub fn new() -> Self {
        KV {
            key: KVElement {
                text: "".to_string(),
                active: true,
            },
            value: KVElement {
                text: "".to_string(),
                active: false,
            },
        }
    }
    pub fn change_active(&mut self) {
        self.value.active = !self.value.active;
        self.key.active = !self.key.active;
    }
    pub fn add_to_active(&mut self, ch: char) {
        if self.key.active {
            self.key.text.push(ch);
            return;
        }
        self.value.text.push(ch)
    }
    pub fn remove_from_active(&mut self) {
        if self.key.active {
            self.key.text.pop();
            return;
        }
        self.value.text.pop();
    }
    pub fn is_key_active(&self) -> bool {
        return self.key.active;
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub headers: Option<Vec<(String, String, bool)>>,
    pub params: Option<Vec<(String, String, bool)>>,
    pub body: Body,
    pub address: Address,
    pub verb: HttpVerb,
    pub response: Option<Response>,
    pub new_header: Option<KV>,
    pub new_param: Option<KV>,
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
                uri: "".to_string(),
            },
            verb: HttpVerb::GET,
            response: None,
            new_header: None,
            new_param: None,
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
            },
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
}

#[derive(Debug)]
pub struct Address {
    pub uri: String,
}
impl Address {
    pub fn to_string(&self) -> String {
        self.uri.clone()
    }
    pub fn pop(&mut self) {
        self.uri.pop();
    }
    pub fn add(&mut self, c: char) {
        self.uri.push(c);
    }
}
