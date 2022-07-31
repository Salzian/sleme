extern crate core;

use worker::{
    console_debug, console_log, event, Context, Date, Env, Method, Request, Response, Result,
};

use crate::slack::block_kit::plain_text::PlainText;
use slack::block_kit::view::view_type::ViewType;
use slash_command_form_data::SlashCommandFormData;
use view_open_req_res::ViewOpenRequest;

use crate::slack::block_kit::view::View;
use crate::view_open_req_res::ViewOpenResponse;

mod builder;
mod slack;
mod slash_command_form_data;
mod utils;
mod view_open_req_res;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub(crate) async fn main(mut request: Request, env: Env, context: Context) -> Result<Response> {
    log_request(&request);
    utils::set_panic_hook();

    let mut response = Response::error("Internal Server Error", 500);

    let method = request.method();

    let path = request.path();
    let path = path.as_str();

    let form_data = request.form_data().await.unwrap();

    if method == Method::Post && path.starts_with("/commands/") {
        let slash_command_form_data = SlashCommandFormData::from_form_data(form_data);

        match path {
            "/commands/meme" => {
                response = Response::empty();

                context.wait_until(async move {
                    let view = View {
                        type_: ViewType::Modal,
                        title: PlainText::new("Meme Generator 9000"),
                        blocks: &[],
                        close: None,
                        submit: Some(PlainText::new("Send to channel")),
                        private_metadata: None,
                        callback_id: Some("my-callback-id"),
                        clear_on_close: None,
                        notify_on_close: None,
                        external_id: None,
                        submit_disabled: None,
                    };

                    let slack_access_token = env
                        .secret("SLACK_ACCESS_TOKEN")
                        .expect("Missing secret: SLACK_ACCESS_TOKEN")
                        .to_string();

                    let client = reqwest::Client::new();

                    let open_request = ViewOpenRequest {
                        trigger_id: slash_command_form_data.trigger_id,
                        view,
                    };

                    console_debug!("{:#?}", open_request);

                    let response = client
                        .post("https://slack.com/api/views.open")
                        .bearer_auth(slack_access_token)
                        .json(&open_request)
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
                })
            }
            _ => {
                console_debug!("Unknown command: {}", path);
                response = Response::error("Not Found", 404)
            }
        }
    } else if method == Method::Post && path == "/interactivity" {
        response = Response::ok("Not implemented");
    } else {
        console_debug!("Unknown path: {:#?} {:#?}", &method, path);
        response = Response::error("Not Found", 404)
    }

    response
}
