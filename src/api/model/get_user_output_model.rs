use serde::Serialize;

#[derive(Serialize)]
pub struct GetUserOutputModel {
    pub id: String,
    pub name: String,
}
