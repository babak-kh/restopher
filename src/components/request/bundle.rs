use std::collections::HashMap;

use reqwest::header::{self, HeaderMap};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::{
    components::{self, default_block, to_selected},
    layout,
    utils::app_state::{State, REQUESTS},
};

use super::{
    request::Request,
    response::{handle_response_headers, Response},
    ui,
    view::ReqView,
    HttpVerb, ADDRESS, BODY, HEADERS, PARAMS, VERB,
};

#[derive(Debug)]
pub struct ReqBundle {
    request: super::request::Request,
    pub view: super::view::ReqView,
    response: Option<Response>,
}

impl ReqBundle {
    pub fn new() -> Self {
        ReqBundle {
            request: Request::new(),
            view: ReqView::new(),
            response: None,
        }
    }
    pub fn name(&self) -> String {
        if !self.request.name.is_empty() {
            return self.request.name.clone();
        }
        let mut n = self.request.address.uri.to_string();
        if n.len() >= 10 {
            n = n[0..9].to_string();
        };
        n
    }
    pub fn address(&self) -> String {
        self.request.address.to_string()
    }
    pub fn add_to_header(&mut self) {
        let (key, value, active) = self.view.current_set_header();
        if !key.is_empty() && !value.is_empty() {
            self.request.add_to_header(key, value, active)
        }
    }
    pub fn add_to_param(&mut self) {
        let (key, value, active) = self.view.current_set_param();
        if !key.is_empty() && !value.is_empty() {
            self.request.add_to_param(key, value, active)
        }
    }
    pub fn add_to_address(&mut self, c: char) {
        self.request.address.add(c)
    }
    pub fn remove_from_address(&mut self) {
        self.request.address.pop()
    }
    pub fn verb_up(&mut self) {
        self.request.verb = self.request.verb.up();
    }
    pub fn verb_down(&mut self) {
        self.request.verb = self.request.verb.down();
    }
    pub fn verb(&self) -> HttpVerb {
        self.request.verb.clone()
    }
    pub fn resp_body_formatted(&self) -> String {
        if let Some(resp) = &self.response {
            if let Some(body) = &resp.body {
                let mut ct: Option<(&String, &String)> = None;
                if let Some(ref res) = self.response {
                    if let Some(headers) = &resp.headers {
                        ct = headers
                            .iter()
                            .filter(|item| item.0 == "content-type")
                            .last();
                    };
                    match ct {
                        Some(content_type) => {
                            if content_type.1.contains("application/json") {
                                return serde_json::to_string_pretty(
                                    &serde_json::from_str::<serde_json::Value>(&body.clone())
                                        .unwrap(),
                                )
                                .unwrap()
                                .to_string();
                            } else {
                                return body.clone();
                            }
                        }
                        None => return body.clone(),
                    };
                };
            };
        };
        String::from("")
    }
    pub fn status_code(&self) -> i32 {
        if let Some(sc) = &self.response {
            return sc.status_code;
        }
        0
    }
    pub fn headers(&self) -> Option<Vec<(String, String, bool)>> {
        self.request.headers.clone()
    }
    pub fn headers_len(&self) -> usize {
        if let Some(headers) = &self.request.headers {
            return headers.len();
        }
        0
    }
    pub fn handle_headers(&self) -> HashMap<String, String> {
        self.request.handle_headers()
    }
    pub fn handle_params(&self) -> HashMap<String, String> {
        self.request.handle_params()
    }
    pub fn handle_json_body(&self) -> Result<Option<serde_json::Value>, crate::app::Error> {
        self.request.handle_json_body()
    }
    pub fn params(&self) -> Option<Vec<(String, String, bool)>> {
        self.request.params.clone()
    }
    pub fn params_len(&self) -> usize {
        if let Some(params) = &self.request.params {
            return params.len();
        }
        0
    }
    pub fn set_response_headers(&mut self, h: &HeaderMap) -> Result<(), crate::app::Error> {
        let headers = handle_response_headers(h)?;
        if let Some(ref mut resp) = self.response {
            resp.headers = Some(headers);
        } else {
            self.response = Some(Response {
                headers: Some(headers),
                body: None,
                status_code: 0,
            })
        }
        Ok(())
    }
    pub fn set_response_body(&mut self, b: String) {
        if let Some(resp) = &mut self.response {
            resp.body = Some(b);
        } else {
            self.response = Some(Response {
                headers: None,
                body: Some(b),
                status_code: 0,
            })
        }
    }
    pub fn set_response_status_code(&mut self, sc: i32) {
        if let Some(resp) = &mut self.response {
            resp.status_code = sc;
        } else {
            self.response = Some(Response {
                headers: None,
                body: None,
                status_code: sc,
            })
        }
    }
    pub fn set_response(
        &mut self,
        body: String,
        headers: &HeaderMap,
        sc: i32,
    ) -> Result<(), crate::app::Error> {
        let headers = handle_response_headers(headers)?;
        self.response = Some(Response {
            headers: Some(headers),
            body: Some(body),
            status_code: sc,
        });
        Ok(())
    }
    pub fn add_to_active_header(&mut self, ch: char) {
        self.view.add_to_active_header(ch)
    }
    pub fn delete_selected_header(&mut self) {
        let idx = self.view.header_idx();
        self.request.delete_header(idx)
    }
    pub fn active_deactive_header(&mut self) {
        let idx = self.view.header_idx();
        self.request.active_deactive_header(idx)
    }
    pub fn remove_from_active_header(&mut self) {
        self.view.remove_from_active_header()
    }
    pub fn change_active_header(&mut self) {
        self.view.change_active_header()
    }
    pub fn is_key_active_in_header(&self) -> bool {
        self.view.is_key_active_in_header()
    }

