use super::schema::member::dsl::*;
use super::DbConnection;
use common::Member;
use diesel::prelude::*;
use diesel::result::Error;

type MemberId = i32;

pub fn get_member_by_id(conn: &mut DbConnection) -> Result<Member, Error> {
    member.find(0).first(conn)
}

pub fn get_all_member_id(conn: &mut DbConnection) -> Result<Vec<MemberId>, Error> {
    member.select(id).load::<i32>(conn)
}

pub fn get_all_member_role(conn: &mut DbConnection) -> Result<Vec<String>, Error> {
    member.select(role).load::<String>(conn)
}

pub fn get_all_member_profile(conn: &mut DbConnection) -> Result<Vec<Option<i32>>, Error> {
    member.select(profile_id).load::<Option<i32>>(conn)
}
