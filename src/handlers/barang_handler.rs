use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;

use crate::{
    dtos::barang::{BarangData, BarangDto, BarangResponseDto, BarangsData, BarangsResponseDto},
    models::barang::BarangModel,
    schemas::barang::{GetBarangSchema, InsertBarangSchema, SyncBarangSchema},
    services::barang_service::BarangService,
    AppState,
};

#[utoipa::path(
    post,
    path = "/api/barang",
    tag = "Barang Endpoint",
    request_body(content = (), description = "Insert new barang", example = json!({"name":"Barang 1", "price": 11000, "stock": 100, "expired_at": "2024-02-05"})),
    responses(
        (status=200, description= "Success insert new barang", body= BarangResponseDto ),
        (status=500, description= "Failed insert barang", body= Response ),
    ),
    security(
       ("token" = [])
   )
)]
pub async fn insert_barang_handler(
    body: web::Json<InsertBarangSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    match body.validate() {
        Ok(()) => {
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
        Err(e) => HttpResponse::BadRequest().json(json!({
            "status":"fail",
            "message": e,
        })),
    }
}

#[utoipa::path(
    get,
    path = "/api/barang",
    tag = "Barang Endpoint",
    params(
        GetBarangSchema,
    ),
    responses(
        (status=200, description= "Success get barang", body= BarangsResponseDto ),
        (status=500, description= "Failed get barang", body= Response ),
    ),
    security(
       ("token" = [])
   )
)]
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

pub async fn sync_barang_handler(
    body: web::Json<SyncBarangSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let barang_service = BarangService::new(data.db.clone());

    for barang in &body.barang {}

    HttpResponse::Ok().json(json!({
        "status": "success".to_string(),
    }))
}
