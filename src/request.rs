use std::collections::HashMap;

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
pub struct Request {
    pub headers: Option<HashMap<String,String>>,
    pub params: Option<HashMap<String,String>>,
    pub body: Option<String>,
    pub address: String,
    pub verb: HttpVerb,
}
