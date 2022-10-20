use crate::db::{get_conn_from_pool, member::*, DbPool};
use actix_web::*;
use common::Member;
use web::{Data, Json, Path};

#[get("/")]
async fn get_all_members(pool: Data<DbPool>) -> Result<Json<Vec<MemberId>>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let ids = web::block(move || get_all_member_id(&mut conn).unwrap()).await?;

    Ok(Json(ids))
}

#[get("/{id}")]
async fn get_member(path: Path<i32>, pool: Data<DbPool>) -> Result<Json<Member>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let member =
        web::block(move || get_member_by_id(&mut conn, path.into_inner()).unwrap()).await?;

    Ok(Json(member.into()))
}

#[get("/preview")]
async fn get_member_preview(pool: Data<DbPool>) -> Result<Json<Vec<MemberPreview>>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let previews = web::block(move || get_all_member_preview(&mut conn).unwrap()).await?;

    Ok(Json(previews))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/member")
            .service(get_all_members)
            .service(get_member_preview)
            .service(get_member),
    );
}
