use crate::slack::block_kit::divider::DividerBlock;
use crate::slack::block_kit::{plain_text::PlainText, view::View};

use serde::Deserialize;
use slack::block_kit::block::input_block::InputBlock;
use slack::block_kit::block::section_block::SectionBlock;
use slack::interactivity::interactivity_request::InteractivityRequest;
use worker::{console_debug, console_log, event, Context, Env, Method, Request, Response, Result};

mod builder;
mod handle_commands;
mod handle_interactivity;
mod meme_search;
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

    // TODO Evaluate builtin router or wrap this in a custom router

    if path.starts_with("/commands/") {
        console_debug!("Received slash command request");
        handle_commands::handle_commands(&mut request, env, path).await
    } else if path == "/interactivity" {
        console_debug!("Received interactivity request");
        handle_interactivity::handle_interactivity(&mut request).await
    } else if path == "/events" {
        let challenge = request.json::<SlackEventsChallenge>().await?.challenge;
        Response::ok(challenge)
    } else {
        console_debug!("Unknown path: {:#?} {:#?}", &method, path);
        Response::error("Not Found", 404)
    }
}

#[derive(Deserialize)]
struct SlackEventsChallenge {
    token: String,
    challenge: String,
    #[serde(rename = "type")]
    type_: String,
}
