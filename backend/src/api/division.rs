use actix_web::*;
use common::Division;
use crate::db::division;
use crate::db::{get_conn_from_pool, DbPool};
use web::{Path, Json, Data};
use division::DivisionId;

#[get("/{id}")]
async fn get_division(pool: Data<DbPool>, id: Path<DivisionId>) -> Result<Json<Division>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let division = web::block(move || division::get_division(&mut conn, *id).unwrap()).await?;

    Ok(Json(division))
}

pub fn cofig(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/division")
            .service(get_division)
    );
}
