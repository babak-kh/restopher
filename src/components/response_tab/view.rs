#[derive(Debug, Clone)]
pub enum Focus {
    Header(usize),
    Body,
}

impl Focus {
    pub fn next(&mut self) -> Focus {
        match self {
            Focus::Header(idx) => Focus::Body,
            Focus::Body => Focus::Header(0),
        }
    }
    pub fn prev(&mut self) -> Focus {
        self.next()
    }
}
