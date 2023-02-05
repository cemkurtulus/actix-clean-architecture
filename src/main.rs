mod api;
mod infra;
mod kore;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use api::service_resolver::state_factory;
use api::user_controller::{create_user, get_user};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .data_factory(state_factory)
            .wrap(Logger::default())
            .route("/user", web::post().to(create_user))
            .route("/user/{user_id}", web::get().to(get_user))
        // .service(
        //     web::resource("/user")
        //         .name("user_controller")
        //         .guard(guard::Header("content-type", "application/json"))
        //         .route(web::post().to(create_user))
        //         .route(web::get().to(get_user)),
        // )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
