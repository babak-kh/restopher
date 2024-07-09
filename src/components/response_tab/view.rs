pub enum Focus {
    None,
    Header(usize),
    Body,
}

impl Focus {
    pub fn next(&mut self) {
        match self {
            Focus::None => *self = Focus::Header(0),
            Focus::Header(idx) => *self = Focus::Body,
            Focus::Body => *self = Focus::None,
        }
    }
    pub fn prev(&mut self) {
        match self {
            Focus::None => *self = Focus::Body,
            Focus::Header(idx) => *self = Focus::None,
            Focus::Body => *self = Focus::Header(0),
        }
    }
}
