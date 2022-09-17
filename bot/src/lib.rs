extern crate core;

use serde_json::{from_str, to_string};
use worker::{
    console_debug, console_error, console_log, event, Context, Env, FormEntry, Method, Request,
    Response, Result,
};

use slack::block_kit::block::input_block::InputBlock;
use slack::block_kit::block::section_block::SectionBlock;
use slack::block_kit::block::Block::Divider;
use slack::block_kit::view::view_type::ViewType;
use slack::form_data::slash_command_form_data::SlashCommandFormData;
use slack::interactivity::interactivity_request::InteractivityRequest;
use view_open_req_res::ViewOpenRequest;

use crate::slack::block_kit::block::Block::Section;
use crate::slack::block_kit::divider::DividerBlock;
use crate::{
    slack::block_kit::{plain_text::PlainText, view::View},
    view_open_req_res::ViewOpenResponse,
};

mod builder;
mod slack;
mod utils;
mod view_open_req_res;

#[event(fetch)]
pub async fn main(mut request: Request, env: Env, _context: Context) -> Result<Response> {
    utils::set_panic_hook();

    let method = request.method();

    let path = request.path();
    let path = path.as_str();

    console_log!("[{:?}] {}", method, path);

    if method != Method::Post {
        console_debug!("No route with non-post method: {:#?} {:#?}", &method, path);
        return Response::error("Not Found", 404);
    }

    if path.starts_with("/commands/") {
        console_debug!("Received slash command request");
        handle_commands(&mut request, env, path).await
    } else if path == "/interactivity" {
        console_debug!("Received interactivity request");
        handle_interactivity(&mut request).await
    } else {
        console_debug!("Unknown path: {:#?} {:#?}", &method, path);
        Response::error("Not Found", 404)
    }
}

async fn handle_commands(
    request: &mut Request,
    env: Env,
    path: &str,
) -> Result<Response> {
    let form_data = request.form_data().await.unwrap();
    let slash_command_form_data = SlashCommandFormData::from_form_data(form_data);

    match path {
        "/commands/meme" => {
            let view = View {
                type_: ViewType::Modal,
                title: PlainText::new("Meme Generator 9000"),
                blocks: vec![
                    Section(SectionBlock::new(PlainText::new("This is a test"))),
                    Divider(DividerBlock::default()),
                    Section(SectionBlock::new(PlainText::new("This is a test 2"))),
                ],
                close: None,
                submit: Some(PlainText::new("Send to channel")),
                private_metadata: None,
                callback_id: Some("my-callback-id".to_string()),
                clear_on_close: None,
                notify_on_close: None,
                external_id: None,
                submit_disabled: None,
            };

            let slack_oauth_token = env
                .secret("SLACK_OAUTH_TOKEN")
                .expect("Missing secret: SLACK_OAUTH_TOKEN")
                .to_string();

            let client = reqwest::Client::new();

            let open_request = ViewOpenRequest {
                trigger_id: slash_command_form_data.trigger_id,
                view,
            };

            let json_body = to_string(&open_request).unwrap();

            let response = client
                .post("https://slack.com/api/views.open")
                .bearer_auth(slack_oauth_token)
                .header("Content-Type", "application/json")
                .body(json_body)
                .send()
                .await
                .expect("Failed to send request");

            let view_open_response = response
                .json::<ViewOpenResponse>()
                .await
                .expect("Failed to parse response");

            if !view_open_response.ok {
                panic!("ViewOpenResponse error: {:#?}", view_open_response);
            }

            Response::empty()
        }
        _ => {
            console_debug!("Unknown command: {}", path);
            Response::error("Not Found", 404)
        }
    }
}

async fn handle_interactivity(request: &mut Request) -> Result<Response> {
    let form_data = request.form_data().await.unwrap();

    let payload = match form_data.get("payload").unwrap() {
        FormEntry::Field(field) => field,
        FormEntry::File(_) => return Response::error("Bad Request", 400),
    };

    let interactivity_request: InteractivityRequest =
        from_str(payload.as_str()).expect("Failed to parse payload as InteractivityRequest");

    return match interactivity_request.type_.as_str() {
        "block_actions" => Response::ok("Not Implemented"),
        "shortcut" => Response::ok("Not Implemented"),
        "view_submission" => {
            console_debug!("view_submission: {:#?}", interactivity_request);
            Response::empty()
        }
        "view_closed" => Response::ok("Not Implemented"),
        _ => {
            console_error!(
                "Unknown interactivity request type: {:#?}",
                &interactivity_request
            );
            Response::error("Not Found", 404)
        }
    };
}
