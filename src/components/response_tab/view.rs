#[derive(Debug, Clone)]
pub enum Focus {
    Header,
    Body,
}

impl Focus {
    pub fn next(&mut self) -> Focus {
        match self {
            Focus::Header => Focus::Body,
            Focus::Body => Focus::Header,
        }
    }
}
