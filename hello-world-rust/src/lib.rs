use worker::*;

#[event(fetch)]
async fn main(mut req: Request, env: Env, ctx: Context) -> Result<Response> {
    if !matches!(req.method(), Method::Get) {
        return Response::error("Please use Post to send an image", 405);
    }

    let txt = req.text().await?;
    let msg = format!("Got a non get request with body size {}", txt);

    return Response::ok(msg);
}
