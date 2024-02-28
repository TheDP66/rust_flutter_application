use crate::{
    dtos::{
        global::Response,
        user::{UserData, UserDto, UserResponseDto},
    },
    services::user_services::UserService,
    utils::extractor::Authenticated,
    AppState,
};
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Users Endpoint",
    responses(
        (status=201, description= "Logged in user detail", body= UserResponseDto ),
    ),
    security(
       ("token" = [])
   )
)]
pub async fn get_me_handler(user: Authenticated) -> impl Responder {
    let user_dto = UserDto::filter(&user);

    let response_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData { user: user_dto },
    };

    HttpResponse::Ok().json(response_data)
}

pub async fn update_photo_handler(
    user: Authenticated,
    payload: Multipart,
    data: web::Data<AppState>,
) -> impl Responder {
    let photo_id = uuid::Uuid::new_v4().to_string();

    let user_service = UserService::new(data.db.clone());

    let user_dto = UserDto::filter(&user);

    match user_service
        .update_photo(Some(&photo_id), Some(&user_dto.id), payload)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"fail",
                "message": e,
            }))
        }
    }

    return HttpResponse::Ok().json(Response {
        status: "success",
        message: "User updated".to_string(),
    });
}
