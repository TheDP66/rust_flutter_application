use actix_web::{HttpResponse, Responder};

use crate::{
    dtos::user::{UserData, UserDto, UserResponseDto},
    models::user::UserModel,
    utils::extractor::Authenticated,
};

pub async fn get_me_handler(user: Authenticated) -> impl Responder {
    let user_dto: UserDto = UserModel::into(user);

    let response_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData { user: user_dto },
    };

    Ok(HttpResponse::Ok().json(response_data))
}
