use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use rust_flutter_application::{
    dtos::{
        global::Response,
        user::{TokenData, UserData, UserDto, UserLoginResponseDto, UserResponseDto},
    },
    handlers,
    models::user::UserRole,
    routes::{auth::auth_config, barang::barang_config, user::user_config},
    schemas::auth::{LoginUserSchema, RegisterUserSchema},
    utils::{config::Config, extractor::RequireAuth},
    AppState,
};
use sqlx::mysql::MySqlPoolOptions;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_checker_handler,
        handlers::auth_handler::logout_user_handler,handlers::auth_handler::login_user_handler,handlers::auth_handler::register_user_handler,
        handlers::user_handler::get_me_handler,
    ),
    components(
        schemas(UserRole,UserDto,UserData,UserResponseDto,RegisterUserSchema,Response,UserLoginResponseDto,LoginUserSchema,TokenData)
    ),
    tags(
        (name = "Authentication Endpoint", description = "Handle authentication"),
        (name = "Users Endpoint", description = "Handle user")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        // ? actix_web = debug | info
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenv().ok();
    env_logger::init();

    // initialize env variable
    let config = Config::init().to_owned();

    // setup pool connection*
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            eprintln!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1)
        }
    };

    // run migration
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("✅ Migrations executed successfully."),
        Err(e) => eprintln!("🔥 Error executing migrations: {}", e),
    };

    let port = config.clone().port;
    println!(
        "{}",
        format!("🚀 Server is running on port http://127.0.0.1:{}", port)
    );

    let openapi = ApiDoc::openapi();

    // setup server
    let server = HttpServer::new(move || {
        // configure cors
        let cors = Cors::default()
            // .allowed_origin("http://localhost:3000")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                config: config.clone(),
            }))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(auth_config)
            .configure(user_config)
            .configure(barang_config)
            .route("/api/healthchecker", web::get().to(health_checker_handler))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", port))?;

    // run server
    server.run().await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/healthchecker",
    tag = "Health Checker Endpoint",
    responses(
        (status = 200, description= "Authenticated User", body = Response),       
    )
)]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "100% healthy";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
