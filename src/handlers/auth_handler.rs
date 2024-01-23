use actix_web::{
    cookie::time::Duration as ActixWebDuration, cookie::Cookie, post, web, HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    models::user::UserDto,
    schemas::auth::{LoginUserSchema, RegisterUserSchema},
    services::{auth_service::AuthService, user_services::UserService},
    utils::{password, token},
    AppState,
};

#[post("/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let auth_service = AuthService::new(data.db.clone());

    let user_id = uuid::Uuid::new_v4().to_string();

    if let Err(err) = auth_service.create_user(&user_id, body).await {
        if err.contains("Duplicate entry") {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "fail",
                "message": "User with that email already exists"
            }));
        }

        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status":"error",
            "message": format!("{:?}", err)
        }));
    }

    let user_service = UserService::new(data.db.clone());

    match user_service.get_user_by_id(&user_id).await {
        Ok(user_model) => {
            let user_dto: UserDto = user_model.into();

            let user_response = serde_json::json!({
                "status": "success",
                "data" : serde_json::json!({
                    "note": user_dto
                })
            });

            HttpResponse::Ok().json(user_response)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("{:?}", e)
        })),
    }
}

#[post("/login")]
pub async fn login_user_handler(
    data: web::Data<AppState>,
    body: web::Json<LoginUserSchema>,
) -> impl Responder {
    let _ = body.validate().map_err(|e| {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status":"fail",
            "message": e.to_string(),
        }));
    });

    let user_service = UserService::new(data.db.clone());

    let result = user_service
        .get_user(None, None, Some(&body.email))
        .await
        .map_err(|e| {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status":"fail",
                "message": e.to_string(),
            }))
        });

    let user = match result.unwrap() {
        Some(data) => data,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status": "fail",
                "message": "User not found"
            }))
        }
    };

    let password_matches = password::compare(&body.password, &user.password)
        .map_err(|_| {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "status":"fail",
                "message":"Email or password is wrong",
            }))
        })
        .unwrap();

    if password_matches {
        let token = token::create_token(
            &user.id.to_string(),
            data.config.jwt_secret.as_bytes(),
            data.config.jwt_maxage,
        )
        .map_err(|e| {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status":"fail",
                "message": e.to_string(),
            }))
        })
        .unwrap();

        let cookie = Cookie::build("token", token.to_owned())
            .path("/")
            .max_age(ActixWebDuration::new(60 * &data.config.jwt_maxage, 0))
            .http_only(true)
            .finish();

        let token_response = serde_json::json!({
            "status": "success",
            "data" : serde_json::json!({
                "token": token,
            })
        });

        HttpResponse::Ok()
            .cookie(cookie)
            .json(serde_json::json!(token_response))
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Email or password is wrong"
        }))
    }
}
