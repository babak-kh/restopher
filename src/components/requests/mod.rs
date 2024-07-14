use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{List, ListItem, ListState},
    Frame,
};

use crate::{
    components::{default_block, tabs},
    request::Request,
};

pub struct RequestsComponent {
    selected: Option<usize>,
    is_focused: bool,
}

impl RequestsComponent {
    pub fn new(names: Vec<String>, idx: usize) -> Self {
        Self {
            selected: None,
            is_focused: false,
        }
    }
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
    pub fn update(&mut self) {
        println!("update");
    }
    pub fn focus(&mut self) {
        self.is_focused = true;
    }
    pub fn lose_focus(&mut self) {
        self.is_focused = false;
    }
    pub fn gain_focus(&mut self) {
        self.is_focused = true;
    }
    pub fn blur(&mut self) {
        self.is_focused = false;
    }
    pub fn draw(&self, f: &mut Frame, names: Vec<String>, selected: usize, rect: Rect) {
        f.render_widget(
            tabs(
                names.iter().map(|t| Span::from(t.to_string())).collect(),
                "requests",
                selected,
                self.is_focused,
            ),
            rect,
        );
    }
}
