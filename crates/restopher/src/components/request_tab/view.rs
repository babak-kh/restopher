pub enum Focus {
    NewHeaderKV,
    NewParamKV,
    Header,
    Param,
    Body,
}
impl Focus {
    pub fn next(&self) -> Focus {
        match self {
            Focus::Header => Focus::Body,
            Focus::Param => Focus::Header,
            Focus::Body => Focus::Param,
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
