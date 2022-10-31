use super::schema::member::dsl::*;
use super::DbConnection;
use common::MemberSql;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

pub type MemberId = i32;

#[derive(Serialize, Deserialize)]
pub(crate) struct MemberPreview {
    pub(crate) profile: Option<i32>,
    pub(crate) role: String,
}

impl From<(Option<i32>, String)> for MemberPreview {
    fn from(other: (Option<i32>, String)) -> Self {
        Self {
            profile: other.0,
            role: other.1,
        }
    }
}

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
    let out: std::result::Result<Vec<(Option<i32>, String)>, _> = member
        .select((profile_id, role))
        .load::<(Option<i32>, String)>(conn);

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
