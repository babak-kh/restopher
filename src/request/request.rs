use std::collections::HashMap;

use super::view::ReqView;
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
    //pub fn render(&self, f: &mut Frame, r: Rect, state: &State) {
    //    let response_headers = handle_response_headers();
    //    f.render_widget(default_block("Response"), r);
    //}
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
    pub name: String,
    pub headers: Option<Vec<(String, String, bool)>>,
    pub params: Option<Vec<(String, String, bool)>>,
    pub body: Body,
    pub address: String,
    pub verb: HttpVerb,
    pub response: Option<Response>,
    req_view: ReqView,
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
            address: "".to_string(),
            verb: HttpVerb::GET,
            response: None,
            req_view: ReqView::new(),
        }
    }
    pub fn name(&self) -> String {
        if self.name.is_empty() {
            return self.name.clone();
        }
        let mut n = self.address.clone();
        if n.len() >= 10 {
            n = n[0..9].to_string();
        };
        n
    }
    pub fn address(&self) -> String {
        self.address.to_string()
    }
    pub fn add_to_header(&mut self, key: String, value: String, active: bool) {
        if !key.is_empty() && !value.is_empty() {
            self.add_to_header(key, value, active)
        }
    }
    pub fn add_to_param(&mut self, key: String, value: String, active: bool) {
        if !key.is_empty() && !value.is_empty() {
            self.add_to_param(key, value, active)
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
    //pub fn handle_headers(&self) -> HashMap<String, String> {
    //    self.()
    //}
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
    //pub fn add_to_active_header(&mut self, ch: char) {
    //    self.view.add_to_active_header(ch)
    //}
    //pub fn delete_selected_header(&mut self, idx: usize) {
    //    self.delete_header(idx)
    //}
    //pub fn active_deactive_header(&mut self, idx: usize) {
    //    self.active_deactive_header(idx)
    //}
    //pub fn remove_from_active_header(&mut self) {
    //    self.view.remove_from_active_header()
    //}
    //pub fn change_active_header(&mut self) {
    //    self.view.change_active_header()
    //}
    //pub fn is_key_active_in_header(&self) -> bool {
    //    self.view.is_key_active_in_header()
    //}

    //pub fn delete_selected_param(&mut self) {
    //    let idx = self.view.param_idx();
    //    self.delete_param(idx)
    //}
    //pub fn active_deactive_param(&mut self) {
    //    let idx = self.view.param_idx();
    //    self.active_deactive_param(idx)
    //}
    //pub fn add_to_active_param(&mut self, ch: char) {
    //    self.view.add_to_active_header(ch)
    //}
    //pub fn remove_from_active_param(&mut self) {
    //    self.view.remove_from_active_param()
    //}
    //pub fn change_active_param(&mut self) {
    //    self.view.change_active_param()
    //}
    //pub fn is_key_active_in_param(&self) -> bool {
    //    self.view.is_key_active_in_param()
    //}
    //pub fn response_headers(&self) -> Option<HashMap<String, String>> {
    //    if let Some(resp) = &self.response {
    //        return resp.headers.clone();
    //    }
    //    None
    //}
    // headers
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
