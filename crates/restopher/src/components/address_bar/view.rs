pub enum Focus {
    Address,
    Verb,
}
impl Focus {
    pub fn next(&mut self) {
        match self {
            Focus::Verb => *self = Focus::Address,
            Focus::Address => *self = Focus::Verb,
        }
    }
}
