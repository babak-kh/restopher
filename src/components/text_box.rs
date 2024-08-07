use crate::keys::keys::{Event, Key};
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
    pub fn from(s: String) -> Self {
        let len = s.len();
        TextBox {
            buffer: s,
            cursor_pos: len,
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
    pub fn get_content(&self) -> String {
        self.buffer.clone()
    }
    pub fn update(&mut self, event: &Event) {
        match event.key {
            Key::Char(c) => {
                self.push(c);
            }
            Key::Backspace => {
                self.pop();
            }
            Key::Left => {
                self.cursor_pre();
            }
            Key::Right => {
                self.cursor_next();
            }
            _ => {}
        }
    }
}
