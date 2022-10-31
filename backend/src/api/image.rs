use actix_web::{*, web::{Data, Path, Json}};
use crate::db::{image, get_conn_from_pool, DbPool};
use common::Image;

#[get("/{id}")]
async fn get_image(id: Path<i32>, pool: Data<DbPool>) -> Result<Json<Image>, Error> {
    let conn = get_conn_from_pool(pool);

    let image = web::block(move || image::get_image(conn, *id).unwrap()).await?;

    Ok(Json(image))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/image").service(get_image)
    ); 
} 
