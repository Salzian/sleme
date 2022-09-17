use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlainText {
    #[serde(rename = "type")]
    type_: String,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl PlainText {
    pub fn new(text: &str) -> Self {
        Self {
            type_: "plain_text".to_string(),
            text: text.to_string(),
            emoji: None,
            verbatim: None,
        }
    }
}
