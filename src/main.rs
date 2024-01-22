use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use rust_flutter_application::{utils::config::Config, AppState};
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        // ? actix_web = debug | info
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenv().ok();
    env_logger::init();

    // initialize env variable
    let config = Config::init();

    // setup pool connection
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

    println!(
        "{}",
        format!("🚀 Server is running on port {}", config.port)
    );

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
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(cors)
            .wrap(Logger::default())
            .route("/api/healthchecker", web::get().to(health_checker_handler))
    })
    .bind(("127.0.0.1", config.port))?;

    // run server
    server.run().await?;

    Ok(())
}

async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "100% healthy";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
