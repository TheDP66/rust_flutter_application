use actix_web::web;

use crate::{
    handlers::auth_handler::{login_user_handler, logout_user_handler, register_user_handler},
    utils::extractor::RequireAuth,
};

pub fn auth_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .route("/register", web::post().to(register_user_handler))
        .route("/login", web::post().to(login_user_handler))
        .route(
            "/logout",
            web::post().to(logout_user_handler).wrap(RequireAuth),
        );

    conf.service(scope);
}
