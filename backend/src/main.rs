use actix_files::Files;
use actix_web::*;
use actix_web_lab::web::spa;

mod api;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/data", "data"))
            .configure(api::config)
            .service(
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
