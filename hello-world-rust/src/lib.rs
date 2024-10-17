use worker::*;

#[event(fetch)]
async fn main(mut req: Request, env: Env, ctx: Context) -> Result<Response> {
    // Check if the request method is either GET or POST
    match req.method() {
        Method::Get => {
            Response::ok("This is a GET request. Please send a POST request to see the request body.")
        }
        Method::Post => {
            // Read the request body
            let body = req.text().await?;
            
            // Respond with the request body
            Response::ok(format!("Received the following body:\n{}", body))
        }
        _ => {
            Response::error("Only GET and POST requests are allowed", 405)
        }
    }
}
