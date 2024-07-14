pub enum Focus {
    None,
    Header(usize),
    Body,
}

impl Focus {
    pub fn next(&mut self) -> Focus {
        match self {
            Focus::None => Focus::Header(0),
            Focus::Header(idx) => Focus::Body,
            Focus::Body => Focus::None,
        }
    }
    pub fn prev(&mut self) -> Focus {
        match self {
            Focus::None => Focus::Body,
            Focus::Header(idx) => Focus::None,
            Focus::Body => Focus::Header(0),
        }
    }
}
