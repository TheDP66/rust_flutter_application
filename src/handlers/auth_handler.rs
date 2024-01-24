use actix_web::{
    cookie::time::Duration as ActixWebDuration, cookie::Cookie, web, HttpResponse, Responder,
};
use serde_json::json;
use validator::Validate;

use crate::{
    schemas::auth::{LoginUserSchema, RegisterUserSchema},
    services::{auth_service::AuthService, user_services::UserService},
    utils::{password, token},
    AppState,
};

pub async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let auth_service = AuthService::new(data.db.clone());

    let user_id = uuid::Uuid::new_v4().to_string();

    if let Err(err) = auth_service.create_user(&user_id, body).await {
        if err.contains("Duplicate entry") {
            return HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "User with that email already exists"
            }));
        }

        return HttpResponse::InternalServerError().json(json!({
            "status":"error",
            "message": format!("{:?}", err)
        }));
    }

    let user_service = UserService::new(data.db.clone());

    match user_service.get_user(Some(&user_id), None, None).await {
        Ok(user) => {
            let user_response = json!({
                "status": "success",
                "data" : json!({
                    "user": user
                })
            });

            HttpResponse::Ok().json(user_response)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", e)
        })),
    }
}

pub async fn login_user_handler(
    data: web::Data<AppState>,
    body: web::Json<LoginUserSchema>,
) -> impl Responder {
    let _ = body.validate().map_err(|e| {
        return HttpResponse::BadRequest().json(json!({
            "status":"fail",
            "message": e.to_string(),
        }));
    });

    let user_service = UserService::new(data.db.clone());

    match user_service.get_user(None, None, Some(&body.email)).await {
        Ok(result) => {
            let user = match result {
                Some(user) => user,
                None => {
                    return HttpResponse::InternalServerError().json(json!({
                        "status":"fail",
                        "message":"User not found!",
                    }))
                }
            };

            let password_matches = password::compare(&body.password, &user.password)
                .map_err(|_| {
                    HttpResponse::Unauthorized().json(json!({
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
                    HttpResponse::InternalServerError().json(json!({
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

                let token_response = json!({
                    "status": "success",
                    "data" : json!({
                        "token": token,
                    })
                });

                return HttpResponse::Ok()
                    .cookie(cookie)
                    .json(json!(token_response));
            } else {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Email or password is wrong"
                }));
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"fail",
                "message": e.to_string(),
            }))
        }
    }
}

pub async fn logout_user_handler() -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(json!({
        "status":"success"
    }))
}
