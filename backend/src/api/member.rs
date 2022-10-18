use crate::db::{get_conn_from_pool, member::*, DbPool};
use actix_web::*;
use web::Data;

#[get("/member")]
async fn get_all_members(pool: Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = get_conn_from_pool(pool);

    let ids = web::block(move || get_all_member_id(&mut conn).unwrap()).await?;

    Ok(HttpResponse::Ok().json(ids))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_members);
}
