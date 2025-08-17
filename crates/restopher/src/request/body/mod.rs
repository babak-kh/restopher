use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyKind {
    JSON,
    TEXT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub kind: BodyKind,
    pub payload: Option<String>,
}
