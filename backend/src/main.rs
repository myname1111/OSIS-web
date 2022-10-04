use actix_web::*;
use actix_web_lab::web::spa;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            spa()
                .index_file("../frontend/dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("../frontend/dist")
                .finish(),
        )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
