use actix_web::{post, web, HttpResponse, Responder};
use validator::Validate;

use crate::{
    schemas::auth::RegisterUserSchema, services::auth_service::AuthService, utils::password,
    AppState,
};

#[post("/register")]
pub async fn register(
    data: web::Data<AppState>,
    body: web::Json<RegisterUserSchema>,
) -> impl Responder {
    body.validate().map_err(|e| {
        HttpResponse::BadRequest().json(serde_json::json!({
            "status":"error",
            "message": format!("{:?}", e)
        }))
    })?;

    let hashed_password = password::hash(&body.password).map_err(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status":"error",
            "message": format!("{:?}", e)
        }))
    })?;

    let auth_service = AuthService::new(data.db.clone());

    let user_id = uuid::Uuid::new_v4().to_string();

    if let Err(err) = auth_service.register_user(&user_id, body).await {}

    // match result {
    //     Ok(user) => Ok(HttpResponse::Created().json(UserResponseDto {
    //         status: "success".to_string(),
    //         data: UserData {
    //             user: FilterUserDto::filter_user(&user),
    //         },
    //     })),
    //     Err(sqlx::Error::Database(db_err)) => {
    //         if db_err.is_unique_violation() {
    //             Err(HttpResponse::unique_constraint_voilation(
    //                 ErrorMessage::EmailExist,
    //             ))
    //         } else {
    //             Err(HttpResponse::server_error(db_err.to_string()))
    //         }
    //     }
    //     Err(e) => Err(HttpResponse::server_error(e.to_string())),
    // }

    Ok(())
}
