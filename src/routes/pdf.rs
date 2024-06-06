use actix_web::web;

use crate::{
    handlers::pdf_handler::get_genpdf_handler, models::user::UserRole,
    utils::extractor::RequireAuth,
};

pub fn pdf_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/pdf").route("/genpdf", web::get().to(get_genpdf_handler)); // .route("/printpdf", web::get().to(get_printpdf_handler));

    conf.service(scope);
}
