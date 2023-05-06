use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

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
