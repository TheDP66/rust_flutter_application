use crate::{
    dtos::global::Response, services::pdf_service::PdfService, utils::error::ErrorResponse,
    AppState,
};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{
    body::BodyStream,
    error,
    http::header::{self, ContentDisposition, ContentType, HeaderMap},
    web::{self, Data},
    HttpResponse, Responder,
};
use futures::{StreamExt, TryStreamExt};
use image::io::Reader as ImageReader;
use sanitize_filename::sanitize;
use serde_json::json;
use std::{
    borrow::Borrow,
    fs::File,
    io::{BufReader, BufWriter, Cursor, Read, Write},
    path::Path,
};

// Ex: https://git.sr.ht/~ireas/genpdf-rs/tree/master/examples/demo.rs
// Ex: https://github.com/tokio-rs/axum/discussions/608#discussioncomment-1789020
pub async fn get_genpdf_handler(data: Data<AppState>, mut payload: Multipart) -> impl Responder {
    let response_data = Response {
        status: "success",
        message: "Success".to_string(),
    };

    let pdf_service = PdfService::new(data.db.clone());

    let filename = "test-report2".to_string();

    let buffer = match pdf_service.generate_pdf_service(payload, filename).await {
        Ok(buffer) => buffer,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                status: "failed".to_string(),
                message: e.to_string(),
            });
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(ContentDisposition::attachment("genpdf.pdf"))
        .body(buffer)
}
