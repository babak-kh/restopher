
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
    //   pub fn add_to_header(&mut self, key: String, value: String, active: bool) {
    //       if let Some(ref mut h) = self.headers {
    //           h.push((key, value, active));
    //       } else {
    //           self.headers = Some(vec![(key, value, active)])
    //       }
    //   }
    //   pub fn delete_header(&mut self, idx: usize) {
    //       if let Some(h) = &mut self.headers {
    //           h.remove(idx);
    //           if h.len() == 0 {
    //               self.params = None;
    //           }
    //       }
    //   }
    //   pub fn active_deactive_header(&mut self, idx: usize) {
    //       if let Some(h) = &mut self.headers {
    //           h[idx].2 = !h[idx].2;
    //       }
    //   }

    //   // body
    //   pub fn add_to_req_body(&mut self, c: char) {
    //       match &mut self.body.payload {
    //           Some(s) => s.push(c),
    //           None => self.body.payload = Some(c.to_string()),
    //       }
    //   }
    //   pub fn remove_from_req_body(&mut self) {
    //       match &mut self.body.payload {
    //           Some(s) => {
    //               s.pop();
    //               if s.len() == 0 {
    //                   self.body.payload = None;
    //               }
    //           }
    //           None => (),
    //       }
    //   }
    //   // params
    //   pub fn add_to_param(&mut self, key: String, value: String, active: bool) {
    //       if let Some(ref mut h) = self.params {
    //           h.push((key, value, active));
    //       } else {
    //           self.params = Some(vec![(key, value, active)])
    //       }
    //   }
    //   pub fn delete_param(&mut self, idx: usize) {
    //       if let Some(h) = &mut self.params {
    //           h.remove(idx);
    //           if h.len() == 0 {
    //               self.params = None;
    //           }
    //       }
    //   }
    //   pub fn active_deactive_param(&mut self, idx: usize) {
    //       if let Some(h) = &mut self.params {
    //           h[idx].2 = !h[idx].2;
    //       }
    //   }
}
