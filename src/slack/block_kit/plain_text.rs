use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct PlainText {
    #[serde(rename = "type")]
    type_: &'static str,

    pub(crate) text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl PlainText {
    pub(crate) fn new(text: &str) -> Self {
        Self {
            type_: "plain_text",
            text: text.to_string(),
            emoji: None,
            verbatim: None,
        }
    }
}
