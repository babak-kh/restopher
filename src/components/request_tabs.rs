use crate::components::request::{BODY, HEADERS, PARAMS};
use crate::utils::app_state::Section;

#[derive(Debug)]
pub enum RequestOptions<'a> {
    Headers(usize, &'a str),
    Params(usize, &'a str),
    Body(usize, &'a str),
}
impl RequestOptions<'_> {
    pub fn to_section(&self) -> Section {
        match self {
            RequestOptions::Headers(_, _) => HEADERS,
            RequestOptions::Params(_, _) => PARAMS,
            RequestOptions::Body(_, _) => BODY,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            RequestOptions::Headers(_, _) => "Headers".to_string(),
            RequestOptions::Params(_, _) => "Params".to_string(),
            RequestOptions::Body(_, _) => "Body".to_string(),
        }
    }
}
impl<'a> RequestOptions<'a> {
    pub fn split_at(&self) -> (&str, &str) {
        match self {
            RequestOptions::Headers(_, name)
            | RequestOptions::Params(_, name)
            | RequestOptions::Body(_, name) => name.split_at(1),
        }
    }
}
#[derive(Debug)]
pub struct ReqTabs<'a> {
    pub req_tabs: Vec<&'a RequestOptions<'a>>,
    selected_idx: usize,
}
impl<'a> ReqTabs<'a> {
    pub fn new() -> Self {
        let tabs = vec![
            &RequestOptions::Headers(0, "Headers"),
            &RequestOptions::Body(1, "Body"),
            &RequestOptions::Params(2, "Params"),
        ];
        ReqTabs {
            req_tabs: tabs,
            selected_idx: 0,
        }
    }
    pub fn next(&mut self) {
        if self.selected_idx == self.req_tabs.len() - 1 {
            self.selected_idx = 0;
            return;
        }
        self.selected_idx += 1;
    }
    pub fn active(&self) -> &RequestOptions {
        self.req_tabs[self.selected_idx]
    }
    pub fn active_idx(&self) -> usize {
        self.selected_idx
    }
}
