#[derive(Debug)]
pub enum RequestTabOptions<'a> {
    Headers(&'a str),
    Params(&'a str),
    Body(&'a str),
}

impl<'a> RequestTabOptions<'a> {
    pub fn to_string(&self) -> String {
        match self {
            RequestTabOptions::Headers(name)
            | RequestTabOptions::Params(name)
            | RequestTabOptions::Body(name) => name.to_string(),
        }
    }
}
#[derive(Debug)]
pub struct ReqTabs<'a> {
    pub req_tabs: Vec<&'a RequestTabOptions<'a>>,
    state: usize,
}
impl<'a> ReqTabs<'a> {
    pub fn new() -> Self {
        let tabs = vec![
            &RequestTabOptions::Headers("Headers"),
            &RequestTabOptions::Body("Body"),
            &RequestTabOptions::Params("Params"),
        ];
        ReqTabs {
            req_tabs: tabs,
            state: 0,
        }
    }
    pub fn next(&mut self) {
        if self.state == self.req_tabs.len() - 1 {
            self.state = 0;
            return;
        }
        self.state += 1;
    }
    pub fn active(&self) -> &RequestTabOptions {
        self.req_tabs[self.state]
    }
    pub fn active_idx(&self) -> usize {
        self.state
    }
}
