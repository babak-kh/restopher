#[derive(Debug)]
pub enum RequestTabOptions<'a> {
    Headers(usize, &'a str),
    Params(usize, &'a str),
    Body(usize, &'a str),
}

#[derive(Debug, Default)]
struct ReqBody {
    payload: Option<String>,
}

impl<'a> RequestTabOptions<'a> {
    pub fn split_at(&self) -> (&str, &str) {
        match self {
            RequestTabOptions::Headers(_, name)
            | RequestTabOptions::Params(_, name)
            | RequestTabOptions::Body(_, name) => name.split_at(1),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            RequestTabOptions::Headers(_, name)
            | RequestTabOptions::Params(_, name)
            | RequestTabOptions::Body(_, name) => name.to_string(),
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
            &RequestTabOptions::Headers(0, "Headers"),
            &RequestTabOptions::Body(1, "Body"),
            &RequestTabOptions::Params(2, "Params"),
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
    pub fn increment(&mut self) {
        if self.state == self.req_tabs.len() - 1 {
            self.state = 0;
            return;
        }
        self.state += 1;
    }
    pub fn decrement(&mut self) {
        if self.state == 0 {
            self.state = self.req_tabs.len() - 1;
            return;
        }
        self.state -= 1;
    }
}
