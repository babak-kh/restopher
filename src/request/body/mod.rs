use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyKind {
    JSON,
    TEXT,
}
impl BodyKind {
    pub fn to_string(&self) -> String {
        match self {
            BodyKind::JSON => "JSON".to_string(),
            BodyKind::TEXT => "Text".to_string(),
        }
    }
    pub fn change(&self) -> Self {
        match self {
            BodyKind::JSON => BodyKind::TEXT,
            BodyKind::TEXT => BodyKind::JSON,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub kind: BodyKind,
    pub payload: Option<String>,
}
impl Body {
    pub fn default() -> Self {
        Body {
            kind: BodyKind::JSON,
            payload: None,
        }
    }
    pub fn add(&mut self, c: char) {
        if let Some(payload) = &mut self.payload {
            payload.push(c);
        } else {
            self.payload = Some(c.to_string());
        }
    }
    pub fn pop(&mut self) {
        if let Some(payload) = &mut self.payload {
            payload.pop();
        }
    }
    pub fn to_string(&self) -> String {
        if let Some(payload) = &self.payload {
            payload.clone()
        } else {
            "".to_string()
        }
    }
}
