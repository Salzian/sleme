use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DividerBlock {
    #[serde(rename = "type")]
    type_: String,
}

impl DividerBlock {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DividerBlock {
    fn default() -> Self {
        Self {
            type_: "divider".to_string(),
        }
    }
}
