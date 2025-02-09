mod request_tab;
mod view;

use crate::{
    keys::keys::is_ctrl_b,
    request::{Body, BodyKind},
    trace_dbg,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
pub use request_tab::{ReqTabs, RequestTabOptions};
use serde_json::{from_str, to_string_pretty};

use crate::{
    components::{default_block, tabs, text_area::TextArea, KV},
    keys::keys::{Event, Key, Modifier},
    request::Request,
};
use view::{Focus, RequestBodyOptions};

pub struct RequestTabComponent<'a> {
    focus: Focus,
    focused: bool,
    req_tabs: ReqTabs<'a>,

    new_header: KV,
    new_param: KV,
    body_view: TextArea,
    temp_body: String,
    request_body_options: RequestBodyOptions,
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
            temp_body: String::from(""),
            request_body_options: RequestBodyOptions::Json,
        }
    }
    pub fn update_inner_focus(&mut self) {
        self.focus = self.focus.next();
        self.req_tabs.next();
    }
    pub fn update(&mut self, req: &mut Request, event: Event) {
        match &self.focus {
            Focus::NewHeaderKV => self.handle_new_header_update(req, event),
            Focus::NewParamKV => self.handle_new_param_update(req, event),
            Focus::Header(_) => self.handle_header_update(req, event),
            Focus::Param(_) => self.handle_param_update(req, event),
            Focus::Body => self.handle_body_update(req, event),
            Focus::None => (),
        }
    }
    fn handle_new_header_update(&mut self, req: &mut Request, event: Event) {
        match event.key {
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
        }
    }
    fn handle_new_param_update(&mut self, req: &mut Request, event: Event) {
        match event.key {
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
        };
    }
    fn handle_header_update(&mut self, req: &mut Request, event: Event) {
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
    fn handle_param_update(&mut self, req: &mut Request, event: Event) {
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
    fn handle_body_update(&mut self, _: &mut Request, event: Event) {
        if is_ctrl_b(&event) {};
        if let Some(modifier) = &event.modifier {
            match modifier {
                Modifier::Control => match event.key {
                    Key::Char('o') => {
                        self.request_body_options.next();
                        return;
                    }
                    Key::Char('b') => {
                        match self.request_body_options {
                            RequestBodyOptions::Json => {
                                trace_dbg!(level:tracing::Level::INFO, &self.temp_body);
                                let (prettied, err) = from_str(&self.temp_body.clone())
                                    .map_or_else(
                                        |e| {
                                            let mut error = String::from("");
                                            error.push_str("Error: ");
                                            error.push_str(e.to_string().as_str());
                                            (self.temp_body.clone(), error)
                                        },
                                        |data: serde_json::Value| {
                                            let content =
                                                serde_json::to_string_pretty(&data).unwrap();
                                            (content, String::from(""))
                                        },
                                    );
                                if !err.is_empty() {
                                    self.body_view.set_error(err);
                                    return;
                                };
                                self.body_view.set_lines(prettied.lines().collect());
                            }
                            RequestBodyOptions::Text => (),
                        };
                        return;
                    }
                    _ => (),
                },
                _ => (),
            }
        };
        match event.key {
            Key::Char(c) => {
                self.temp_body
                    .insert(self.body_view.get_flattened_cursor_position(), c);
                self.body_view.push(c);
            }
            Key::Backspace => {
                let pos = self.body_view.get_flattened_cursor_position();
                if pos != 0 {
                    self.temp_body.remove(pos - 1);
                    self.body_view.pop();
                }
            }
            Key::Enter => {
                self.temp_body
                    .insert(self.body_view.get_flattened_cursor_position(), '\n');
                self.body_view.new_line();
            }
            _ => (),
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
    pub fn draw(&mut self, f: &mut Frame, request: &Request, rect: Rect) {
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
                        let chunks_vertical = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                            .split(chunks[1]);
                        render_items(
                            f,
                            "Headers",
                            &request.headers(),
                            Some(0),
                            self.focused,
                            chunks_vertical[0],
                        );
                        self.new_header.draw(f, chunks_vertical[1]);
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
            RequestTabOptions::Body(_, _) => {
                self.body_view.set_focus(self.is_focused());

                let body_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(12), Constraint::Percentage(88)])
                    .split(chunks[1]);
                f.render_widget(
                    Paragraph::new(format!("Body: {}", self.request_body_options.to_string()))
                        .block(default_block(Some("Body"), self.focused)),
                    body_chunks[0],
                );
                //self.body_view.set_lines(self.temp_body.lines().collect());
                self.body_view.draw(f, body_chunks[1]);
            }
            RequestTabOptions::Params(_, _) => {
                match self.focus {
                    Focus::NewParamKV => {
                        let chunks_vertical = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                            .split(chunks[1]);
                        render_items(
                            f,
                            "Params",
                            &request.params(),
                            Some(0),
                            self.focused,
                            chunks_vertical[0],
                        );
                        self.new_param.draw(f, chunks_vertical[1]);
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
