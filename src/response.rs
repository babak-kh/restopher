use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
   pub  headers: Option<HashMap<String,String>>,
   pub  body: Option<String>,
   pub  status_code: i32,
}
