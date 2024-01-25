use actix_web::web;

use crate::handlers::user_handler::get_me_handler;

pub fn auth_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/users").route("/me", web::get().to(get_me_handler));

    conf.service(scope)
}
