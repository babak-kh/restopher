use crate::components::request::{RESPONSE_BODY, RESPONSE_HEADERS};
use crate::utils::app_state::Section;

#[derive(Debug)]
pub enum ResponseOptions<'a> {
    Body(usize, &'a str),
    Headers(usize, &'a str),
}
impl<'a> ResponseOptions<'a> {
    pub fn split_at(&self) -> (&str, &str) {
        match self {
            ResponseOptions::Headers(_, name) | ResponseOptions::Body(_, name) => name.split_at(1),
        }
    }
    pub fn to_section(&self) -> Section {
        match self {
            ResponseOptions::Headers(_, _) => RESPONSE_HEADERS,
            ResponseOptions::Body(_, _) => RESPONSE_BODY,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            ResponseOptions::Headers(_, _) => "Headers".to_string(),
            ResponseOptions::Body(_, _) => "Body".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct RespTabs<'a> {
    pub resp_tabs: Vec<&'a ResponseOptions<'a>>,
    selected_idx: usize,
}
impl<'a> RespTabs<'a> {
    pub fn new() -> Self {
        let resp_tabs = vec![
            &ResponseOptions::Headers(0, "Headers"),
            &ResponseOptions::Body(1, "Body"),
        ];
        RespTabs {
            resp_tabs: resp_tabs,
            selected_idx: 0,
        }
    }
    pub fn next(&mut self) {
        if self.selected_idx == self.resp_tabs.len() - 1 {
            self.selected_idx = 0;
            return;
        }
        self.selected_idx += 1;
    }
    pub fn active(&self) -> &ResponseOptions {
        self.resp_tabs[self.selected_idx]
    }
    pub fn active_idx(&self) -> usize {
        self.selected_idx
    }
}
