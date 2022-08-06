use serde::Deserialize;
use serde_json::Value;

use crate::View;

#[derive(Debug, Deserialize)]
pub struct InteractivityRequest {
    #[serde(rename = "type")]
    pub(crate) type_: String,
    //team: String,
    //user: String,
    //view: View,
    //hash: String,
    //response_urls: String,
}

// TODO parse Value correctly
#[derive(Debug, Deserialize)]
struct ResponseUrlObject {
    block_id: Value,
    action_id: Value,
    channel_id: Value,
    response_url: Value,
}
