mod request_tab;
mod view;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::Paragraph,
    Frame,
};
pub use request_tab::{ReqTabs, RequestTabOptions};

use crate::{
    components::{default_block, tabs},
    keys::keys::Key,
    request::Request,
};
use view::Focus;

pub struct RequestTabComponent<'a> {
    focus: Focus,
    active: bool,
    req_tabs: ReqTabs<'a>,
}

impl<'a> RequestTabComponent<'a> {
    pub fn new() -> Self {
        RequestTabComponent {
            focus: Focus::None,
            active: true,
            req_tabs: ReqTabs::new(),
        }
    }
    pub fn update(&self, req: &mut Request, key: Key) {
        match &self.focus {
            Focus::NewHeaderKey => todo!(),
            Focus::NewHeaderValue => todo!(),
            Focus::NewParamKey => todo!(),
            Focus::NewParamValue => todo!(),
            Focus::Header(_) => todo!(),
            Focus::Param(_) => todo!(),
            Focus::None => (),
        }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn lose_focus(&mut self) {
        self.active = false;
    }
    pub fn gain_focus(&mut self) {
        self.active = true;
    }
    pub fn draw(&self, f: &mut Frame, request: &Request, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(rect);

        f.render_widget(
            tabs(
                self.req_tabs
                    .req_tabs
                    .iter()
                    .map(|t| Span::from(t.to_string()))
                    .collect(),
                "Request data tabs",
                self.req_tabs.active_idx(),
            ),
            chunks[0],
        );
        match self.req_tabs.active() {
            RequestTabOptions::Headers(_, _) => f.render_widget(
                Paragraph::new("Req Headers").block(default_block("headers", self.active)),
                chunks[1],
            ),
            RequestTabOptions::Body(_, _) => f.render_widget(
                Paragraph::new("Req body").block(default_block("body", self.active)),
                chunks[1],
            ),
            RequestTabOptions::Params(_, _) => f.render_widget(
                Paragraph::new("Req params").block(default_block("params", self.active)),
                chunks[1],
            ),
        }
    }
}
