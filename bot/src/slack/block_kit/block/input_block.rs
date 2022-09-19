use crate::slack::block_kit::input::input_element::InputElement;
use crate::slack::block_kit::plain_text::PlainTextLabel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct InputBlock {
    #[serde(rename = "type")]
    type_: String,
    element: InputElement,
    label: PlainTextLabel,
    dispatch_action: bool,
}

impl InputBlock {
    pub(crate) fn new(label: &str, input_element: InputElement, dispatch_action: bool) -> Self {
        InputBlock {
            type_: "input".to_string(),
            element: input_element,
            label: PlainTextLabel::new(label),
            dispatch_action,
        }
    }
}
