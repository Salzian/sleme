use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ViewType {
    #[serde(rename = "modal")]
    Modal,
    #[serde(rename = "home")]
    Home,
}
