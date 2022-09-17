use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputBlock {
    #[serde(rename = "type")]
    type_: String,
}

impl InputBlock {
    pub fn new() -> Self {
        InputBlock {
            type_: "input".to_string(),
        }
    }
}
