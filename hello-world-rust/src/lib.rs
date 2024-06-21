use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    if !matches!(req.method(), Method::Get) {
        return Response::error("Please use Post to send an image", 405);
    }

    return Response::ok("Got a non get request");
}
