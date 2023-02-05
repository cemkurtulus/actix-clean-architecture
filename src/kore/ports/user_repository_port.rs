use crate::kore::models::{get_user_output_dto::GetUserOutputDto, user_input_dto::UserInputDto};
use async_trait::async_trait;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult};

#[async_trait]
pub trait UserRepositoryPort {
    async fn create_user(&self, user_input_dto: UserInputDto) -> Result<InsertOneResult, Error>;
    async fn get_user(&self, user_id: String) -> Option<GetUserOutputDto>;
}
