use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{List, ListItem, ListState},
    Frame,
};

use crate::components::default_block;

pub struct RequestsComponent {
    names: Vec<String>,
    selected: Option<usize>,
    is_focused: bool,
}

impl RequestsComponent {
    pub fn new(names: Vec<String>, idx: usize) -> Self {
        Self {
            names: vec![],
            selected: None,
            is_focused: false,
        }
    }
    pub fn get_selected(&self) -> Option<&String> {
        self.selected.map(|i| &self.names[i])
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
    pub fn draw(&self, f: &mut Frame, rect: Rect) {
        let items: Vec<ListItem> = self
            .names
            .iter()
            .map(|name| ListItem::new(Span::raw(name.as_str())))
            .collect();
        let items = List::new(items)
            .block(default_block("Requests", self.is_focused))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::LightBlue),
            )
            .highlight_symbol(">> ");
        let mut state = ListState::default().with_selected(self.selected);
        f.render_stateful_widget(items, rect, &mut state);
    }
}
