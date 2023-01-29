use super::super::models::user_input_dto::UserInputDto;
use super::super::models::user_output_dto::UserOutputDto;
use super::super::ports::user_repository_port::UserRepositoryPort;

// use crate::infra::adapter::user_repository::UserRepository;

use mongodb::bson::extjson::de::Error;

pub struct UserService<T: UserRepositoryPort> {
    user_repository_port: T,
}

impl<T: UserRepositoryPort> UserService<T> {
    // pub async fn init(t: T) -> Self {
    //     let user_repository = t.init().await;

    //     UserService {
    //         user_repository_port: user_repository,
    //     }
    // }

    pub async fn create_user(&self, user_input_dto: UserInputDto) -> Result<UserOutputDto, Error> {
        let result_user = self.user_repository_port.create_user(user_input_dto).await;

        match result_user {
            Ok(user) => Ok(UserOutputDto {
                id: user.inserted_id.to_string(),
            }),
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    }
}
