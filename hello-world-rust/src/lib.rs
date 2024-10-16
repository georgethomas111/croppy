use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    if !matches!(req.method(), Method::Get) {
        return Response::error("Please use Post to send an image", 405);
    }

    let body = req.text();
    let msg = format!("Got a non get request with body size {}", body.len());

    return Response::ok(msg);
}
