mod request_tab;
mod view;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
pub use request_tab::{ReqTabs, RequestTabOptions};

use crate::{
    components::{default_block, tabs, KV},
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
        self.focus = self.focus.next();
        self.req_tabs.next();
    }
    pub fn update(&mut self, req: &mut Request, event: Event) {
        match &self.focus {
            Focus::NewHeaderKV => match event.key {
                Key::Enter => {
                    req.add_to_header(self.new_header.get_key(), self.new_header.get_value(), true);
                    self.new_header.clear();
                    self.focus = Focus::Header(0);
                }
                Key::Tab => {
                    self.new_header.change_active();
                }
                Key::Char(x) => {
                    self.new_header.add_to_active(x);
                }
                Key::Backspace => {
                    self.new_header.remove_from_active();
                }
                Key::Esc => self.focus = Focus::Header(0),
                _ => (),
            },
            Focus::NewParamKV => match event.key {
                Key::Enter => {
                    req.add_to_param(self.new_param.get_key(), self.new_param.get_value(), true);
                    self.new_param.clear();
                    self.focus = Focus::Param(0);
                }
                Key::Tab => {
                    self.new_param.change_active();
                }
                Key::Char(x) => {
                    self.new_param.add_to_active(x);
                }
                Key::Backspace => {
                    self.new_param.remove_from_active();
                }
                Key::Esc => self.focus = Focus::Param(0),
                _ => (),
            },
            Focus::Header(_) => {
                if let Some(modifier) = event.modifier {
                    match modifier {
                        Modifier::Control => match event.key {
                            Key::Char('n') => {
                                self.focus = Focus::NewHeaderKV;
                                self.new_header = KV::new();
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            Focus::Param(_) => {
                if let Some(modifier) = event.modifier {
                    match modifier {
                        Modifier::Control => match event.key {
                            Key::Char('n') => {
                                self.focus = Focus::NewParamKV;
                                self.new_param = KV::new();
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            Focus::Body => match event.key {
                Key::Char(x) => {
                    req.add_to_body(x);
                }
                Key::Backspace => {
                    req.remove_from_body();
                }
                _ => (),
            },
            Focus::None => (),
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
                self.focused,
            ),
            chunks[0],
        );
        match self.req_tabs.active() {
            RequestTabOptions::Headers(_, _) => {
                match self.focus {
                    Focus::NewHeaderKV => {
                        let chunksVer = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                            .split(chunks[1]);
                        render_items(f, &request.headers(), Some(0), self.focused, chunksVer[0]);
                        self.new_header.draw(f, chunksVer[1]);
                    }
                    _ => {
                        render_items(f, &request.headers(), Some(0), self.focused, chunks[1]);
                    }
                };
            }
            RequestTabOptions::Body(_, _) => f.render_widget(
                Paragraph::new(request.body().to_string()).block(default_block(
                    "Body",
                    self.focused && matches!(self.focus, Focus::Body),
                )),
                chunks[1],
            ),
            RequestTabOptions::Params(_, _) => {
                match self.focus {
                    Focus::NewParamKV => {
                        let chunksVer = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                            .split(chunks[1]);
                        render_items(f, &request.params(), Some(0), self.focused, chunksVer[0]);
                        self.new_param.draw(f, chunksVer[1]);
                    }
                    _ => {
                        render_items(f, &request.params(), Some(0), self.focused, chunks[1]);
                    }
                };
            }
        }
    }
}

fn render_items(
    f: &mut Frame,
    items: &Option<Vec<(String, String, bool)>>,
    selected: Option<usize>,
    focused: bool,
    rect: Rect,
) {
    if let Some(items) = items {
        let mut rows = Vec::new();
        for item in items {
            rows.push(Row::new(vec![
                Cell::from(item.0.clone()),
                Cell::from(item.1.clone()),
                Cell::from(format!("{}", item.2)),
            ]));
        }
        // Create table
        f.render_widget(
            Table::new(
                rows,
                vec![
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length(10),
                ],
            )
            .header(Row::new(vec!["Key", "Value", "Active"]))
            .block(Block::default().borders(Borders::ALL).title("Table")),
            rect,
        );
    }
    f.render_widget(
        Paragraph::new("Req Headers").block(default_block("headers", focused)),
        rect,
    );
}
