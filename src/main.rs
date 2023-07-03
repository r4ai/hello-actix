use std::{io, sync::Mutex};

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/count")]
async fn increment(date: web::Data<AppStateWithCounter>) -> String {
    let mut counter = date.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[get("/show")]
async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("Users: Bob, Alice")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(
                web::scope("/users")
                    .guard(guard::Host("www.rust-lang.org"))
                    .service(show_users),
            )
            .service(hello)
            .service(echo)
            .service(increment)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
