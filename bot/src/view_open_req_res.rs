use crate::View;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub(crate) struct ViewOpenRequest {
    pub(crate) trigger_id: String,
    pub(crate) view: View,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ViewOpenResponse {
    pub(crate) ok: bool,
    pub(crate) error: Option<String>,
    view: Option<Value>,
    response_metadata: Option<Value>,
}
