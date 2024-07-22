use ratatui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::keys::keys::{Event, Key};

pub enum Kind {
    Plain,
    Json,
}
pub struct TextArea {
    buffer: String,
    cursor_pos: usize,
    kind: Kind,
}

impl TextArea {
    pub fn new() -> Self {
        TextArea {
            buffer: String::from(""),
            cursor_pos: 0,
            kind: Kind::Json,
        }
    }
    pub fn from(s: String) -> Self {
        let len = s.len();
        TextArea {
            buffer: s,
            cursor_pos: len,
            kind: Kind::Json,
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
            Key::Enter => {
                self.push('\n');
            }
            _ => {}
        }
    }
    pub fn draw(&self, f: &mut Frame, rect: Rect) {
        let data: String = serde_json::from_str(self.buffer.clone().as_str()).unwrap();
        let paragraph = Paragraph::new(data)
            .block(Block::default().borders(Borders::ALL).title("TextArea"))
            .wrap(Wrap { trim: false });
        f.render_widget(paragraph, rect);
    }
}
