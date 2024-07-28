use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame,
};
use serde_json::Value;
use tracing::trace;

use crate::{
    keys::keys::{Event, Key, Modifier as keyModifier},
    styles::cursor_like_span,
    trace_dbg,
};

pub enum Kind {
    Plain,
    Json,
}
pub struct TextArea {
    lines: Vec<String>,
    cursor_pos: (usize, usize),
    kind: Kind,
}

impl TextArea {
    pub fn new() -> Self {
        TextArea {
            lines: Vec::new(),
            cursor_pos: (0, 0),
            kind: Kind::Json,
        }
    }
    pub fn from(s: String) -> Self {
        let len = s.len();

        TextArea {
            lines: s.lines().map(|l| l.to_string()).collect::<Vec<String>>(),
            cursor_pos: (0, 0),
            kind: Kind::Json,
        }
    }
    pub fn new_line(&mut self) {
        if self.cursor_pos.1 == self.lines.len() - 1 {
            self.lines.push(String::new());
            self.cursor_pos.1 = self.lines.len() - 1;
            self.cursor_pos.0 = 0;
        } else {
            self.lines.insert(self.cursor_pos.1 + 1, String::new());
            self.cursor_pos = (0, self.cursor_pos.1 + 1);
        }
    }
    pub fn push(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
            return;
        }
        match self.lines.get_mut(self.cursor_pos.1) {
            None => {
                self.lines.push(String::from(c));
            }
            Some(l) => l.insert(self.cursor_pos.0, c),
        }
        self.cursor_pos.0 += 1;
    }
    pub fn pop(&mut self) {
        if self.cursor_pos.0 == 0 && self.cursor_pos.1 == 0 {
            return;
        }
        if self.cursor_pos.0 == 0 {
            self.cursor_pos.1 -= 1;
            self.cursor_pos.0 = self.lines[self.cursor_pos.1].len();
            self.lines[self.cursor_pos.1 + 1].pop();
            return;
        }
        self.lines
            .get_mut(self.cursor_pos.1)
            .unwrap()
            .remove(self.cursor_pos.0 - 1);
        self.cursor_pos.0 -= 1;
    }
    pub fn cursor_pre(&mut self) {
        if self.cursor_pos.0 == 1 && self.cursor_pos.1 == 0 {
            return;
        }
        if self.cursor_pos.0 == 0 {
            self.cursor_pos.1 -= 1;
            self.cursor_pos.0 = self.lines[self.cursor_pos.1].len();
            return;
        }
        self.cursor_pos.0 -= 1;
    }
    pub fn cursor_next(&mut self) {
        if self.is_on_last_line() && self.is_on_last_char() {
            return;
        }
        if !self.is_on_last_line() && self.is_on_last_char() {
            self.cursor_pos.1 += 1;
            self.cursor_pos.0 = 0;
            return;
        }
        self.cursor_pos.0 += 1;
    }
    pub fn cursor_up(&mut self) {
        if self.is_on_first_line() {
            return;
        }
        self.cursor_pos.1 -= 1;
        match self.lines[self.cursor_pos.1].chars().nth(self.cursor_pos.0) {
            None => {
                self.cursor_pos.0 = self.lines[self.cursor_pos.1].len();
            }
            Some(_) => {}
        }
    }
    pub fn cursor_down(&mut self) {
        if self.is_on_last_line() {
            return;
        }
        self.cursor_pos.1 += 1;
        match self.lines[self.cursor_pos.1].chars().nth(self.cursor_pos.0) {
            None => {
                self.cursor_pos.0 = self.lines[self.cursor_pos.1].len();
            }
            Some(_) => {}
        }
    }
    fn is_on_last_line(&self) -> bool {
        self.cursor_pos.1 == self.lines.len() - 1
    }
    fn is_on_first_line(&self) -> bool {
        self.cursor_pos.1 == 0
    }
    fn is_on_last_char(&self) -> bool {
        self.cursor_pos.0 == self.lines[self.cursor_pos.1].len()
    }
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor_pos
    }
    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }
    pub fn update(&mut self, event: &Event) {
        if let Some(modif) = &event.modifier {
            match modif {
                keyModifier::Control => match event.key {
                    Key::Char('b') => {
                        self.kind = Kind::Json;
                        self.format_json_mut();
                    }
                    _ => {}
                },
                _ => {}
            }
            return;
        }
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
            Key::Up => {
                self.cursor_up();
            }
            Key::Down => {
                self.cursor_down();
            }
            Key::Enter => {
                self.push('\n');
            }
            _ => {}
        }
    }

    fn format_json(&self) -> (String, String) {
        match self.kind {
            Kind::Plain => (self.lines.clone().join("\n"), String::from("")),
            Kind::Json => serde_json::from_str(&self.lines.clone().join("\n")).map_or_else(
                |e| {
                    let mut error = String::from("");
                    error.push_str("Error: ");
                    error.push_str(e.to_string().as_str());
                    (String::new(), error)
                },
                |data: Value| {
                    let content = serde_json::to_string_pretty(&data).unwrap();
                    (content, String::from(""))
                },
            ),
        }
    }
    fn format_json_mut(&mut self) {
        let (formatted, error) = self.format_json();
        if !error.is_empty() {
            return;
        }
        self.lines = formatted
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        self.cursor_pos = (0, 0);
    }
    pub fn draw(&self, f: &mut Frame, rect: Rect) {
        let mut chunks: Vec<Rect> = vec![Rect::default(), rect];
        let (_, error) = self.format_json();
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

        let mut display_lines = self.lines.clone();
        let actual_height = chunks[1].height as usize - 2;
        trace_dbg!(
            level:tracing::Level::INFO,
            (self.cursor_pos, chunks[1].height as usize)
        );
        let mut diff = 0;
        if self.cursor_pos.1 - 1 >= actual_height {
            diff = self.cursor_pos.1 - 1 - actual_height;
            display_lines = display_lines[diff..].to_vec();
        };
        let paragraph = Paragraph::new(TextArea::prepare_body(
            &display_lines,
            self.cursor_pos,
            diff,
        ))
        .block(Block::default().borders(Borders::ALL).title("TextArea"))
        .wrap(Wrap { trim: false });
        f.render_widget(paragraph, chunks[1]);

        let mut state = ScrollbarState::default()
            .content_length(self.lines.len())
            .viewport_content_length(chunks[1].height as usize);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(None)
                .thumb_symbol("▐"),
            chunks[1].inner(Margin {
                vertical: 1,
                horizontal: 2,
            }),
            &mut state,
        );
    }
    fn prepare_body(
        content: &Vec<String>,
        cursor_position: (usize, usize),
        moved: usize,
    ) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        for (idx, line) in content.iter().enumerate() {
            if cursor_position.1 == idx + moved {
                if cursor_position.0 == line.len() {
                    let mut ll = Line::raw(line);
                    ll.push_span(cursor_like_span(' '));
                    lines.push(ll);
                } else {
                    let mut ll = Line::default();
                    ll.push_span(Span::raw(&line[..cursor_position.0]));
                    ll.push_span(cursor_like_span(
                        line.chars().nth(cursor_position.0).unwrap(),
                    ));
                    ll.push_span(Span::raw(&line[cursor_position.0 + 1..]));
                    lines.push(ll);
                };
            } else {
                lines.push(Line::raw(line));
            }
        }
        lines
    }
}
