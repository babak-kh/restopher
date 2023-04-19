use std::collections::HashMap;

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

#[derive(Debug)]
pub struct Request {
    pub headers: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>,
    pub body: Option<String>,
    pub address: Address,
    pub verb: HttpVerb,
    pub response: Option<Response>,
    pub new_header: Option<KV>,
    pub new_param: Option<KV>,
}
impl Request {
    pub fn new() -> Self {
        Request {
            headers: None,
            params: None,
            body: None,
            address: Address {
                uri: "".to_string(),
            },
            verb: HttpVerb::GET,
            response: None,
            new_header: None,
            new_param: None,
        }
    }
    pub fn handle_headers(&self) -> HeaderMap {
        let headers: HeaderMap = (&self.headers.clone().unwrap_or(HashMap::new()))
            .try_into()
            .expect("valid headers");
        headers
    }
    pub fn handle_json_body(&self) -> Result<serde_json::Value, crate::app::Error> {
        serde_json::from_str(&self.body.clone().unwrap_or("".to_string()))
            .map_err(|e| crate::app::Error::JsonErr(e))
    }
}

#[derive(Debug)]
pub struct Address {
    uri: String,
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
