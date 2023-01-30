use crate::kore::models::user_input_dto::UserInputDto;
use async_trait::async_trait;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult};

#[async_trait]
pub trait UserRepositoryPort {
    async fn create_user(&self, user_input_dto: UserInputDto) -> Result<InsertOneResult, Error>;
}
