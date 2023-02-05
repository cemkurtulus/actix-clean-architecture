use super::{
    model::{
        create_user_input_model::CreateUserInputModel,
        create_user_output_model::CreateUserOutputModel, get_user_output_model::GetUserOutputModel,
    },
    service_resolver::ServiceState,
};

use crate::kore::models::user_input_dto::UserInputDto;
use actix_web::{web, HttpResponse};

pub async fn create_user(
    service_state: web::Data<ServiceState>,
    user_input_model: web::Json<CreateUserInputModel>,
) -> HttpResponse {
    let user_model = UserInputDto {
        name: user_input_model.name.to_string(),
    };

    let result = service_state.user_service.create_user(user_model).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(CreateUserOutputModel { id: user.id }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_user(
    service_state: web::Data<ServiceState>,
    user_path: web::Path<String>,
) -> HttpResponse {
    let user_id = user_path.into_inner();
    let user = service_state.user_service.get_user(user_id).await;

    match user {
        Some(user) => HttpResponse::Ok().json(Some(GetUserOutputModel {
            id: user.id,
            name: user.name,
        })),
        _ => HttpResponse::BadRequest().body("User not Found".to_string()),
    }
}
