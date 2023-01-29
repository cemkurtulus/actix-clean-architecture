mod api;
mod infra;
mod kore;

use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use api::service_resolver::get_user_service;
use api::user_controller::{create_user, get_user};
use env_logger::Env;
// use kore::ports::user_repository_port::UserRepositoryPort;
// use kore::service::user_service::UserService;
// use kore::service::user_service::UserService;
// use kore::service::user_service::UserService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let user_service = get_user_service().await;
    let user_service_data = web::Data::new(user_service);

    // let data = web::Data<UserService<impl UserRepositoryPort>>

    // .route(web::post().to::<_, (Data<UserService<impl UserRepositoryPort>>, Json<UserInputModel>)>(create_user))

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/user")
                    .name("user_controller")
                    .app_data(user_service_data.clone())
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(create_user))
                    .route(web::get().to(get_user)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
