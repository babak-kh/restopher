use std::collections::HashMap;

use crate::request::body::{Body, BodyKind};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
    pub status_code: i32,
}

impl Response {
    pub fn headers(&self) -> Option<HashMap<String, String>> {
        self.headers.clone()
    }
    pub fn new() -> Self {
        Response {
            headers: None,
            body: None,
            status_code: 0,
        }
    }
}

pub fn handle_response_headers(
    r: &HeaderMap,
) -> Result<HashMap<String, String>, crate::app::Error> {
    let mut response_headers = HashMap::new();
    for (key, value) in r.iter() {
        response_headers.insert(
            key.as_str().to_string(),
            value
                .to_str()
                .map_err(|_| crate::app::Error::HeaderIsNotString)?
                .to_string(),
        );
    }
    Ok(response_headers)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    name: String,
    headers: Option<Vec<(String, String, bool)>>,
    params: Option<Vec<(String, String, bool)>>,
    body: Body,
    address: String,
    verb: HttpVerb,
    response: Option<Response>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            name: String::new(),
            headers: None,
            params: None,
            body: Body {
                kind: BodyKind::JSON,
                payload: None,
            },
            address: "".to_string(),
            verb: HttpVerb::GET,
            response: None,
        }
    }
    pub fn name(&self) -> String {
        if !self.name.is_empty() {
            return self.name.clone();
        }
        let mut n = self.address.clone();
        if n.len() >= 10 {
            n = n[0..9].to_string();
        };
        n
    }
    pub fn set_name(&mut self, n: String) {
        self.name = n;
    }
    pub fn address(&self) -> String {
        self.address.to_string()
    }
    pub fn response(&self) -> Option<Response> {
        self.response.clone()
    }
    pub fn add_to_header(&mut self, key: String, value: String, active: bool) {
        if !key.is_empty() && !value.is_empty() {
            self.headers
                .get_or_insert_with(|| Vec::new())
                .push((key, value, active));
        }
    }
    pub fn add_to_param(&mut self, key: String, value: String, active: bool) {
        if !key.is_empty() && !value.is_empty() {
            self.params
                .get_or_insert_with(|| Vec::new())
                .push((key, value, active));
        }
    }
    pub fn add_to_address(&mut self, c: char) {
        self.address.push(c);
    }
    pub fn remove_from_address(&mut self) {
        self.address.pop();
    }
    pub fn verb_up(&mut self) {
        self.verb = self.verb.up();
    }
    pub fn verb_down(&mut self) {
        self.verb = self.verb.down();
    }
    pub fn verb(&self) -> HttpVerb {
        self.verb.clone()
    }
    pub fn add_to_body(&mut self, c: char) {
        self.body.add(c);
    }
    pub fn remove_from_body(&mut self) {
        self.body.pop();
    }
    pub fn body(&self) -> Body {
        self.body.clone()
    }
    pub fn resp_body_formatted(&self) -> String {
        if let Some(resp) = &self.response {
            if let Some(body) = &resp.body {
                let mut ct: Option<(&String, &String)> = None;
                if let Some(ref res) = self.response {
                    if let Some(headers) = &resp.headers {
                        ct = headers
                            .iter()
                            .filter(|item| item.0 == "content-type")
                            .last();
                    };
                    match ct {
                        Some(content_type) => {
                            if content_type.1.contains("application/json") {
                                return serde_json::to_string_pretty(
                                    &serde_json::from_str::<serde_json::Value>(&body.clone())
                                        .unwrap(),
                                )
                                .unwrap()
                                .to_string();
                            } else {
                                return body.clone();
                            }
                        }
                        None => return body.clone(),
                    };
                };
            };
        };
        String::from("")
    }
    pub fn status_code(&self) -> i32 {
        if let Some(sc) = &self.response {
            return sc.status_code;
        }
        0
    }
    pub fn headers(&self) -> Option<Vec<(String, String, bool)>> {
        self.headers.clone()
    }
    pub fn headers_len(&self) -> usize {
        if let Some(headers) = &self.headers {
            return headers.len();
        }
        0
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
    pub fn set_body(&mut self, b: Body) {
        self.body = b;
    }
    pub fn handle_json_body(&self) -> Result<Option<serde_json::Value>, crate::app::Error> {
        match &self.body.payload {
            Some(data) => {
                serde_json::from_str(&*data.clone()).map_err(|e| crate::app::Error::JsonErr(e))
            }
            None => Ok(None),
        }
    }
    pub fn params(&self) -> Option<Vec<(String, String, bool)>> {
        self.params.clone()
    }
    pub fn params_len(&self) -> usize {
        if let Some(params) = &self.params {
            return params.len();
        }
        0
    }
    pub fn set_response_headers(&mut self, h: &HeaderMap) -> Result<(), crate::app::Error> {
        let headers = handle_response_headers(h)?;
        if let Some(ref mut resp) = self.response {
            resp.headers = Some(headers);
        } else {
            self.response = Some(Response {
                headers: Some(headers),
                body: None,
                status_code: 0,
            })
        }
        Ok(())
    }
    pub fn set_response_body(&mut self, b: String) {
        if let Some(resp) = &mut self.response {
            resp.body = Some(b);
        } else {
            self.response = Some(Response {
                headers: None,
                body: Some(b),
                status_code: 0,
            })
        }
    }
    pub fn set_response_status_code(&mut self, sc: i32) {
        if let Some(resp) = &mut self.response {
            resp.status_code = sc;
        } else {
            self.response = Some(Response {
                headers: None,
                body: None,
                status_code: sc,
            })
        }
    }
    pub fn set_response(
        &mut self,
        body: String,
        headers: &HeaderMap,
        sc: i32,
    ) -> Result<(), crate::app::Error> {
        let headers = handle_response_headers(headers)?;
        self.response = Some(Response {
            headers: Some(headers),
            body: Some(body),
            status_code: sc,
        });
        Ok(())
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
}
