use super::schema::division::dsl::*;
use super::DbConnection;
use common::Division;
use diesel::prelude::*;
use diesel::result::Error;

pub type DivisionId = i32;

pub(crate) fn get_division(conn: &mut DbConnection, div_id: DivisionId) -> Result<Division, Error> {
    division.find(div_id).first(conn)
}
