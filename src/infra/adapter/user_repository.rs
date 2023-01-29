use std::env;

use crate::kore::models::user_input_dto::UserInputDto;
use crate::kore::ports::user_repository_port::UserRepositoryPort;
use async_trait::async_trait;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};

use super::entity::user_entity::UserEntity;
use dotenv::dotenv;

pub struct UserRepository {
    collection: Collection<UserEntity>,
}

#[async_trait]
impl UserRepositoryPort for UserRepository {
    async fn init() -> Self {
        dotenv().ok();
        // let ck = env::var("MONGOURI").unwrap();
        let uri = match env::var("MONGOURI") {
            Ok(value) => value.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("ck");
        let collection: Collection<UserEntity> = db.collection("user");

        UserRepository { collection }
    }

    async fn create_user(&self, user_input_dto: UserInputDto) -> Result<InsertOneResult, Error> {
        let doc = UserEntity {
            id: None,
            name: user_input_dto.name,
        };

        let user = self
            .collection
            .insert_one(doc, None)
            .await
            .ok()
            .expect("An Error is occurred while create user");

        Ok(user)
    }
}
