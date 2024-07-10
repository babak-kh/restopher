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
    environments::KV,
    keys::keys::{Event, Key, Modifier},
    request::Request,
};
use view::Focus;

pub struct RequestTabComponent<'a> {
    focus: Focus,
    focused: bool,
    req_tabs: ReqTabs<'a>,
    new_header: KV,
    new_param: KV,
}

impl<'a> RequestTabComponent<'a> {
    pub fn new() -> Self {
        RequestTabComponent {
            focus: Focus::Header(0),
            focused: true,
            req_tabs: ReqTabs::new(),
            new_param: KV::new(),
            new_header: KV::new(),
        }
    }
    pub fn update_inner_focus(&mut self) {
        self.req_tabs.next();
    }
    pub fn update(&mut self, req: &mut Request, event: Event) {
        match &self.focus {
            Focus::NewHeaderKey => match event.key {
                Key::Enter => {
                    req.add_to_header(
                        self.new_header.key.clone(),
                        self.new_header.value.clone(),
                        true,
                    );
                    self.focus = Focus::Header(0);
                }
                Key::Tab => {
                    self.focus = Focus::NewHeaderValue;
                }
                Key::Char(x) => {
                    if self.new_header.is_key_active {
                        self.new_header.key.push(x);
                    } else {
                        self.new_header.value.push(x);
                    }
                }
                _ => (),
            },
            Focus::NewHeaderValue => todo!(),
            Focus::NewParamKey => todo!(),
            Focus::NewParamValue => match event.key {
                Key::Enter => {
                    req.add_to_param(
                        self.new_header.key.clone(),
                        self.new_header.value.clone(),
                        true,
                    );
                    self.focus = Focus::Param(0);
                }
                Key::Tab => {
                    self.focus = Focus::NewParamValue;
                }
                Key::Char(x) => {
                    if self.new_param.is_key_active {
                        self.new_param.key.push(x);
                    } else {
                        self.new_param.value.push(x);
                    }
                }
                _ => (),
            },
            Focus::Header(_) => {
                if let Some(modifier) = event.modifier {
                    match modifier {
                        Modifier::Control => match event.key {
                            Key::Char('n') => {
                                self.focus = Focus::NewHeaderKey;
                            }
                            Key::Char('p') => println!("header end"),
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            Focus::Param(_) => todo!(),
            Focus::None => (),
            Focus::Body => (),
        }
    }
    pub fn is_focused(&self) -> bool {
        self.focused
    }
    pub fn lose_focus(&mut self) {
        self.focused = false;
    }
    pub fn gain_focus(&mut self) {
        self.focused = true;
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
            RequestTabOptions::Headers(_, _) => {
                if Focus::NewHeaderValue == self.focus || self.focus == Focus::NewParamKey {
                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[1]);
                }
                f.render_widget(
                    Paragraph::new("Req Headers").block(default_block("headers", self.focused)),
                    chunks[1],
                )
            }
            RequestTabOptions::Body(_, _) => f.render_widget(
                Paragraph::new("Req body").block(default_block("body", self.focused)),
                chunks[1],
            ),
            RequestTabOptions::Params(_, _) => f.render_widget(
                Paragraph::new("Req params").block(default_block("params", self.focused)),
                chunks[1],
            ),
        }
    }
}
