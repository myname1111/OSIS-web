use super::schema::member::dsl::*;
use super::DbConnection;
use super::image::ImageId;
use common::{MemberSql, MemberPreview};
use diesel::prelude::*;
use diesel::result::Error;

pub type MemberId = i32;

pub(crate) fn get_member_by_id(
    conn: &mut DbConnection,
    member_id: i32,
) -> Result<MemberSql, Error> {
    member.find(member_id).first(conn)
}

pub(crate) fn get_all_member_id(conn: &mut DbConnection) -> Result<Vec<MemberId>, Error> {
    member.select(id).load::<i32>(conn)
}

pub(crate) fn get_all_member_preview(conn: &mut DbConnection) -> Result<Vec<MemberPreview>, Error> {
    let out = member
        .select((profile_id, role, m_name, division_id))
        .load::<(Option<i32>, String, String, Option<ImageId>)>(conn);

    Ok(out?
        .into_iter()
        .map(|out| out.into())
        .collect::<Vec<MemberPreview>>())
}

// TODO: Implement these things
// fn insert_member(conn: &mut DbConnection, new_member: NewMember);
// fn report_member(conn: &mut DbConnection, member_id: MemberId);
// fn delete_member(conn: &mut DbConnection, member_id: MemberId);
// fn update_member(conn: &mut DbConnection, member_id: MemberId, new_member: Memver);
