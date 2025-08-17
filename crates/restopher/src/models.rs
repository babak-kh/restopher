use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SaveOptions {
    Save,
    SaveAs,
}
impl ToString for SaveOptions {
    fn to_string(&self) -> String {
        match self {
            SaveOptions::Save => "Save".to_string(),
            SaveOptions::SaveAs => "Save As".to_string(),
        }
    }
}
