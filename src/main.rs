use actix_multipart::Multipart;
use sanitize_filename;
use actix_web::{dev::Path, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use futures_util::stream::StreamExt as _;
use std::io::Write;
use actix_files::NamedFile;
use std::path::PathBuf;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse> {
    let mut message = String::from("No file uploaded");
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let filename = field.content_disposition().get_filename().unwrap();
        let sanitized_file = sanitize_filename::sanitize(&filename);
        println!("{sanitized_file}");
        let file = format!("- {sanitized_file}");
        message = file;
        let filepath = format!("./uploads/{}", sanitized_file);

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }
        
    }
    Ok(HttpResponse::Ok().json(message))
}

// #[get("/")]
/**
 * A route that sends at home
 */
async fn home() -> HttpResponse {
    // open the uploads folder
    // read the file and create a link to 127.0.0.1::8080/view/{filename}
    // img src={file}
    //done.
    HttpResponse::Ok().json("hello")
}

async fn view_file(req: HttpRequest) -> impl Responder {
    let folder = "./uploads";
    let file_name: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file_path = PathBuf::from(folder).join(file_name);

    if file_path.exists() {
        NamedFile::open(file_path).unwrap().into_response(&req)
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./uploads")?;
    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(save_file))
            .route("/",web::get().to(home))
            .route("/view/{filename}", web::get().to(view_file))
            .service(actix_files::Files::new("/uploads", "./uploads").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}