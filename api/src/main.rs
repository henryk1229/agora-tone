use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn enter_app() -> impl Responder {
    HttpResponse::Ok().body("TODO: serve root response")
}

#[post("/create-garden")]
async fn create_tone_garden() -> impl Responder {
    // TODO - create a listening room with a unique id that can be shared
    // implement subscriptions usinig the id
    HttpResponse::Ok().body("TODO: create listening room, return id")
}

#[get("/{id}")]
async fn enter_garden() -> impl Responder {
    HttpResponse::Ok().body("TODO: connect to shared room")
}

#[post("/{id}/record-sequence")]
async fn record_sequence() -> impl Responder {
    HttpResponse::Ok().body("TODO: process sequence based on user inputs, serve sequence")
}

#[get("/{id}/listen")]
async fn serve_sequence() -> impl Responder {
    HttpResponse::Ok().body("TODO: serve musical sequence to listener")
}

// handle mutation of musical sequence over time
#[post("/{id}/process-sequence")]
async fn process_sequence(_req_body: String) -> impl Responder {
    HttpResponse::Ok().body("TODO: process tick")
}

#[post("/{id}/pause-sequence")]
async fn pause_sequence() -> impl Responder {
    HttpResponse::Ok().body("TODO: allow users to pause sequence")
}

#[post("/{id}/delete-sequence")]
async fn delete_sequence() -> impl Responder {
    HttpResponse::Ok().body("TODO: allow users to delete sequence")
}

// TODO - do we still want this public listeniing route?
// #[get("/tone-garden")]
// async fn serve_sequence() -> impl Responder {
//     HttpResponse::Ok().body("TODO: serve musical sequence")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(enter_app)
            .service(create_tone_garden)
            .service(enter_garden)
            .service(record_sequence)
            .service(serve_sequence)
            .service(process_sequence)
            .service(pause_sequence)
            .service(delete_sequence)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}