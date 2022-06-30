mod services;

use actix_web::{App, HttpServer};

use crate::services::spotify_ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(spotify_ws::routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}