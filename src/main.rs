use std::{env, io};
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web::spa;
use handlers::{sign_in, sign_up};
mod models;
mod handlers;
mod password_handler;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let bind = ("127.0.0.1", 8081);
    eprintln!("staring server at http://{}:{}", &bind.0, &bind.1);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default().log_target("@"))
            .service(
                web::scope("/api")
                .service(sign_up)
                .service(sign_in)
            )
            .service(
                spa()
                    .index_file("./src/app/build/index.html")
                    .static_resources_mount("/static")
                    .static_resources_location("./src/app/build/static")
                    .finish(),
            )
    })
    .workers(1)
    .bind(bind)?
    .run()
    .await
}

