use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserInputModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub surname: Option<String>,
}
