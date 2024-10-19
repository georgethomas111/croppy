use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use std::path::PathBuf;
use std::env;

// Handler for the root route, providing a menu of all endpoints
async fn root() -> impl Responder {
    let menu = r#"
<html>
<head>
  <style>
    html {
      font-family: sans-serif;
    }

    body {
      margin: 0;
    }

    header {
      background: purple;
      height: 100px;
    }

    h1 {
      text-align: center;
      color: white;
      line-height: 100px;
      margin: 0;
    }

    article {
      padding: 10px;
      margin: 10px;
      width: 40%;
      height: 20%;
    }

    /* Flexbox styling for the section */
    section {
      display: flex;
      flex-direction: row;
      flex-wrap: wrap;
    }
  </style>
</head>

<body>
  <section>
    <article>
      <h2>API Endpoints</h2>
        <li><b>POST /upload</b> - Upload an image (binary data)</li>
        <li><b>GET /image</b> - Get the uploaded image</li>
    </article>

	<article>
	  <h2>Upload an Image</h2>
	  <form action="/upload" method="post" enctype="multipart/form-data">
		<input type="file" name="image" accept="image/png,image/jpeg" required>
		<button type="submit">Upload</button>
	  </form>
	</article>
  </section>
</body>
</html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(menu)
}

// Handler for uploading image bytes
async fn upload_image_bytes(image_data: web::Bytes) -> impl Responder {
    let upload_dir = "./uploads";
    std::fs::create_dir_all(upload_dir).unwrap();

    // Write the image data to a file
    let filepath = format!("{}/{}", upload_dir, "uploaded_image.png");
    std::fs::write(&filepath, &image_data).unwrap();

    HttpResponse::Ok().body(format!("Image uploaded successfully to {}", filepath))
}


// Handler to retrieve the uploaded image
async fn get_uploaded_image(req: HttpRequest) -> impl Responder {
    let image_path = PathBuf::from("./uploads/uploaded_image.png");

    if image_path.exists() {
        NamedFile::open(image_path)
            .unwrap()
            .use_last_modified(true)
            .into_response(&req)
    } else {
        HttpResponse::NotFound().body("Image not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the port from command-line arguments or use default 80
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).unwrap_or(&"80".to_string()).clone();

    println!("Listening on port: {}", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root)) // Root route providing menu
            .route("/upload", web::post().to(upload_image_bytes)) // Upload route
            .route("/image", web::get().to(get_uploaded_image)) // Get uploaded image route
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))? // Parse the port as u16
    .run()
    .await
}
