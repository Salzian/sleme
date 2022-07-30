use worker::{console_log, Context, Date, Env, event, Request, Response, Result, Router};

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
pub async fn main(request: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&request);
    utils::set_panic_hook();

    let router = Router::new();
    router.post_async("/commands/meme", |_, _| async {
        Response::ok("meme")
    }).run(request, env).await
}
