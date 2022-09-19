use crate::slack::block_kit::block::Block;
use crate::slack::block_kit::input::input_element::InputElement;
use crate::slack::block_kit::input::plain_text_input::PlainTextInput;
use crate::slack::form_data::slash_command_form_data::SlashCommandFormData;
use crate::view_open_req_res::ViewOpenRequest;
use crate::view_open_req_res::ViewOpenResponse;
use crate::InputBlock;
use crate::View;
use worker::{console_debug, Env, Request, Response};

pub async fn handle_commands(
    request: &mut Request,
    env: Env,
    path: &str,
) -> worker::Result<Response> {
    let form_data = request.form_data().await.unwrap();
    let slash_command_form_data = SlashCommandFormData::from_form_data(form_data);

    match path {
        "/commands/meme" => handle_meme_command(env, slash_command_form_data).await,
        _ => {
            console_debug!("Unknown command: {}", path);
            Response::error("Not Found", 404)
        }
    }
}

async fn handle_meme_command(
    env: Env,
    slash_command_form_data: SlashCommandFormData,
) -> worker::Result<Response> {
    let blocks = vec![Block::Input(InputBlock::new(
        "Search meme template",
        InputElement::PlainText(PlainTextInput::new("search_meme".to_string())),
        true,
    ))];

    let view = View::simple_modal("Meme Generator 9000", "Send to channel", blocks);

    let slack_oauth_token = env.secret("SLACK_OAUTH_TOKEN")?.to_string();

    let client = reqwest::Client::new();

    let open_request = ViewOpenRequest {
        trigger_id: slash_command_form_data.trigger_id,
        view,
    };

    let json_body = serde_json::to_string(&open_request).unwrap();

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
