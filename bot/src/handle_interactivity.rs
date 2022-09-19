use crate::meme_search::search_meme;
use crate::{console_debug, InteractivityRequest};
use httpstatus::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use worker::{console_error, FormEntry, Request, Response};

pub async fn handle_interactivity(request: &mut Request) -> worker::Result<Response> {
    let form_data = request.form_data().await.unwrap();

    let payload = match form_data.get("payload").unwrap() {
        FormEntry::Field(field) => field,
        FormEntry::File(_) => return Response::error("Bad Request", 400),
    };
    let payload_json: Value = serde_json::from_str(&payload).unwrap();
    let interaction_type = payload_json["type"].as_str().unwrap();

    match interaction_type {
        "block_actions" => handle_block_actions(payload).await,
        "shortcut" => Response::error("Not Implemented", 501),
        "view_submission" => {
            console_debug!("Not implemented: view_submission");
            status_code_to_response_error(StatusCode::NotImplemented)
        }
        "view_closed" => Response::ok("Not Implemented"),
        _ => {
            console_error!(
                "Unknown interactivity request type: {:#?}",
                &interaction_type
            );
            status_code_to_response_error(StatusCode::BadRequest)
        }
    }
}

async fn handle_block_actions(request: String) -> worker::Result<Response> {
    let block_action_payload: BlockActionPayload = serde_json::from_str(&request).unwrap();
    console_debug!("Number of actions: {}", block_action_payload.actions.len());

    for action in block_action_payload.actions {
        let action_id = action.action_id;
        console_debug!("Action ID: {}", action_id);

        if action_id == "search_meme" {
            let search_term = action.value;
            let meme = match search_meme(&search_term, 1).await {
                Ok(meme) => {
                    console_debug!("meme: {}", meme);
                    meme
                }
                Err(error) => return error.into(),
            };
        }
    }

    status_code_to_response_error(StatusCode::NotImplemented)
}

#[derive(Debug, Deserialize)]
struct BlockActionPayload {
    actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
struct Action {
    block_id: String,
    action_id: String,
    value: String,
}

pub(crate) fn status_code_to_response_error(status_code: StatusCode) -> worker::Result<Response> {
    Response::error(status_code.reason_phrase(), status_code.as_u16())
}
