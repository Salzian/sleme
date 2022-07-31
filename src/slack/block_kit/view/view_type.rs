use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) enum ViewType {
    #[serde(rename = "modal")]
    Modal,
    #[serde(rename = "home")]
    Home,
}
