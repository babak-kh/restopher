use super::text_box::TextBox;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    Frame,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct KVElement {
    text: TextBox,
    active: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KV {
    key: KVElement,
    value: KVElement,
}

impl KV {
    pub fn new() -> Self {
        KV {
            key: KVElement {
                text: TextBox::new(),
                active: true,
            },
            value: KVElement {
                text: TextBox::new(),
                active: false,
            },
        }
    }
    pub fn from(key: String, value: String) -> Self {
        KV {
            key: KVElement {
                text: TextBox::from(key),
                active: true,
            },
            value: KVElement {
                text: TextBox::from(value),
                active: false,
            },
        }
    }
    pub fn change_active(&mut self) {
        self.value.active = !self.value.active;
        self.key.active = !self.key.active;
    }
    pub fn add_to_active(&mut self, ch: char) {
        if self.key.active {
            self.key.text.push(ch);
            return;
        }
        self.value.text.push(ch);
    }
    pub fn paste(&mut self, text: String) {
        if self.key.active {
            self.key.text.paste(text);
            return;
        }
        self.value.text.paste(text);
    }
    pub fn remove_from_active(&mut self) {
        if self.key.active {
            self.key.text.pop();
            return;
        }
        self.value.text.pop();
    }
    pub fn get_key(&self) -> String {
        self.key.text.to_string()
    }
    pub fn get_key_spans(&self) -> Vec<Span> {
        let mut spans: Vec<Span> = Vec::new();
        self.key
            .text
            .get_content_styled(&mut spans, self.key.active);
        spans
    }
    pub fn get_value_spans(&self) -> Vec<Span> {
        let mut spans: Vec<Span> = Vec::new();
        self.value
            .text
            .get_content_styled(&mut spans, self.value.active);
        spans
    }
    pub fn get_value(&self) -> String {
        self.value.text.to_string()
    }
    pub fn clear(&mut self) {
        self.key.text.clear();
        self.value.text.clear();
        self.key.active = true;
    }
    pub fn draw(&self, f: &mut Frame, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(rect);
        self.key.text.draw(f, chunks[0], "Key", self.key.active);
        self.value
            .text
            .draw(f, chunks[1], "Value", self.value.active);
    }
}
