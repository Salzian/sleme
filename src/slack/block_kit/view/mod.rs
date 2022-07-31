use serde::Serialize;

use view_type::ViewType;

use crate::slack::block_kit::block::Block;
use crate::slack::block_kit::plain_text::PlainText;

pub mod view_type;

// TODO Cleanup unused

#[derive(Debug, Serialize)]
pub(crate) struct View {
    /// The type of view. Set to modal for modals and home for Home tabs.
    #[serde(rename = "type")]
    pub(crate) type_: ViewType,

    /// The title that appears in the top-left of the modal.
    /// Must be a [PlainText] text element with a max length of 24 characters.
    /// Used for modals.
    pub(crate) title: PlainText,

    /// An array of [Block]s that defines the content of the view.
    /// Max of 100 blocks.
    pub(crate) blocks: &'static [Block],

    /// An optional [PlainText] element that defines the text displayed in the close button
    /// at the bottom-right of the view.
    /// Max length of 24 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) close: Option<PlainText>,

    /// An optional [PlainText] element that defines the text displayed in the submit button
    /// at the bottom-right of the view. `submit` is required when an `input` block is within the
    /// `blocks` array.
    /// Max length of 24 characters.
    /// Used for modals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) submit: Option<PlainText>,

    /// An optional string that will be sent to your app in `view_submission` and `block_actions`
    /// events.
    /// Max length of 3000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) private_metadata: Option<String>,

    /// An identifier to recognize interactions and submissions of this particular view.
    /// Don't use this to store sensitive information (use `private_metadata` instead).
    /// Max length of 255 characters.
    /// Used for modals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) callback_id: Option<&'static str>,

    /// When set to true, clicking on the close button will clear all views in a modal and close it.
    /// Defaults to false.
    /// Used for modals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clear_on_close: Option<bool>,

    /// Indicates whether Slack will send your request URL a view_closed event
    /// when a user clicks the close button.
    /// Defaults to false.
    /// Used for modals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) notify_on_close: Option<bool>,

    /// A custom identifier that must be unique for all views on a per-team basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) external_id: Option<&'static str>,

    /// When set to true, disables the submit button
    /// until the user has completed one or more inputs.
    /// This property is for
    /// [configuration modals](https://api.slack.com/reference/workflows/configuration-view).
    /// Used for modals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) submit_disabled: Option<bool>,
}
