use std::collections::HashMap;

use reqwest::header::HeaderMap;
use tui::{layout::{Rect, Layout, Direction, Constraint}, backend::Backend, Frame};

use crate::utils::app_state::State;

use super::{
    request::Request,
    response::{handle_response_headers, Response},
    view::ReqView,
    HttpVerb, ui,
};

#[derive(Debug)]
pub struct ReqBundle {
    request: super::request::Request,
    view: super::view::ReqView,
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
    pub fn params(&self) -> Option<Vec<(String, String, bool)>> {
        self.request.params.clone()
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
    pub fn add_to_active_param(&mut self, ch: char) {
        self.view.add_to_active_header(ch)
    }
    pub fn remove_from_active_header(&mut self) {
        self.view.remove_from_active_header()
    }
    pub fn remove_from_active_param(&mut self) {
        self.view.remove_from_active_param()
    }
    pub fn change_active_header(&mut self) {
        self.view.change_active_header()
    }
    pub fn change_active_param(&mut self) {
        self.view.change_active_param()
    }
    pub fn is_key_active_in_header(&self) -> bool {
        self.view.is_key_active_in_header()
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
    pub fn delete_selected_header(&mut self) {
        let idx = self.view.header_idx();
        self.request.delete_header(idx)
    }
    pub fn delete_selected_param(&mut self) {
        let idx = self.view.param_idx();
        self.request.delete_param(idx)
    }
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, state: &State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(6),  // verb + address
                Constraint::Percentage(6),  // req tabs
                Constraint::Percentage(41), // req headers/body/params
                Constraint::Percentage(6),  // resp headers/body tabs
                Constraint::Percentage(35), // response
            ])
            .split(rect);
        let verb_address_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(chunks[0]);

        f.render_widget(ui::verb(self.request.verb.clone()), verb_address_rect[0]);
        f.render_widget(ui::address(self.request.address.to_string()), verb_address_rect[1]);
    }
}
