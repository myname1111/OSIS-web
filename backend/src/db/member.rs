use super::schema::member::dsl::*;
use super::DbConnection;
use common::{MemberSql, MemberPreview};
use diesel::prelude::*;
use diesel::result::Error;

pub type MemberId = i32;

mod models {
    use common::NewMember;
    use diesel::prelude::*;
    use crate::db::schema::member;

    pub type MemberPreviewSql = (Option<i32>, String, String, Option<i32>, i32);

    #[derive(Insertable)]
    #[diesel(table_name = member)]
    pub struct NewMemberSql {
        m_name: String,
        joined: time::Date,
        class: String
    }

    impl TryFrom<NewMember> for NewMemberSql {
        type Error = time::error::ComponentRange;

        fn try_from(rhs: NewMember) -> Result<NewMemberSql, time::error::ComponentRange> {
            Ok(NewMemberSql { 
                m_name: rhs.name, 
                joined: rhs.joined.try_into()?,
                class: rhs.class
            })
        }
    }

    #[derive(AsChangeset)]
    #[diesel(table_name = member)]
    pub struct UpdatedMember {
        pub m_name: String,
        pub profile_id: Option<i32>,
        pub role: String,
        pub bio: String,
        pub class: String,
        pub division_id: Option<i32>
    }
}

use models::*;

/// short for diesel result
type DResult<T> = Result<T, Error>;

pub(crate) fn get_member_by_id(
    conn: &mut DbConnection,
    member_id: i32,
) -> Result<MemberSql, Error> {
    member.find(member_id).first(conn)
}

pub(crate) fn get_all_member_id(conn: &mut DbConnection) -> DResult<Vec<MemberId>> {
    member.select(id).load::<i32>(conn)
}

pub(crate) fn get_all_member_preview(conn: &mut DbConnection) -> DResult<Vec<MemberPreview>> {
    let out = member
        .select((profile_id, role, m_name, division_id, id))
        .load::<MemberPreviewSql>(conn);

    Ok(out?
        .into_iter()
        .map(|out| out.into())
        .collect::<Vec<MemberPreview>>())
}

pub(crate) fn insert_member(conn: &mut DbConnection, new_member: NewMemberSql)  -> DResult<usize> {
    diesel::insert_into(member)
        .values(&new_member)
        .execute(conn)
}

pub(crate) fn report_member(conn: &mut DbConnection, member_id: MemberId) -> DResult<usize> {
    diesel::update(member.find(member_id))
        .set(reported.eq(reported + 1))
        .execute(conn)
}

pub(crate) fn delete_member(conn: &mut DbConnection, member_id: MemberId) -> DResult<usize> {
    diesel::delete(member.find(member_id))
        .execute(conn)
}

pub(crate) fn update_member(conn: &mut DbConnection, member_id: MemberId, updated_member: UpdatedMember) -> DResult<usize> {
    diesel::update(member.find(member_id))
        .set(updated_member)
        .execute(conn)
}
