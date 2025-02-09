use super::default_block;
use crate::keys::keys::{Event, Key};
use ratatui::{
    layout::Rect,
    prelude::*,
    text::{Span, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};
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
    pub fn add_to_buffer(&mut self, to_add: String) {
        self.buffer.push_str(&to_add);
        self.cursor_pos += to_add.len();
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
        if self.cursor_pos == 0 {
            return;
        }
        self.cursor_pos -= 1;
    }
    pub fn cursor_next(&mut self) {
        if self.cursor_pos == self.buffer.len() {
            return;
        }
        self.cursor_pos += 1;
    }
    pub fn cursor_position(&self) -> usize {
        self.cursor_pos
    }
    pub fn get_content(&self) -> String {
        self.buffer.clone()
    }
    pub fn get_index(&self) -> usize {
        self.cursor_pos
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
    pub fn draw(&self, f: &mut Frame, rect: Rect, name: &str, is_focused: bool) {
        let cont = self.get_content();
        let mut spans: Vec<Span> = vec![Span::from(cont.clone())];

        if is_focused {
            if self.cursor_pos >= self.buffer.len() {
                spans = vec![Span::from(cont), Span::from("_")];
            } else {
                let (left, right) = cont.split_at(self.cursor_pos);
                let (first, rest) = right.split_at(1);
                let rest2 = {
                    (
                        Span::from(first).style(Style::default().underlined()),
                        Span::from(rest),
                    )
                };
                spans = vec![Span::from(left), rest2.0, rest2.1];
            }
        }

        f.render_widget(
            Paragraph::new(Text::from(Line::from(spans)))
                .block(default_block(Some("Address"), is_focused))
                .wrap(Wrap { trim: true }),
            rect,
        )
    }
}
