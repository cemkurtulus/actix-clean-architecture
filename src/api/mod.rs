use std::sync::Arc;

use crate::{
    infra::adapter::user_repository::UserRepository,
    kore::service::{user_service::UserService, UserRepositoryResolver},
};

pub mod model;
pub mod user_controller;

pub async fn user_service_resolver() -> UserService {
    let user_repository = UserRepository::new().await;
    let user_repository_resolver = UserRepositoryResolver {
        repository: Arc::new(user_repository),
    };

    let user_service = UserService {
        user_repository: Arc::new(&user_repository),
    };
    user_service
}
