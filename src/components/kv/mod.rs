use std::collections::HashMap;

use crate::components::default_block;

use super::text_box::TextBox;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
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
    pub fn remove_from_active(&mut self) {
        if self.key.active {
            self.key.text.pop();
            return;
        }
        self.value.text.pop();
    }
    pub fn is_key_active(&self) -> bool {
        self.key.active
    }
    pub fn get_key(&self) -> String {
        self.key.text.to_string()
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
        f.render_widget(
            Paragraph::new(self.key.text.to_string())
                .style(Style::default().fg(Color::White))
                .block(default_block("Key", self.key.active)),
            chunks[0],
        );
        f.render_widget(
            Paragraph::new(self.value.text.to_string())
                .style(Style::default().fg(Color::White))
                .block(default_block("Value", self.value.active)),
            chunks[1],
        );
    }
}
