use std::env;

use crate::kore::{
    models::{get_user_output_dto::GetUserOutputDto, user_input_dto::UserInputDto},
    ports::user_repository_port::UserRepositoryPort,
};
use async_trait::async_trait;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection,
};

use super::entity::user_entity::UserEntity;
use dotenv::dotenv;

pub struct UserRepository {
    pub collection: Collection<UserEntity>,
}

#[async_trait]
impl UserRepositoryPort for UserRepository {
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

    async fn get_user(&self, user_id: String) -> Option<GetUserOutputDto> {
        let obj_id = mongodb::bson::oid::ObjectId::parse_str(user_id).unwrap();

        let user_result = self
            .collection
            .find_one(Some(doc! {"_id": obj_id}), None)
            .await
            .expect("err");

        match user_result {
            Some(user_entity) => Some(GetUserOutputDto {
                id: user_entity.id.unwrap().to_string(),
                name: user_entity.name,
            }),
            _ => None,
        }
    }
}

impl UserRepository {
    pub async fn new() -> UserRepository {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(value) => value.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("ck");
        let collection: Collection<UserEntity> = db.collection("user");

        UserRepository { collection }
    }
}
