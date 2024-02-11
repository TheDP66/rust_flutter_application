use crate::utils::config::Config;
use actix_files::NamedFile;
use actix_web::{error, web, Responder};
use std::path::PathBuf;

pub async fn get_image_handler(title: web::Path<String>) -> impl Responder {
    let config = Config::init().to_owned();

    let file_path: PathBuf = format!("{}{}", config.storage_dir, title).parse().unwrap();

    match NamedFile::open(file_path) {
        Ok(file) => Ok(file),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}
