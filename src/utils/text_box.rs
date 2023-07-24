use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TextBox {
    buffer: String,
    CursorPos: usize,
}
impl TextBox {
    pub fn new() -> Self {
        TextBox {
            buffer: String::from(""),
            CursorPos: 0,
        }
    }
    pub fn to_string(&self) -> String {
        self.buffer.clone()
    }
    pub fn push(&mut self, c: char) {
        self.buffer.insert(self.CursorPos, c);
        self.CursorPos += 1;
    }
    pub fn pop(&mut self) {
        self.buffer.remove(self.CursorPos);
    }
    pub fn cursor_pre(&mut self) {
        self.CursorPos -= 1;
    }
    pub fn cursor_next(&mut self) {
        self.CursorPos += 1;
    }
    pub fn cursor_position(&self) -> usize {
        self.CursorPos
    }
}
