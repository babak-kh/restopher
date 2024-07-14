use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct TextBox {
    buffer: String,
    cursor_pos: usize,
}
impl TextBox {
    pub fn new() -> Self {
        TextBox {
            buffer: String::from(""),
            cursor_pos: 0,
        }
    }
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor_pos = 0;
    }
    pub fn to_string(&self) -> String {
        self.buffer.clone()
    }
    pub fn push(&mut self, c: char) {
        self.buffer.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }
    pub fn pop(&mut self) {
        if self.buffer.len() == 0 {
            return;
        }
        self.cursor_pos -= 1;
        self.buffer.remove(self.cursor_pos);
    }
    pub fn cursor_pre(&mut self) {
        self.cursor_pos -= 1;
    }
    pub fn cursor_next(&mut self) {
        self.cursor_pos += 1;
    }
    pub fn cursor_position(&self) -> usize {
        self.cursor_pos
    }
}
