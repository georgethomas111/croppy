use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn upload_image_bytes(image_data: web::Bytes) -> impl Responder {
    let upload_dir = "./uploads";
    std::fs::create_dir_all(upload_dir).unwrap();

    // Write the image data to a file
    let filepath = format!("{}/{}", upload_dir, "uploaded_image.png");
    std::fs::write(&filepath, &image_data).unwrap();

    HttpResponse::Ok().body(format!("Image uploaded successfully to {}", filepath))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_image_bytes))
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}

