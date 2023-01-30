use std::sync::Arc;

use super::ports::user_repository_port::UserRepositoryPort;
pub mod user_service;

pub struct UserRepositoryResolver {
    pub repository: Arc<dyn UserRepositoryPort>,
}
