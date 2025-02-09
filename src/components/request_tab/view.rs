pub enum Focus {
    None,
    NewHeaderKV,
    NewParamKV,
    Header(usize),
    Param(usize),
    Body,
}
impl Focus {
    pub fn next(&self) -> Focus {
        match self {
            Focus::None => Focus::Header(0),
            Focus::Header(idx) => Focus::Body,
            Focus::Param(idx) => Focus::Header(0),
            Focus::Body => Focus::Param(0),
            Focus::NewHeaderKV => Focus::NewHeaderKV,
            Focus::NewParamKV => Focus::NewParamKV,
        }
    }
}

pub enum RequestBodyOptions {
    Json,
    Text,
}
impl RequestBodyOptions {
    pub fn to_string(&self) -> String {
        match self {
            RequestBodyOptions::Json => "JSON".to_string(),
            RequestBodyOptions::Text => "Text".to_string(),
        }
    }
    pub fn next(&mut self) {
        *self = match self {
            RequestBodyOptions::Json => RequestBodyOptions::Text,
            RequestBodyOptions::Text => RequestBodyOptions::Json,
        }
    }
}
