use actix_web::{web, App, HttpResponse, HttpServer};
mod v1;
pub mod config;

pub struct Context {
    pub client: reqwest::Client,
    pub config: config::AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::var("HOST_ADDR").unwrap_or("0.0.0.0:3000".to_string());
    println!("Started on {}.", addr);

    let ctx = Context {
        client: reqwest::Client::new(),
        config: config::AppConfig::new(),
    };

    let context = web::Data::new(ctx);
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(v1::scope())
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
