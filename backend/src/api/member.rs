use crate::db::{get_conn_from_pool, member::{self, MemberId}, DbPool};
use actix_web::*;
use common::{Member, MemberPreview, NewMember, UpdatedMember};
use web::{Data, Json, Path};

#[get("/")]
async fn get_all_members(pool: Data<DbPool>) -> Result<Json<Vec<MemberId>>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let ids = web::block(move || member::get_all_member_id(&mut conn).unwrap()).await?;

    Ok(Json(ids))
}

#[get("/{id}")]
async fn get_member(path: Path<i32>, pool: Data<DbPool>) -> Result<Json<Member>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let member =
        web::block(move || member::get_member_by_id(&mut conn, path.into_inner()).unwrap()).await?;

    Ok(Json(member.into()))
}

#[get("/preview")]
async fn get_member_preview(pool: Data<DbPool>) -> Result<Json<Vec<MemberPreview>>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let previews = web::block(move || member::get_all_member_preview(&mut conn).unwrap()).await?;

    Ok(Json(previews))
}

#[post("/")]
async fn new_member(member: Json<NewMember>, pool: Data<DbPool>) -> Result<Json<MemberId>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let member_id = web::block(move || 
            member::insert_member(&mut conn, member.into_inner().try_into().unwrap()).unwrap()
        ).await?.id;

    Ok(Json(member_id))
}

#[put("/{id}/report")]
async fn report_member(path: Path<i32>, pool: Data<DbPool>) -> Result<Json<MemberId>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let member_id = web::block(move || 
            member::report_member(&mut conn, path.into_inner()).unwrap()
        ).await?.id;

    Ok(Json(member_id))
}

#[put("/{id}/delete")]
async fn delete_member(path: Path<i32>, pool: Data<DbPool>) -> Result<Json<MemberId>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let member_id = web::block(move || 
            member::delete_member(&mut conn, path.into_inner()).unwrap()
        ).await?.id;

    Ok(Json(member_id))
}

#[put("/{id}")]
async fn update_member(path: Path<i32>, update_member: Json<UpdatedMember>, pool: Data<DbPool>) -> Result<Json<MemberId>, Error> {
    let mut conn = get_conn_from_pool(pool);
    let member_id = path.into_inner();

    let member_id = web::block(move || 
        member::update_member(&mut conn, member_id, update_member.into_inner().into()).unwrap()
    ).await?.id;

    Ok(Json(member_id))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/member")
            .service(get_all_members)
            .service(get_member_preview)
            .service(get_member),
    );
}
