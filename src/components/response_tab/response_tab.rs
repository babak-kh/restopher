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
    state: usize,
}
impl<'a> RespTabs<'a> {
    pub fn new() -> Self {
        let resp_tabs = vec![
            &ResponseOptions::Headers(0, "Headers"),
            &ResponseOptions::Body(1, "Body"),
        ];
        RespTabs {
            resp_tabs,
            state: 0,
        }
    }
    pub fn next(&mut self) {
        if self.state == self.resp_tabs.len() - 1 {
            self.state = 0;
            return;
        }
        self.state += 1;
    }
    pub fn active(&self) -> &ResponseOptions {
        self.resp_tabs[self.state]
    }
    pub fn active_idx(&self) -> usize {
        self.state
    }
}
