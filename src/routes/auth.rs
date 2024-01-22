use actix_web::web;

use crate::handlers::auth_handler::register;

pub fn auth_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/auth").service(register);

    conf.service(scope);
}
