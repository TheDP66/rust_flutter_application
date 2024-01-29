use actix_web::web;

use crate::{
    handlers::barang_handler::{get_barang_handler, insert_barang_handler},
    models::user::UserRole,
    utils::extractor::RequireAuth,
};

pub fn barang_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/barang")
        .route(
            "",
            web::get()
                .to(get_barang_handler)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::User,
                    UserRole::Moderator,
                    UserRole::Admin,
                ])),
        )
        .route(
            "",
            web::post()
                .to(insert_barang_handler)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::User,
                    UserRole::Moderator,
                    UserRole::Admin,
                ])),
        );

    conf.service(scope);
}
