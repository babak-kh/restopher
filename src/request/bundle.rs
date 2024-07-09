use std::collections::HashMap;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Cell, Paragraph, Row, Table, TableState},
    Frame,
};
use reqwest::header::{self, HeaderMap};

use crate::{
    components::{self, default_block, to_selected},
    layout,
};

use super::{request::Request, view::ReqView};

//#[derive(Debug)]
//pub struct ReqBundle {
//    request: super::request::Request,
//    pub view: super::view::ReqView,
//    response: Option<Response>,
//}
//
//impl ReqBundle {
//    pub fn new() -> Self {
//        ReqBundle {
//            request: Request::new(),
//            view: ReqView::new(),
//            response: None,
//        }
//    }
//    pub fn render(&self, f: &mut Frame, r: layout::RequestsLayout, state: &State) {
//        let verb_address_rect = Layout::default()
//            .direction(Direction::Horizontal)
//            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
//            .split(r.verb_address);
//        let v = Paragraph::new(self.request.verb.to_string());
//        let mut vb = default_block("verb");
//        if state.last().sub() == VERB {
//            vb = to_selected(vb);
//        }
//        let addr = Paragraph::new(self.request.address.to_string());
//        let mut ab = default_block("address");
//        if state.last().sub() == ADDRESS {
//            ab = to_selected(ab);
//        }
//        f.render_widget(v.block(vb), verb_address_rect[0]);
//        f.render_widget(addr.block(ab), verb_address_rect[1]);
//        self.handle_request_data(f, r.request_data, state);
//        match self.response {
//            Some(ref resp) => resp.render(f, r.response_data, state),
//            None => f.render_widget(default_block("Response"), r.response_data),
//        }
//    }
//    fn handle_request_data(&self, f: &mut Frame, r: Rect, state: &State) {
//        match state.last().sub() {
//            BODY => self.render_body(f, state, r),
//            PARAMS => self.render_params(f, state, r),
//            HEADERS => self.render_headers(f, state, r),
//            _ => (),
//        }
//    }
//    fn render_params(&self, f: &mut Frame, state: &State, mut rect: Rect) {
//        if self.view.has_new_param() {
//            let parm_data = Layout::default()
//                .direction(Direction::Vertical)
//                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
//                .split(rect);
//            rect = parm_data[0];
//            let mut key_block = components::default_block("Key");
//            let mut value_block = components::default_block("Value");
//            if self.view.is_key_active_in_param() {
//                key_block = to_selected(key_block);
//            } else {
//                value_block = to_selected(value_block);
//            }
//            let k = Paragraph::new(self.view.current_set_param().0)
//                .wrap(ratatui::widgets::Wrap { trim: true })
//                .block(key_block);
//            let v = Paragraph::new(self.view.current_set_param().1)
//                .wrap(ratatui::widgets::Wrap { trim: true })
//                .block(value_block);
//            let h = Layout::default()
//                .direction(Direction::Horizontal)
//                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
//                .split(parm_data[1]);
//            f.render_widget(k, h[0]);
//            f.render_widget(v, h[1]);
//        }
//        if let Some(params) = &self.request.params {
//            let selected_style = Style::default().add_modifier(Modifier::BOLD);
//            let normal_style = Style::default().bg(Color::Blue);
//            let rows = params.iter().map(|item| {
//                let height = 1;
//                let cells = {
//                    vec![
//                        Cell::from(item.0.clone()),
//                        Cell::from(item.1.clone()),
//                        Cell::from(format!(
//                            "{}",
//                            if item.2.clone() { "Active" } else { "Inactive" }
//                        )),
//                    ]
//                };
//                Row::new(cells).height(height as u16).bottom_margin(0)
//            });
//            let t = Table::new(rows, [Constraint::Length(10), Constraint::Length(10)])
//                .block(default_block("params"))
//                .highlight_style(selected_style)
//                .highlight_symbol(">> ")
//                .widths(&[
//                    Constraint::Percentage(50),
//                    Constraint::Length(30),
//                    Constraint::Min(10),
//                ]);
//            let state = &mut TableState::default();
//            state.select(Some(self.view.param_idx()));
//            f.render_stateful_widget(t, rect, state);
//        } else {
//            f.render_widget(default_block("Params"), rect)
//        }
//    }
//    fn render_body(&self, f: &mut Frame, state: &State, rect: Rect) {
//        let b = Paragraph::new("bod will be here".to_string())
//            .block(default_block("Body"))
//            .wrap(ratatui::widgets::Wrap { trim: true });
//        f.render_widget(b, rect)
//    }
//    fn render_headers(&self, f: &mut Frame, state: &State, mut rect: Rect) {
//        if self.view.has_new_header() {
//            let head_data = Layout::default()
//                .direction(Direction::Vertical)
//                .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
//                .split(rect);
//            rect = head_data[0];
//            let mut key_block = components::default_block("Key");
//            let mut value_block = components::default_block("Value");
//            if self.view.is_key_active_in_header() {
//                key_block = to_selected(key_block);
//            } else {
//                value_block = to_selected(value_block);
//            }
//            let k = Paragraph::new(self.view.current_set_header().0)
//                .wrap(ratatui::widgets::Wrap { trim: true })
//                .block(key_block);
//            let v = Paragraph::new(self.view.current_set_header().1)
//                .wrap(ratatui::widgets::Wrap { trim: true })
//                .block(value_block);
//            let h = Layout::default()
//                .direction(Direction::Horizontal)
//                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
//                .split(head_data[1]);
//            f.render_widget(k, h[0]);
//            f.render_widget(v, h[1]);
//        }
//        let mut b = default_block("Headers");
//        //if *state.last() == HEADERS {
//        //    b = to_selected(b)
//        //}
//        if let Some(headers) = &self.request.headers {
//            let selected_style = Style::default().add_modifier(Modifier::BOLD);
//            let normal_style = Style::default().bg(Color::Blue);
//            let rows = headers.iter().map(|item| {
//                let height = 1;
//                let cells = {
//                    vec![
//                        Cell::from(item.0.clone()),
//                        Cell::from(item.1.clone()),
//                        Cell::from(format!(
//                            "{}",
//                            if item.2.clone() { "Active" } else { "Inactive" }
//                        )),
//                    ]
//                };
//                Row::new(cells).height(height as u16).bottom_margin(0)
//            });
//            let t = Table::new(rows, [Constraint::Length(10), Constraint::Length(10)])
//                .block(b)
//                .highlight_style(selected_style)
//                .highlight_symbol(">> ")
//                .widths(&[
//                    Constraint::Percentage(50),
//                    Constraint::Length(30),
//                    Constraint::Min(10),
//                ]);
//            let state = &mut TableState::default();
//            state.select(Some(self.view.header_idx()));
//            f.render_stateful_widget(t, rect, state);
//        } else {
//            f.render_widget(b, rect)
//        }
//    }
//}
