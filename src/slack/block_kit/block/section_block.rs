use serde::{Deserialize, Serialize};

use crate::PlainText;

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionBlock {
    #[serde(rename = "type")]
    type_: String,
    text: PlainText,
}

impl SectionBlock {
    pub fn new(text: PlainText) -> Self {
        SectionBlock {
            type_: "section".to_string(),
            text,
        }
    }
}
