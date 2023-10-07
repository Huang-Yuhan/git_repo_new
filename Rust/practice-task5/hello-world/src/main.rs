use actix_web::{web, App, HttpResponse, HttpServer, Responder, error};
use serde::{Deserialize, Serialize};
use actix_web::error::Error;


#[derive(Debug, Deserialize)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Debug, Serialize)]
struct AddResponse {
    res: i32,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn add(req: Result<web::Json<AddRequest>, Error>) -> impl Responder {
    match req {
        Ok(json) => {
            let res = AddResponse {
                res: json.a + json.b,
            };
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            if let Some(json_err) = err.as_error::<error::JsonPayloadError>() {
                match json_err {
                    error::JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body("Unsupported media type"),
                    _ => HttpResponse::BadRequest().finish()
                }
            } else {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/add", web::post().to(add))
    })
    .bind("127.0.0.1:8080")? // 可更改监听端口
    .run()
    .await
}