    pub fn delete_selected_param(&mut self) {
        let idx = self.view.param_idx();
        self.request.delete_param(idx)
    }
    pub fn active_deactive_param(&mut self) {
        let idx = self.view.param_idx();
        self.request.active_deactive_param(idx)
    }
    pub fn add_to_active_param(&mut self, ch: char) {
        self.view.add_to_active_header(ch)
    }
    pub fn remove_from_active_param(&mut self) {
        self.view.remove_from_active_param()
    }
    pub fn change_active_param(&mut self) {
        self.view.change_active_param()
    }
    pub fn is_key_active_in_param(&self) -> bool {
        self.view.is_key_active_in_param()
    }
    pub fn response_headers(&self) -> Option<HashMap<String, String>> {
        if let Some(resp) = &self.response {
            return resp.headers.clone();
        }
        None
    }
    pub fn render(&self, f: &mut Frame, r: layout::RequestsLayout, state: &State) {
        let verb_address_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(r.verb_address);
        let v = Paragraph::new(self.request.verb.to_string());
        let mut vb = default_block("verb");
        if state.last().sub() == VERB {
            vb = to_selected(vb);
        }
        let addr = Paragraph::new(self.request.address.to_string());
        let mut ab = default_block("address");
        if state.last().sub() == ADDRESS {
            ab = to_selected(ab);
        }
        f.render_widget(v.block(vb), verb_address_rect[0]);
        f.render_widget(addr.block(ab), verb_address_rect[1]);
        self.handle_request_data(f, r.request_data, state);
        match self.response {
            Some(ref resp) => resp.render(f, r.response_data, state),
            None => f.render_widget(default_block("Response"), r.response_data),
        }
    }
    fn handle_request_data(&self, f: &mut Frame, r: Rect, state: &State) {
        match state.last().sub() {
            BODY => self.render_body(f, state, r),
            PARAMS => self.render_params(f, state, r),
            HEADERS => self.render_headers(f, state, r),
            _ => (),
        }
    }
    fn render_params(&self, f: &mut Frame, state: &State, mut rect: Rect) {
        if self.view.has_new_param() {
            let parm_data = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(rect);
            rect = parm_data[0];
            let mut key_block = components::default_block("Key");
            let mut value_block = components::default_block("Value");
            if self.view.is_key_active_in_param() {
                key_block = to_selected(key_block);
            } else {
                value_block = to_selected(value_block);
            }
            let k = Paragraph::new(self.view.current_set_param().0)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(key_block);
            let v = Paragraph::new(self.view.current_set_param().1)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(value_block);
            let h = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(parm_data[1]);
            f.render_widget(k, h[0]);
            f.render_widget(v, h[1]);
        }
        if let Some(params) = &self.request.params {
            let selected_style = Style::default().add_modifier(Modifier::BOLD);
            let normal_style = Style::default().bg(Color::Blue);
            let rows = params.iter().map(|item| {
                let height = 1;
                let cells = {
                    vec![
                        Cell::from(item.0.clone()),
                        Cell::from(item.1.clone()),
                        Cell::from(format!(
                            "{}",
                            if item.2.clone() { "Active" } else { "Inactive" }
                        )),
                    ]
                };
                Row::new(cells).height(height as u16).bottom_margin(0)
            });
            let t = Table::new(rows, [Constraint::Length(10), Constraint::Length(10)])
                .block(default_block("params"))
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);
            let state = &mut TableState::default();
            state.select(Some(self.view.param_idx()));
            f.render_stateful_widget(t, rect, state);
        } else {
            f.render_widget(default_block("Params"), rect)
        }
    }
    fn render_body(&self, f: &mut Frame, state: &State, rect: Rect) {
        let b = Paragraph::new("bod will be here".to_string())
            .block(default_block("Body"))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(b, rect)
    }
    fn render_headers(&self, f: &mut Frame, state: &State, mut rect: Rect) {
        if self.view.has_new_header() {
            let head_data = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(rect);
            rect = head_data[0];
            let mut key_block = components::default_block("Key");
            let mut value_block = components::default_block("Value");
            if self.view.is_key_active_in_header() {
                key_block = to_selected(key_block);
            } else {
                value_block = to_selected(value_block);
            }
            let k = Paragraph::new(self.view.current_set_header().0)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(key_block);
            let v = Paragraph::new(self.view.current_set_header().1)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(value_block);
            let h = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(head_data[1]);
            f.render_widget(k, h[0]);
            f.render_widget(v, h[1]);
        }
        let mut b = default_block("Headers");
        //if *state.last() == HEADERS {
        //    b = to_selected(b)
        //}
        if let Some(headers) = &self.request.headers {
            let selected_style = Style::default().add_modifier(Modifier::BOLD);
            let normal_style = Style::default().bg(Color::Blue);
            let rows = headers.iter().map(|item| {
                let height = 1;
                let cells = {
                    vec![
                        Cell::from(item.0.clone()),
                        Cell::from(item.1.clone()),
                        Cell::from(format!(
                            "{}",
                            if item.2.clone() { "Active" } else { "Inactive" }
                        )),
                    ]
                };
                Row::new(cells).height(height as u16).bottom_margin(0)
            });
            let t = Table::new(rows, [Constraint::Length(10), Constraint::Length(10)])
                .block(b)
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);
            let state = &mut TableState::default();
            state.select(Some(self.view.header_idx()));
            f.render_stateful_widget(t, rect, state);
        } else {
            f.render_widget(b, rect)
        }
    }
}
