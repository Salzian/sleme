use serde::{Deserialize, Serialize};

use crate::{DividerBlock, InputBlock, SectionBlock};

pub mod block;
pub mod input_block;
pub mod section_block;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Input(InputBlock),
    Divider(DividerBlock),
    Section(SectionBlock),
}
