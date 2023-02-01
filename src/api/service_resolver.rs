use crate::{
    infra::adapter::user_repository::UserRepository, kore::service::user_service::UserService,
};

pub struct ServiceState {
    pub user_service: Box<UserService>,
}

pub async fn state_factory() -> std::io::Result<ServiceState> {
    let user_repository = UserRepository::new().await;

    let user_service = UserService::new(Box::new(user_repository));

    let state = ServiceState {
        user_service: Box::new(user_service),
    };
    Ok(state)
}
