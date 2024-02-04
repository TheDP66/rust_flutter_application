use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    dtos::barang::{BarangData, BarangDto, BarangResponseDto, BarangsData, BarangsResponseDto},
    models::barang::BarangModel,
    schemas::barang::{GetBarangSchema, InsertBarangSchema},
    services::barang_service::BarangService,
    AppState,
};

pub async fn insert_barang_handler(
    body: web::Json<InsertBarangSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let barang_service = BarangService::new(data.db.clone());

    let barang_id = uuid::Uuid::new_v4().to_string();

    if let Err(err) = barang_service.insert_barang(&barang_id, body).await {
        return HttpResponse::InternalServerError().json(json!({
            "status":"error",
            "message": format!("{:?}", err)
        }));
    }

    match barang_service.get_barang_by_id(&barang_id).await {
        Ok(barang) => {
            let response = BarangResponseDto {
                status: "success".to_string(),
                data: BarangData {
                    barang: BarangModel::into(barang),
                },
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", e)
        })),
    }
}

pub async fn get_barang_handler(
    query: web::Query<GetBarangSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_params = query.into_inner();

    let barang_service = BarangService::new(data.db.clone());

    match barang_service
        .get_barang_by_name(query_params.name.as_deref())
        .await
    {
        Ok(barang) => {
            let barangs_response = BarangDto::filter_iter(&barang);

            let response = BarangsResponseDto {
                status: "success".to_string(),
                data: BarangsData {
                    barang: barangs_response,
                },
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", e)
        })),
    }
}
