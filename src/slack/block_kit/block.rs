use crate::slack::block_kit::divider::Divider;
use crate::slack::block_kit::plain_text::PlainText;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) enum Block {
    PlainText(PlainText),
    Divider(Divider),
}
