use crate::kore::{
    models::{create_user_output_dto::CreateUserOutputDto, get_user_output_dto::GetUserOutputDto},
    ports::user_repository_port::UserRepositoryPort,
};

use super::super::models::user_input_dto::UserInputDto;
use mongodb::bson::extjson::de::Error;

pub struct UserService {
    pub user_repository: Box<dyn UserRepositoryPort>,
}

impl UserService {
    pub async fn create_user(
        &self,
        user_input_dto: UserInputDto,
    ) -> Result<CreateUserOutputDto, Error> {
        let result_user = self.user_repository.create_user(user_input_dto).await;

        match result_user {
            Ok(user) => Ok(CreateUserOutputDto {
                id: user.inserted_id.as_object_id().unwrap().to_string(),
            }),
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    }

    pub async fn get_user(&self, user_id: String) -> Option<GetUserOutputDto> {
        let user_dto = self.user_repository.get_user(user_id).await;
        user_dto
    }
}

impl UserService {
    pub fn new(user_repository: Box<dyn UserRepositoryPort>) -> UserService {
        UserService { user_repository }
    }
}
