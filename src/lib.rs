extern crate core;

use slash_command_form_data::SlashCommandFormData;
use worker::{
    console_debug, console_log, event, Context, Date, Env, Request, Response, Result, Router,
};

mod slash_command_form_data;
mod utils;

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
pub async fn main(request: Request, env: Env, _context: Context) -> Result<Response> {
    log_request(&request);
    utils::set_panic_hook();

    let router = Router::new();

    router
        .post_async("/commands/meme", |mut request, _context| async move {
            // Log request query parameters
            let form_data = request.form_data().await.unwrap();
            let parsed_form_data = SlashCommandFormData::from_form_data(form_data);

            console_debug!("{:#?}", parsed_form_data);

            Response::empty()
        })
        .run(request, env)
        .await
}
