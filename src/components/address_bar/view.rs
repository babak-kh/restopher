pub enum Focus {
    None,
    Address,
    Verb,
}
impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::None => Focus::Address,
            Focus::Address => Focus::Verb,
            Focus::Verb => Focus::None,
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            Focus::None => Focus::Verb,
            Focus::Address => Focus::None,
            Focus::Verb => Focus::Address,
        }
    }
}
