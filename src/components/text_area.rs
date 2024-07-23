use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
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
        self.buffer.remove(self.cursor_pos);
        self.cursor_pos -= 1;
    }
    pub fn cursor_pre(&mut self) {
        if self.buffer.len() == 0 {
            return;
        }
        self.cursor_pos -= 1;
    }
    pub fn cursor_next(&mut self) {
        if self.cursor_pos == self.buffer.len() - 1 {
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
        let (content, error) = match self.kind {
            Kind::Plain => (self.buffer.clone(), String::from("")),
            Kind::Json => serde_json::from_str(self.buffer.clone().as_str()).map_or_else(
                |e| {
                    let mut error = String::from("");
                    error.push_str("Error: ");
                    error.push_str(e.to_string().as_str());
                    (self.buffer.clone(), error)
                },
                |data: String| {
                    let pretty = serde_json::to_string_pretty(&data).unwrap();
                    (pretty, String::from(""))
                },
            ),
        };
        let mut chunks: Vec<Rect> = vec![rect];
        if !error.is_empty() {
            let chk =
                Layout::vertical(vec![Constraint::Percentage(12), Constraint::Percentage(88)])
                    .split(rect);
            chunks = vec![chk[0], chk[1]];
            let paragraph = Paragraph::new(error)
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .wrap(Wrap { trim: false });
            f.render_widget(paragraph, chunks[0]);
        }
        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("TextArea"))
            .wrap(Wrap { trim: false });
        f.render_widget(paragraph, chunks[1]);
    }
}
