use worker::{FormData, FormEntry};

pub mod slash_command_form_data;

pub fn form_data_entry_as_string(form_data: &FormData, key: &str) -> Option<String> {
    form_data.get(key).map(|entry| match entry {
        FormEntry::Field(field) => field,
        FormEntry::File(_) => panic!(
            "Form data field '{}' is a file, when a string is expected.",
            key
        ),
    })
}
