#[derive(Debug, Clone)]
pub enum ResponseOptions {
    Body,
    Headers,
}
impl ResponseOptions {
    pub fn to_string(&self) -> String {
        match self {
            ResponseOptions::Headers => "Headers".to_string(),
            ResponseOptions::Body => "Body".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct RespTabs {
    pub resp_tabs: Vec<ResponseOptions>,
    state: usize,
}
impl RespTabs {
    pub fn new() -> Self {
        let resp_tabs = vec![ResponseOptions::Headers, ResponseOptions::Body];
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
    pub fn active(&self) -> ResponseOptions {
        self.resp_tabs[self.state].clone()
    }
    pub fn active_idx(&self) -> usize {
        self.state
    }
}
