use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(default)]
pub(crate) struct Divider {
    #[serde(rename = "type")]
    type_: &'static str,
}

impl Default for Divider {
    fn default() -> Self {
        Self { type_: "divider" }
    }
}
