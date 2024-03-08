use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web, HttpRequest, HttpResponse, Responder,
};
use serde_json::json;
use validator::Validate;

use crate::{
    dtos::{
        global::Response,
        token::{RefreshTokenResponseDto, TokenData},
        user::UserLoginResponseDto,
    },
    schemas::auth::{LoginUserSchema, RefreshTokenSchema, RegisterUserSchema},
    services::{auth_service::AuthService, user_services::UserService},
    utils::{extractor::Authenticated, password, token},
    AppState,
};
use redis::AsyncCommands;

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Authentication Endpoint",
    request_body(content = RegisterUserSchema, description = "Credentials to create account", example = json!({"email": "user1@mail.com","name": "User Name","password": "user1","passwordConfirm": "user1"})),
    responses(
        (status=201, description= "Account created successfully", body= UserRegisterResponseDto ),
        (status=400, description= "Validation Errors", body= Response),
        (status=409, description= "User with email already exists", body= Response),
        (status=500, description= "Internal Server Error", body= Response ),
    )
)]
pub async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    match body.validate() {
        Ok(()) => {
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

            let access_token_details = match token::generate_jwt_token(
                user_id.clone(),
                data.config.access_token_max_age,
                data.config.access_token_private_key.to_owned(),
            ) {
                Ok(token_details) => token_details,
                Err(e) => {
                    return HttpResponse::BadGateway().json(
                        serde_json::json!({"status": "fail", "message": format_args!("{}", e)}),
                    );
                }
            };

            let refresh_token_details = match token::generate_jwt_token(
                user_id.clone(),
                data.config.refresh_token_max_age,
                data.config.refresh_token_private_key.to_owned(),
            ) {
                Ok(token_details) => token_details,
                Err(e) => {
                    return HttpResponse::BadGateway().json(
                        serde_json::json!({"status": "fail", "message": format_args!("{}", e)}),
                    );
                }
            };

            let mut redis_client = match data.redis_client.get_async_connection().await {
                Ok(redis_client) => redis_client,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(
                        serde_json::json!({"status": "fail", "message": format_args!("{}", e)}),
                    );
                }
            };

            let access_result: redis::RedisResult<()> = redis_client
                .set_ex(
                    access_token_details.token_uuid.to_string(),
                    user_id.to_string(),
                    (data.config.access_token_max_age * 60) as u64,
                )
                .await;

            if let Err(e) = access_result {
                return HttpResponse::UnprocessableEntity().json(
                    serde_json::json!({"status": "error", "message": format_args!("{}", e)}),
                );
            }

            let refresh_result: redis::RedisResult<()> = redis_client
                .set_ex(
                    refresh_token_details.token_uuid.to_string(),
                    user_id.clone().to_string(),
                    (data.config.refresh_token_max_age * 60) as u64,
                )
                .await;

            if let Err(e) = refresh_result {
                return HttpResponse::UnprocessableEntity().json(
                    serde_json::json!({"status": "error", "message": format_args!("{}", e)}),
                );
            }

            let access_cookie =
                Cookie::build("access_token", access_token_details.token.clone().unwrap())
                    .path("/")
                    .max_age(ActixWebDuration::new(
                        data.config.access_token_max_age * 60,
                        0,
                    ))
                    .http_only(true)
                    .finish();
            let refresh_cookie = Cookie::build(
                "refresh_token",
                refresh_token_details.token.clone().unwrap(),
            )
            .path("/")
            .max_age(ActixWebDuration::new(
                data.config.refresh_token_max_age * 60,
                0,
            ))
            .http_only(true)
            .finish();
            let logged_in_cookie = Cookie::build("logged_in", "true")
                .path("/")
                .max_age(ActixWebDuration::new(
                    data.config.access_token_max_age * 60,
                    0,
                ))
                .http_only(false)
                .finish();

            let token_response = UserLoginResponseDto {
                status: "success".to_string(),
                data: TokenData {
                    access_token: access_token_details.token.unwrap(),
                    refresh_token: refresh_token_details.token,
                    refresh_token_expired: refresh_token_details.expires_in,
                },
            };

            return HttpResponse::Created()
                .cookie(access_cookie)
                .cookie(refresh_cookie)
                .cookie(logged_in_cookie)
                .json(json!(token_response));
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "status":"fail",
            "message": e,
        })),
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Authentication Endpoint",
    request_body(content = LoginUserSchema, description = "Credentials to login", example = json!({"email": "user1@mail.com","password": "user1"})),
    responses(
        (status=201, description= "Login successfully", body= UserLoginResponseDto ),
        (status=400, description= "Validation Errors", body= Response),
        (status=500, description= "User not found!", body= Response),
        (status=401, description= "Email or password is wrong", body= Response),
    )
)]
pub async fn login_user_handler(
    data: web::Data<AppState>,
    body: web::Json<LoginUserSchema>,
) -> impl Responder {
    match body.validate() {
        Ok(()) => {
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
                        let access_token_details = match token::generate_jwt_token(
                            user.id.clone(),
                            data.config.access_token_max_age,
                            data.config.access_token_private_key.to_owned(),
                        ) {
                            Ok(token_details) => token_details,
                            Err(e) => {
                                return HttpResponse::BadGateway()
                                    .json(serde_json::json!({"status": "fail", "message": format_args!("{}", e)}));
                            }
                        };

                        let refresh_token_details = match token::generate_jwt_token(
                            user.id.clone(),
                            data.config.refresh_token_max_age,
                            data.config.refresh_token_private_key.to_owned(),
                        ) {
                            Ok(token_details) => token_details,
                            Err(e) => {
                                return HttpResponse::BadGateway()
                                    .json(serde_json::json!({"status": "fail", "message": format_args!("{}", e)}));
                            }
                        };

                        let mut redis_client = match data.redis_client.get_async_connection().await
                        {
                            Ok(redis_client) => redis_client,
                            Err(e) => {
                                return HttpResponse::InternalServerError()
                                    .json(serde_json::json!({"status": "fail", "message": format_args!("{}", e)}));
                            }
                        };

                        let access_result: redis::RedisResult<()> = redis_client
                            .set_ex(
                                access_token_details.token_uuid.to_string(),
                                user.id.to_string(),
                                (data.config.access_token_max_age * 60) as u64,
                            )
                            .await;

                        if let Err(e) = access_result {
                            return HttpResponse::UnprocessableEntity()
                                .json(serde_json::json!({"status": "error", "message": format_args!("{}", e)}));
                        }

                        let refresh_result: redis::RedisResult<()> = redis_client
                            .set_ex(
                                refresh_token_details.token_uuid.to_string(),
                                user.id.clone().to_string(),
                                (data.config.refresh_token_max_age * 60) as u64,
                            )
                            .await;

                        if let Err(e) = refresh_result {
                            return HttpResponse::UnprocessableEntity()
                                .json(serde_json::json!({"status": "error", "message": format_args!("{}", e)}));
                        }

                        let access_cookie = Cookie::build(
                            "access_token",
                            access_token_details.token.clone().unwrap(),
                        )
                        .path("/")
                        .max_age(ActixWebDuration::new(
                            data.config.access_token_max_age * 60,
                            0,
                        ))
                        .http_only(true)
                        .finish();
                        let refresh_cookie = Cookie::build(
                            "refresh_token",
                            refresh_token_details.token.clone().unwrap(),
                        )
                        .path("/")
                        .max_age(ActixWebDuration::new(
                            data.config.refresh_token_max_age * 60,
                            0,
                        ))
                        .http_only(true)
                        .finish();
                        let logged_in_cookie = Cookie::build("logged_in", "true")
                            .path("/")
                            .max_age(ActixWebDuration::new(
                                data.config.access_token_max_age * 60,
                                0,
                            ))
                            .http_only(false)
                            .finish();

                        let token_response = UserLoginResponseDto {
                            status: "success".to_string(),
                            data: TokenData {
                                access_token: access_token_details.token.unwrap(),
                                refresh_token: refresh_token_details.token,
                                refresh_token_expired: refresh_token_details.expires_in,
                            },
                        };

                        return HttpResponse::Created()
                            .cookie(access_cookie)
                            .cookie(refresh_cookie)
                            .cookie(logged_in_cookie)
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
        Err(e) => HttpResponse::BadRequest().json(json!({
            "status":"fail",
            "message": e,
        })),
    }
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Authentication Endpoint",
    request_body(content = (), description = "Credentials to logout"),
    responses(
        (status=200, description= "Account logout successfully", body= Response ),
    ),
    security(
       ("token" = [])
   )
)]
pub async fn logout_user_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    auth_guard: Authenticated,
) -> impl Responder {
    let message = "Token is invalid or session has expired";

    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    };

    let refresh_token_details = match token::verify_jwt_token(
        data.config.refresh_token_public_key.to_owned(),
        &refresh_token,
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{:?}", e)}));
        }
    };

    let mut redis_client = data.redis_client.get_async_connection().await.unwrap();
    let redis_result: redis::RedisResult<usize> = redis_client
        .del(&[
            refresh_token_details.token_uuid.to_string(),
            auth_guard.access_token_uuid.to_string(),
        ])
        .await;

    if redis_result.is_err() {
        return HttpResponse::UnprocessableEntity().json(
            serde_json::json!({"status": "error", "message": format_args!("{:?}", redis_result.unwrap_err())}),
        );
    }

    let access_cookie = Cookie::build("access_token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();
    let refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();
    let logged_in_cookie = Cookie::build("logged_in", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(logged_in_cookie)
        .json(Response {
            status: "success",
            message: "Account logout successfully".to_string(),
        })
}

pub async fn refresh_token_handler(
    // req: HttpRequest,
    data: web::Data<AppState>,
    body: web::Json<RefreshTokenSchema>,
) -> impl Responder {
    let message = "could not refresh access token";

    // let refresh_token = match req.cookie("refresh_token") {
    //     Some(c) => c.value().to_string(),
    //     None => {
    //         return HttpResponse::Forbidden()
    //             .json(serde_json::json!({"status": "fail", "message": message}))
    //     }
    // };

    let refresh_token = match body.validate() {
        Ok(()) => body.refresh_token.to_owned(),
        Err(message) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    };

    let refresh_token_details = match token::verify_jwt_token(
        data.config.refresh_token_public_key.to_owned(),
        &refresh_token,
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{:?}", e)}));
        }
    };

    let result = data.redis_client.get_async_connection().await;
    let mut redis_client = match result {
        Ok(redis_client) => redis_client,
        Err(e) => {
            return HttpResponse::Forbidden().json(
                serde_json::json!({"status": "fail", "message": format!("Could not connect to Redis: {}", e)}),
            );
        }
    };
    let redis_result: redis::RedisResult<String> = redis_client
        .get(refresh_token_details.token_uuid.to_string())
        .await;

    let user_id = match redis_result {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    };

    let user_service = UserService::new(data.db.clone());

    let query_result = user_service
        .get_user(Some(&user_id), None, None)
        .await
        .unwrap();

    if query_result.is_none() {
        return HttpResponse::Forbidden()
            .json(serde_json::json!({"status": "fail", "message": "the user belonging to this token no logger exists"}));
    }

    let user = query_result.unwrap();

    let access_token_details = match token::generate_jwt_token(
        user.id.clone(),
        data.config.access_token_max_age,
        data.config.access_token_private_key.to_owned(),
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return HttpResponse::BadGateway()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{:?}", e)}));
        }
    };

    let redis_result: redis::RedisResult<()> = redis_client
        .set_ex(
            access_token_details.token_uuid.to_string(),
            user.id.to_string(),
            (data.config.access_token_max_age * 60) as u64,
        )
        .await;

    if redis_result.is_err() {
        return HttpResponse::UnprocessableEntity().json(
            serde_json::json!({"status": "error", "message": format_args!("{:?}", redis_result.unwrap_err())}),
        );
    }

    let access_cookie = Cookie::build("access_token", access_token_details.token.clone().unwrap())
        .path("/")
        .max_age(ActixWebDuration::new(
            data.config.access_token_max_age * 60,
            0,
        ))
        .http_only(true)
        .finish();

    let logged_in_cookie = Cookie::build("logged_in", "true")
        .path("/")
        .max_age(ActixWebDuration::new(
            data.config.access_token_max_age * 60,
            0,
        ))
        .http_only(false)
        .finish();

    let token_response = RefreshTokenResponseDto {
        status: "success".to_string(),
        data: TokenData {
            access_token: access_token_details.token.unwrap(),
            refresh_token: None,
            refresh_token_expired: None,
        },
    };

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(logged_in_cookie)
        .json(serde_json::json!(token_response))
}
