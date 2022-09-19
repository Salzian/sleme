use crate::slack::block_kit::input::plain_text_input::PlainTextInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum InputElement {
    PlainText(PlainTextInput),
}
