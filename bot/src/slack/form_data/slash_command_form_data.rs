use crate::slack::form_data;
use worker::FormData;

#[derive(Debug)]
pub struct SlashCommandFormData {
    pub token: String,
    team_id: String,
    team_domain: String,
    enterprise_id: Option<String>,
    enterprise_name: Option<String>,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    text: String,
    response_url: String,
    pub trigger_id: String,
    api_app_id: String,
}

impl SlashCommandFormData {
    pub fn from_form_data(form_data: FormData) -> Self {
        Self {
            token: form_data::form_data_entry_as_string(&form_data, "token")
                .expect("Request form data is missing required field: token"),
            team_id: form_data::form_data_entry_as_string(&form_data, "team_id")
                .expect("Request form data is missing required field: team_id"),
            team_domain: form_data::form_data_entry_as_string(&form_data, "team_domain")
                .expect("Request form data is missing required field: team_domain"),
            enterprise_id: form_data::form_data_entry_as_string(&form_data, "enterprise_id"),
            enterprise_name: form_data::form_data_entry_as_string(&form_data, "enterprise_name"),
            channel_id: form_data::form_data_entry_as_string(&form_data, "channel_id")
                .expect("Request form data is missing required field: channel_id"),
            channel_name: form_data::form_data_entry_as_string(&form_data, "channel_name")
                .expect("Request form data is missing required field: channel_name"),
            user_id: form_data::form_data_entry_as_string(&form_data, "user_id")
                .expect("Request form data is missing required field: user_id"),
            user_name: form_data::form_data_entry_as_string(&form_data, "user_name")
                .expect("Request form data is missing required field: user_name"),
            command: form_data::form_data_entry_as_string(&form_data, "command")
                .expect("Request form data is missing required field: command"),
            text: form_data::form_data_entry_as_string(&form_data, "text")
                .expect("Request form data is missing required field: text"),
            response_url: form_data::form_data_entry_as_string(&form_data, "response_url")
                .expect("Request form data is missing required field: response_url"),
            trigger_id: form_data::form_data_entry_as_string(&form_data, "trigger_id")
                .expect("Request form data is missing required field: trigger_id"),
            api_app_id: form_data::form_data_entry_as_string(&form_data, "api_app_id")
                .expect("Request form data is missing required field: api_app_id"),
        }
    }
}
