use crate::db::{
    get_conn_from_pool,
    member::{self, MemberId},
    DbPool,
};
use actix_web::*;
use common::{EmailVer, Member, MemberPreview, NewMember, UpdatedMember};
use lettre::{message::MultiPart, transport::smtp::authentication::Credentials};
use lettre::{Message, SmtpTransport, Transport};
use std::env;
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

    let member_id = web::block(move || {
        member::insert_member(&mut conn, member.into_inner().try_into().unwrap()).unwrap()
    })
    .await?
    .id;

    Ok(Json(member_id))
}

#[put("/{id}/report")]
async fn report_member(path: Path<i32>, pool: Data<DbPool>) -> Result<Json<MemberId>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let member_id =
        web::block(move || member::report_member(&mut conn, path.into_inner()).unwrap())
            .await?
            .id;

    Ok(Json(member_id))
}

#[put("/{id}/delete")]
async fn delete_member(path: Path<i32>, pool: Data<DbPool>) -> Result<Json<MemberId>, Error> {
    let mut conn = get_conn_from_pool(pool);

    let member_id =
        web::block(move || member::delete_member(&mut conn, path.into_inner()).unwrap())
            .await?
            .id;

    Ok(Json(member_id))
}

#[put("/{id}")]
async fn update_member(
    path: Path<i32>,
    update_member: Json<UpdatedMember>,
    pool: Data<DbPool>,
) -> Result<Json<MemberId>, Error> {
    let mut conn = get_conn_from_pool(pool);
    let member_id = path.into_inner();

    let member_id = web::block(move || {
        member::update_member(&mut conn, member_id, update_member.into_inner().into()).unwrap()
    })
    .await?
    .id;

    Ok(Json(member_id))
}

#[put("/email")]
async fn email_ver(data: Json<EmailVer>) -> Option<String> {
    let email = Message::builder()
        .from(
            "OSIS EPISJH <data2.animationstudiocp@gmail.com>"
                .parse()
                .unwrap(),
        )
        .to(format!("you <{}>", data.email).parse().unwrap())
        .subject("Welcome to osis, please enter the code")
        .multipart(MultiPart::alternative_plain_html(
            format!("Please enter the folowwing code: \n{}", data.code),
            format!(
                "<h2>Please enter the following code</h2>
                    <h1>{}</h1>",
                data.code
            ),
        ))
        .unwrap();

    let creds = Credentials::new(
        env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set"),
        env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set"),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let result = mailer.send(&email);

    result.map(|_| data.email.clone()).ok()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/member")
            .service(email_ver)
            .service(get_all_members)
            .service(get_member_preview)
            .service(get_member)
            .service(new_member)
            .service(report_member)
            .service(delete_member)
            .service(update_member),
    );
}
