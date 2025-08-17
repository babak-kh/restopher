use crate::components::text_box::TextBox;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub uri: TextBox,
}
