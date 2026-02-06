use actix_web::{App, HttpResponse, HttpServer, web};
use tracing_subscriber::EnvFilter;

// In Go, you'd write: func healthCheck(w http.ResponseWriter, r *http.Request) { ... }
// In Rust with Actix, handlers are async functions that return an HttpResponse.
// The framework handles serialization — no need to manually write to a ResponseWriter.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "wedding-planner-api"
    }))
}

// #[actix_web::main] is a macro that sets up the tokio async runtime.
// In Go, you just call http.ListenAndServe() in func main().
// In Rust, main() must be async to use .await, and this macro makes that work.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists (silently skip if not found)
    // Like Go's godotenv.Load() but won't panic if missing
    dotenvy::dotenv().ok();

    // Initialize structured logging
    // RUST_LOG env var controls log level (e.g., RUST_LOG=debug)
    // In Go, you'd configure slog or zerolog similarly
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    tracing::info!("Starting server at http://{}:{}", host, port);

    // HttpServer::new takes a closure (like a factory function) that creates the App.
    // In Go, you'd create a mux/router and register handlers. Same idea here.
    // The `move` keyword transfers ownership of variables into the closure.
    HttpServer::new(move || {
        App::new()
            // .route() maps a URL path + HTTP method to a handler function
            // In Go: mux.HandleFunc("/health", healthCheck)
            .route("/health", web::get().to(health_check))
    })
    // .bind() is like Go's http.ListenAndServe(addr, handler)
    // The ? operator propagates errors — similar to Go's if err != nil { return err }
    .bind((host.as_str(), port))?
    .run()
    .await
}
