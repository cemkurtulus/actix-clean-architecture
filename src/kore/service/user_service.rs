// use std::sync::Arc;

// use crate::kore::ports::user_repository_port::UserRepositoryPort;

use crate::infra::adapter::user_repository::UserRepository;

use super::super::models::user_input_dto::UserInputDto;
use super::super::models::user_output_dto::UserOutputDto;
use mongodb::bson::extjson::de::Error;

pub struct UserService {
    pub user_repository: UserRepository,
}

impl UserService {
    // pub async fn new() -> Self {
    //     UserService {
    //         user_repository: UserRepository::new().await,
    //     }
    // }

    pub async fn create_user(&self, user_input_dto: UserInputDto) -> Result<UserOutputDto, Error> {
        let result_user = self.user_repository.create_user(user_input_dto).await;

        match result_user {
            Ok(user) => Ok(UserOutputDto {
                id: user.inserted_id.to_string(),
            }),
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    }
}
