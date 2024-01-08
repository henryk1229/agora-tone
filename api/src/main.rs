use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("TODO: serve root response")
}

#[get("/tone-garden")]
async fn serve_sequence() -> impl Responder {
    HttpResponse::Ok().body("TODO: serve musical sequence")
}

#[post("/process_tick")]
async fn echo(_req_body: String) -> impl Responder {
    HttpResponse::Ok().body("TODO: process tick")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Howdy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}