use crate::{
    handlers::user_handler::{get_me_handler, update_photo_handler},
    models::user::UserRole,
    utils::extractor::RequireAuth,
};
use actix_web::web;

pub fn user_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/users")
        .route(
            "/me",
            web::get()
                .to(get_me_handler)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::User,
                    UserRole::Moderator,
                    UserRole::Admin,
                ])),
        )
        .route(
            "/me",
            web::patch()
                .to(update_photo_handler)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::User,
                    UserRole::Moderator,
                    UserRole::Admin,
                ])),
        );

    conf.service(scope);
}
