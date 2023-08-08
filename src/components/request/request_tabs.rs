#[derive(Debug)]
pub enum RequestOptions<'a> {
    Headers(usize, &'a str),
    Params(usize, &'a str),
    Body(usize, &'a str),
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
    pub selected_idx: usize,
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
}
