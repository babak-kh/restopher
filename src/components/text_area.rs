use copypasta::{ClipboardContext, ClipboardProvider};
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame,
};

use crate::{
    components::default_block,
    keys::keys::{Event, Key, Modifier as keyModifier},
    styles::cursor_like_span,
    trace_dbg,
};

pub struct TextArea {
    lines: Vec<String>,
    cursor_pos: (usize, usize),
    is_focused: bool,
    mutable: bool,
    error: String,
}

impl TextArea {
    pub fn new() -> Self {
        TextArea {
            lines: vec![String::from("")],
            cursor_pos: (0, 0),
            is_focused: true,
            mutable: true,
            error: String::from(""),
        }
    }
    pub fn from(lines: String, is_focused: bool, mutable: bool) -> Self {
        TextArea {
            lines: lines
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
            cursor_pos: (0, 0),
            is_focused,
            mutable,
            error: String::from(""),
        }
    }
    pub fn set_focus(&mut self, focus: bool) {
        self.is_focused = focus;
    }
    pub fn lose_focus(&mut self) {
        self.is_focused = false;
    }
    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }
    pub fn new_line(&mut self) {
        if self.is_on_last_line() {
            self.lines.push(String::from(""));
            self.set_cursor_to_begin_of_last_line();
        } else {
            self.lines.insert(self.cursor_pos.1 + 1, String::new());
            self.cursor_pos = (0, self.cursor_pos.1 + 1);
        }
    }
    fn set_cursor_to_begin_of_last_line(&mut self) {
        if self.is_on_last_line() {
            return;
        }
        self.cursor_pos.1 = self.lines.len() - 1;
        self.cursor_pos.0 = 0;
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
        if self.lines.is_empty() {
            return true;
        }
        self.cursor_pos.1 == self.lines.len() - 1
    }
    fn is_on_first_line(&self) -> bool {
        self.cursor_pos.1 == 0
    }
    fn is_on_last_char(&self) -> bool {
        self.cursor_pos.0 == self.lines[self.cursor_pos.1].len()
    }
    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }
    pub fn set_lines(&mut self, lines: String) {
        self.lines = lines
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
    }
    pub fn add_to_lines(&mut self, s: String) {
        s.lines()
            .rev()
            .map(|l| l.to_string())
            .for_each(|l| self.lines.insert(self.cursor_pos.1, l));
        self.cursor_pos = (0, 0);
    }
    pub fn update(&mut self, event: &Event) {
        if let Some(modif) = &event.modifier {
            match modif {
                keyModifier::Control => match event.key {
                    Key::Char('v') => {
                        let mut ctx = ClipboardContext::new().unwrap();
                        self.add_to_lines(ctx.get_contents().unwrap());
                        return;
                    }
                    _ => (),
                },
                _ => {}
            }
        }
        match event.key {
            Key::Char(c) => {
                if self.mutable {
                    self.push(c);
                }
            }
            Key::Backspace => {
                if self.mutable {
                    self.pop();
                }
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
                if self.mutable {
                    self.push('\n');
                }
            }
            _ => {}
        }
    }

    pub fn prettify_json(&mut self) {
        self.lines = serde_json::to_string_pretty(
            &serde_json::from_str::<serde_json::Value>(&self.lines.clone().join("\n")).unwrap(),
        )
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
        self.go_to_last_line()
    }
    pub fn go_to_last_line(&mut self) {
        self.cursor_pos.1 = self.lines.len() - 1;
        self.cursor_pos.0 = self.lines[self.cursor_pos.1].len();
    }
    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        let mut chunks: Vec<Rect> = vec![Rect::default(), rect];
        if !self.error.is_empty() {
            let chk =
                Layout::vertical(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
                    .split(rect);
            chunks = vec![chk[0], chk[1]];
            let paragraph = Paragraph::new(self.error.clone())
                .style(Style::default().fg(Color::Red))
                .block(default_block(Some("Error"), self.is_focused))
                .wrap(Wrap { trim: false });
            f.render_widget(paragraph, chunks[0]);
        }

        let display_lines = self.lines.clone();
        let mut modified_lines = Vec::new();
        let actual_height = chunks[1].height as usize - 2;
        let actual_width = chunks[1].width as usize - 20;
        let mut repeat: usize = 0;
        for line_idx in 0..display_lines.len() {
            let line = display_lines.get(line_idx).unwrap();
            split_line_with_width(&line, actual_width, &mut modified_lines, &mut repeat);
        }
        //self.lines = modified_lines.clone();
        let mut diff = 0;
        if self.cursor_pos.1 >= actual_height + 1 {
            diff = self.cursor_pos.1 - actual_height + 1;
            modified_lines = modified_lines[diff..].to_vec();
        };
        let paragraph = Paragraph::new(TextArea::style_cursor(&self.lines, self.cursor_pos, diff))
            .block(default_block(Some("Body"), self.is_focused))
            .wrap(Wrap { trim: false });
        trace_dbg!(level: tracing::Level::INFO, self.cursor_pos);
        //trace_dbg!(level: tracing::Level::INFO, self.lines.clone());
        f.render_widget(paragraph, chunks[1]);

        let mut state = ScrollbarState::default()
            .content_length(self.lines.len())
            .position(self.cursor_pos.1)
            .viewport_content_length(chunks[1].height as usize);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some("▒"))
                .thumb_symbol("▐"),
            chunks[1].inner(Margin {
                vertical: 1,
                horizontal: 2,
            }),
            &mut state,
        );
    }
    fn calculate_wrapped_position(
        original_pos: (usize, usize),
        width: usize,
        lines: &[String],
    ) -> (usize, usize) {
        let mut wrapped_line = 0;
        let mut char_count = 0;

        // Count wrapped lines up to the cursor's line
        for (idx, line) in lines[..=original_pos.1].iter().enumerate() {
            if idx < original_pos.1 {
                wrapped_line += (line.len() + width - 1) / width;
            } else {
                // For the cursor's line, calculate exact position
                let full_lines = original_pos.0 / width;
                wrapped_line += full_lines;
                char_count = original_pos.0 % width;
            }
        }

        (char_count, wrapped_line)
    }

    fn style_cursor(content: &Vec<String>, cursor_position: (usize, usize), _: usize) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let width = 80; // This should match actual_width from draw()
        let wrapped_pos = Self::calculate_wrapped_position(cursor_position, width, content);

        let mut current_wrapped_line = 0;
        for line in content {
            // Create chunks as owned Strings
            let chunks: Vec<String> = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(width)
                .map(|c| c.iter().collect::<String>())
                .collect();

            for chunk in chunks {
                if current_wrapped_line == wrapped_pos.1 {
                    let mut ll = Line::default();
                    if wrapped_pos.0 >= chunk.len() {
                        ll.push_span(Span::raw(chunk));
                        ll.push_span(cursor_like_span(' '));
                    } else {
                        let (before, rest) = chunk.split_at(wrapped_pos.0);
                        ll.push_span(Span::raw(before.to_string()));
                        let cursor_char = rest.chars().next().unwrap_or(' ');
                        ll.push_span(cursor_like_span(cursor_char));
                        if wrapped_pos.0 + 1 < chunk.len() {
                            ll.push_span(Span::raw(rest[1..].to_string()));
                        }
                    }
                    lines.push(ll);
                } else {
                    lines.push(Line::raw(chunk));
                }
                current_wrapped_line += 1;
            }
        }
        lines
    }
    pub fn get_flattened_cursor_position(&self) -> usize {
        let mut pos = 0;
        for (i, line) in self.lines.iter().enumerate() {
            if i < self.cursor_pos.1 {
                pos += line.len() + 1;
            }
        }
        pos + self.cursor_pos.0
    }
}

fn split_line_with_width(
    line: &str,
    width: usize,
    modified_line: &mut Vec<String>,
    repeat: &mut usize,
) {
    if line.len() > width {
        *repeat += 1;
        let subs = line.split_at(width);
        split_line_with_width(subs.0, width, modified_line, repeat);
        split_line_with_width(subs.1, width, modified_line, repeat);
    } else {
        modified_line.push(line.to_string());
        return;
    }
}
