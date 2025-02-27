mod response_tab;
mod view;
//use crate::trace_dbg;

use crate::{
    components::{default_block, tabs, text_area::TextArea},
    keys::keys::Event,
    request::Request,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use response_tab::ResponseOptions;
use view::Focus;

pub struct ResponseTabComponent {
    focus: Focus,
    is_focused: bool,
    body_view: TextArea,
    resp_tabs: response_tab::RespTabs,
}

impl ResponseTabComponent {
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
    pub fn new() -> Self {
        ResponseTabComponent {
            focus: Focus::Header,
            is_focused: false,
            resp_tabs: response_tab::RespTabs::new(),
            body_view: TextArea::new(),
        }
    }
    pub fn update_inner_focus(&mut self) {
        self.focus = self.focus.next();
        self.resp_tabs.next();
    }
    pub fn update(&mut self, _: &mut Request, event: &Event) {
        match &self.focus {
            Focus::Header => (),
            Focus::Body => {
                self.body_view.update(event);
            }
        }
    }
    pub fn lose_focus(&mut self) {
        self.is_focused = false;
    }
    pub fn gain_focus(&mut self) {
        self.is_focused = true;
    }
    pub fn draw(&mut self, f: &mut Frame, req: &Request, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Max(3), Constraint::Fill(1)])
            .split(rect);
        f.render_widget(
            tabs(
                self.resp_tabs
                    .resp_tabs
                    .iter()
                    .map(|t| Span::from(t.to_string()))
                    .collect(),
                Some("Response Data"),
                self.resp_tabs.active_idx(),
                self.is_focused,
            ),
            chunks[0],
        );
        let status_code = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(chunks[1]);
        f.render_widget(
            Paragraph::new("Status")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL)),
            status_code[0],
        );
        f.render_widget(
            Paragraph::new({
                if let Some(resp) = &req.response() {
                    resp.status_code.to_string()
                } else {
                    "_".to_string()
                }
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg({
                        if let Some(resp) = &req.response() {
                            if resp.status_code >= 200 && resp.status_code < 300 {
                                Color::Green
                            } else if resp.status_code >= 300 && resp.status_code < 400 {
                                Color::Yellow
                            } else {
                                Color::Red
                            }
                        } else {
                            Color::Reset
                        }
                    })),
            ),
            status_code[1],
        );
        match self.resp_tabs.active() {
            ResponseOptions::Headers => {
                if let Some(resp) = &req.response() {
                    if let Some(headers) = &resp.headers {
                        f.render_widget(
                            Table::new(
                                headers.iter().map(|(k, v)| {
                                    Row::new(vec![
                                        Cell::from(Span::from(k)),
                                        Cell::from(Span::from(v)),
                                    ])
                                }),
                                vec![Constraint::Percentage(50), Constraint::Percentage(50)],
                            )
                            .block(default_block(None, self.is_focused)),
                            chunks[2],
                        );
                    } else {
                        f.render_widget(
                            Paragraph::new("").block(default_block(None, self.is_focused)),
                            chunks[2],
                        );
                    }
                } else {
                    f.render_widget(
                        Paragraph::new("")
                            .block(default_block(Some("Response Headers"), self.is_focused)),
                        chunks[2],
                    );
                }
            }
            ResponseOptions::Body => {
                let formatted_body = req.resp_body_formatted();
                if !formatted_body.is_empty() {
                    self.body_view.set_focus(self.is_focused);
                    self.body_view.set_lines(formatted_body);
                    self.body_view.draw(f, chunks[2])
                } else {
                    f.render_widget(
                        Paragraph::new("No Body")
                            .block(default_block(Some("Response Body"), self.is_focused)),
                        chunks[2],
                    );
                }
            }
        }
    }
}
