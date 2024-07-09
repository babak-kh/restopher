mod response_tab;
mod view;

use crate::{
    components::{default_block, tabs},
    keys::keys::{Event, Key, Modifier},
    request::Request,
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::Paragraph,
    Frame,
};
use response_tab::ResponseOptions;
use view::Focus;

pub struct ResponseTabComponent {
    focus: Focus,
    is_active: bool,
    resp_tabs: response_tab::RespTabs<'static>,
}

impl ResponseTabComponent {
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn new() -> Self {
        ResponseTabComponent {
            focus: Focus::None,
            is_active: false,
            resp_tabs: response_tab::RespTabs::new(),
        }
    }
    pub fn update(&self, req: &mut Request, event: &Event) {
        match &self.focus {
            Focus::None => (),
            Focus::Header(_) => todo!(),
            Focus::Body => todo!(),
        }
    }
    pub fn lose_focus(&mut self) {
        self.is_active = false;
    }
    pub fn gain_focus(&mut self) {
        self.is_active = true;
    }
    pub fn draw(&self, f: &mut Frame, req: &Request, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(rect);
        f.render_widget(
            tabs(
                self.resp_tabs
                    .resp_tabs
                    .iter()
                    .map(|t| Span::from(t.to_string()))
                    .collect(),
                "Request data tabs",
                self.resp_tabs.active_idx(),
            ),
            chunks[0],
        );
        match self.resp_tabs.active() {
            ResponseOptions::Headers(_, _) => f.render_widget(
                Paragraph::new("Resp Headers").block(default_block("headers", self.is_active)),
                chunks[1],
            ),
            ResponseOptions::Body(_, _) => f.render_widget(
                Paragraph::new("Resp body").block(default_block("resp body", self.is_active)),
                chunks[1],
            ),
        }
    }
}
