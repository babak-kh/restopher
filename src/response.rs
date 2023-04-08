use std::collections::HashMap;

#[derive(Debug)]
struct Response {
    headers: Option<HashMap<String,String>>,
    body: Option<String>,
    status_code: i32,
}
