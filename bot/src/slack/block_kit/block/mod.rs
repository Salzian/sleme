use serde::{Deserialize, Serialize};

use crate::{DividerBlock, InputBlock, SectionBlock};

pub(crate) mod block;
pub(crate) mod input_block;
pub(crate) mod section_block;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Block {
    Input(InputBlock),
    Divider(DividerBlock),
    Section(SectionBlock),
}
