use crate::utils::text_box::TextBox;
use serde::{Deserialize, Serialize};

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
