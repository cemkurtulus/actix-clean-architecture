pub mod model;
pub mod user_controller;

pub mod service_resolver {
    use crate::{
        infra::adapter::user_repository::UserRepository,
        kore::{
            ports::user_repository_port::UserRepositoryPort, service::user_service::UserService,
        },
    };

    pub async fn get_user_service() {
        let user_repository = UserRepository::init().await;

        UserService {
            user_repository_port: user_repository,
        };
    }
}
