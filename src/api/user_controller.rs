use super::model::user_input_model::UserInputModel;
use super::model::user_output_model::UserOutputModel;
use crate::kore::service::user_service::UserService;
use crate::kore::{
    models::user_input_dto::UserInputDto, ports::user_repository_port::UserRepositoryPort,
};
use actix_web::{web, HttpResponse, Responder, Result};

pub async fn create_user(
    user_service: web::Data<UserService<impl UserRepositoryPort>>,
    user_input_model: web::Json<UserInputModel>,
) -> HttpResponse {
    let user_model = UserInputDto {
        name: user_input_model.name.to_string(),
    };

    let result = user_service.create_user(user_model).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(UserOutputModel { id: user.id }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

    // Ok(user_input_model.name.to_string())
}

pub async fn get_user() -> Result<impl Responder> {
    let user = UserOutputModel {
        id: String::from("cem"),
    };

    Ok(web::Json(user))
}
