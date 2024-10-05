mod request_tab;
mod view;

use crate::request::{Body, BodyKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
pub use request_tab::{ReqTabs, RequestTabOptions};

use crate::{
    components::{default_block, tabs, text_area::TextArea, KV},
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
    body_view: TextArea,
}

impl<'a> RequestTabComponent<'a> {
    pub fn new() -> Self {
        RequestTabComponent {
            focus: Focus::Header(0),
            focused: true,
            req_tabs: ReqTabs::new(),
            new_param: KV::new(),
            new_header: KV::new(),
            body_view: TextArea::new(),
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
            Focus::Body => {
                self.body_view.update(&event);
            }
            Focus::None => (),
        }
    }
    pub fn is_focused(&self) -> bool {
        self.focused
    }
    pub fn lose_focus(&mut self, request: &mut Request) {
        request.set_body(Body {
            payload: Some(self.body_view.get_content()),
            kind: BodyKind::JSON,
        });
        self.focused = false;
        self.body_view.lose_focus();
    }
    pub fn gain_focus(&mut self) {
        self.focused = true;
    }
    pub fn draw(&self, f: &mut Frame, request: &Request, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(rect);

        f.render_widget(
            tabs(
                self.req_tabs
                    .req_tabs
                    .iter()
                    .map(|t| Span::from(t.to_string()))
                    .collect(),
                Some("Request Data"),
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
                        render_items(
                            f,
                            "Headers",
                            &request.headers(),
                            Some(0),
                            self.focused,
                            chunksVer[0],
                        );
                        self.new_header.draw(f, chunksVer[1]);
                    }
                    _ => {
                        render_items(
                            f,
                            "Headers",
                            &request.headers(),
                            Some(0),
                            self.focused,
                            chunks[1],
                        );
                    }
                };
            }
            RequestTabOptions::Body(_, _) => self.body_view.draw(f, chunks[1]),
            RequestTabOptions::Params(_, _) => {
                match self.focus {
                    Focus::NewParamKV => {
                        let chunksVer = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                            .split(chunks[1]);
                        render_items(
                            f,
                            "Params",
                            &request.params(),
                            Some(0),
                            self.focused,
                            chunksVer[0],
                        );
                        self.new_param.draw(f, chunksVer[1]);
                    }
                    _ => {
                        render_items(
                            f,
                            "Params",
                            &request.params(),
                            Some(0),
                            self.focused,
                            chunks[1],
                        );
                    }
                };
            }
        }
    }
}

fn render_items(
    f: &mut Frame,
    block_title: &str,
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
        Paragraph::new("").block(default_block(Some(block_title), focused)),
        rect,
    );
}
