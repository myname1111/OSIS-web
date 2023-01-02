use super::schema::member::dsl::*;
use super::DbConnection;
use common::{MemberPreview, MemberSql};
use diesel::prelude::*;
use diesel::result::Error;

pub type MemberId = i32;

mod models {
    use crate::db::schema::member;
    use common::{NewMember, SqlConversionError, UpdatedMember};
    use diesel::prelude::*;

    pub type MemberPreviewSql = (Option<i32>, String, String, Option<i32>, i32);

    #[derive(Insertable)]
    #[diesel(table_name = member)]
    pub struct NewMemberSql {
        m_name: String,
        joined: time::Date,
        class: String,
    }

    impl TryFrom<NewMember> for NewMemberSql {
        type Error = SqlConversionError;

        fn try_from(rhs: NewMember) -> Result<NewMemberSql, SqlConversionError> {
            Ok(NewMemberSql {
                m_name: rhs.name,
                joined: rhs.joined.try_into()?,
                class: rhs.class,
            })
        }
    }

    #[derive(AsChangeset)]
    #[diesel(table_name = member)]
    pub struct UpdatedMemberSql {
        pub m_name: String,
        pub profile_id: Option<i32>,
        pub role: String,
        pub bio: String,
        pub class: String,
        pub division_id: Option<i32>,
    }

    impl From<UpdatedMember> for UpdatedMemberSql {
        fn from(rhs: UpdatedMember) -> UpdatedMemberSql {
            UpdatedMemberSql {
                m_name: rhs.name.clone(),
                profile_id: rhs.profile,
                role: rhs.role,
                bio: rhs.bio.clone(),
                class: rhs.class.clone(),
                division_id: rhs.division,
            }
        }
    }

    impl From<UpdatedMemberSql> for UpdatedMember {
        fn from(rhs: UpdatedMemberSql) -> UpdatedMember {
            UpdatedMember {
                name: rhs.m_name.clone(),
                profile: rhs.profile_id,
                role: rhs.role,
                bio: rhs.bio.clone(),
                class: rhs.class.clone(),
                division: rhs.division_id,
            }
        }
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
        .map(|out| out.try_into().unwrap())
        .collect::<Vec<MemberPreview>>())
}

pub(crate) fn insert_member(
    conn: &mut DbConnection,
    new_member: NewMemberSql,
) -> DResult<MemberSql> {
    diesel::insert_into(member)
        .values(&new_member)
        .get_result(conn)
}

pub(crate) fn report_member(conn: &mut DbConnection, member_id: MemberId) -> DResult<MemberSql> {
    diesel::update(member.find(member_id))
        .set(reported.eq(reported + 1))
        .get_result(conn)
}

pub(crate) fn delete_member(conn: &mut DbConnection, member_id: MemberId) -> DResult<MemberSql> {
    diesel::delete(member.find(member_id)).get_result(conn)
}

pub(crate) fn update_member(
    conn: &mut DbConnection,
    member_id: MemberId,
    updated_member: UpdatedMemberSql,
) -> DResult<MemberSql> {
    diesel::update(member.find(member_id))
        .set(updated_member)
        .get_result(conn)
}
