use actix_web::{HttpResponse, Responder};

use crate::{
    dtos::user::{UserData, UserDto, UserResponseDto},
    utils::extractor::Authenticated,
};

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Users Endpoint",
    request_body(content = (), description = "Return logged in user data"),
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
