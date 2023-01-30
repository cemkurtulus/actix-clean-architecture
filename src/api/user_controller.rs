use super::model::user_input_model::UserInputModel;
use super::model::user_output_model::UserOutputModel;
use super::service_resolver::ServiceState;

use crate::kore::models::user_input_dto::UserInputDto;
use actix_web::{web, HttpResponse, Responder, Result};

pub async fn create_user(
    service_state: web::Data<ServiceState>,
    user_input_model: web::Json<UserInputModel>,
) -> HttpResponse {
    let user_model = UserInputDto {
        name: user_input_model.name.to_string(),
    };

    let result = service_state.user_service.create_user(user_model).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(UserOutputModel { id: user.id }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_user() -> Result<impl Responder> {
    let user = UserOutputModel {
        id: String::from("cem"),
    };

    Ok(web::Json(user))
}
