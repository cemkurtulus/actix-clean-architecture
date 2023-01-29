mod api;
mod infra;
mod kore;

use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use api::user_controller::{create_user, get_user};
use api::user_service_resolver;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let user_service = user_service_resolver().await;
    let user_service_data = web::Data::new(user_service);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/user")
                    .name("user_controller")
                    .app_data(user_service_data)
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(create_user))
                    .route(web::get().to(get_user)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
