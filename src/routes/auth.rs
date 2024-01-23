use actix_web::web;

use crate::handlers::auth_handler::{login_user_handler, register_user_handler};

pub fn auth_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(register_user_handler)
        .service(login_user_handler);

    conf.service(scope);
}
