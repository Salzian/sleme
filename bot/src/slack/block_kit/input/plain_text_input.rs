use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlainTextInput {
    #[serde(rename = "type")]
    type_: String,
    action_id: String,
}

impl PlainTextInput {
    fn new(action_id: String) -> Self {
        PlainTextInput {
            type_: "plain_text_input".to_string(),
            action_id,
        }
    }
}
