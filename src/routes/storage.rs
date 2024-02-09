use crate::{
    handlers::storage_handler::get_image_handler, models::user::UserRole,
    utils::extractor::RequireAuth,
};
use actix_web::web;

pub fn storage_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/storage").route(
        "/img/{title}",
        web::get()
            .to(get_image_handler)
            .wrap(RequireAuth::allowed_roles(vec![
                UserRole::User,
                UserRole::Moderator,
                UserRole::Admin,
            ])),
    );

    conf.service(scope);
}
