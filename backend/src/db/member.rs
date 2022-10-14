use super::schema::member::dsl::*;
use common::Member;
use diesel::prelude::*;
use diesel::result::Error; // TODO: replace diesel::result::Error with custom error struct/enum

type MemberId = i32;

pub fn get_member_by_id(conn: &mut PgConnection) -> Result<Member, Error> {
    member.find(0).first(conn)
}

pub fn get_all_member_id(conn: &mut PgConnection) -> Result<Vec<MemberId>, Error> {
    member.select(id).load::<i32>(conn)
}

pub fn get_all_member_role(conn: &mut PgConnection) -> Result<Vec<String>, Error> {
    member.select(role).load::<String>(conn)
}

pub fn get_all_member_profile(conn: &mut PgConnection) -> Result<Vec<i32>, Error> {
    member.select(profile_id).load::<i32>(conn)
}
