use crate::View;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ViewOpenRequest {
    pub trigger_id: String,
    pub view: View,
}

#[derive(Debug, Deserialize)]
pub struct ViewOpenResponse {
    pub ok: bool,
    pub error: Option<String>,
    view: Option<Value>,
    response_metadata: Option<Value>,
}
