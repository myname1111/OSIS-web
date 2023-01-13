use diesel::{data_types::PgInterval, Queryable};
use serde::*;
use std::fmt;

#[derive(Queryable)]
pub struct Blog {
    pub id: i32,
    pub member: Option<i32>,
}

#[derive(Queryable)]
pub struct BlogPost {
    pub id: i32,
    pub blog: Option<i32>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Division {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub date: time::Date,
    pub publicity: Publicity,
    pub profile_id: i32,
    pub program_id: i32,
}

pub enum Publicity {
    Public,
    Private,
}

pub struct EventImage {
    pub event: i32,
    pub image: i32,
}

#[derive(Queryable)]
pub struct Forum {
    pub id: i32,
    pub desc: String,
    pub forum: i32,
    pub member: i32,
}

pub struct Forums {
    pub id: i32,
    pub title: String,
    pub member: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Image {
    pub id: i32,
    pub path: String,
    pub title: String,
}

#[derive(Queryable)]
pub struct Improvement {
    pub id: i32,
    pub body: String,
    pub likes: i32,
    pub dislikes: i32,
    pub event: Option<i32>,
    pub program: Option<i32>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Date {
    year: i32,
    ordinal: u16,
}

impl From<time::Date> for Date {
    fn from(other: time::Date) -> Self {
        Self {
            year: other.year(),
            ordinal: other.ordinal(),
        }
    }
}

impl TryFrom<Date> for time::Date {
    type Error = SqlConversionError;

    fn try_from(other: Date) -> Result<time::Date, SqlConversionError> {
        Self::from_ordinal_date(other.year, other.ordinal).map_err(|err| SqlConversionError {
            type_name: "Date".to_string(),
            conversion_type: SqlConversionType::ToSql,
            message: err.to_string(),
        })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", time::Date::try_from(*self).unwrap())
    }
}

type DivisionId = i32;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Role {
    Member = 0,
    Head = 1,
    VicePresident = 4,
    President = 5,
    Treasurer = 2,
    Secretary = 3,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl TryFrom<String> for Role {
    type Error = SqlConversionError; // TODO: Replace this with custom error handling

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "member" => Ok(Self::Member),
            "head" => Ok(Self::Head),
            "VP" => Ok(Self::VicePresident),
            "president" => Ok(Self::President),
            "treasurer" => Ok(Self::Treasurer),
            "secretary" => Ok(Self::Secretary),
            unintended => Err(SqlConversionError {
                type_name: "Role".to_string(),
                conversion_type: SqlConversionType::FromSql,
                message: format!(
                    "expected either member, head, VP president, exchequer or secretary. got {}",
                    unintended
                ),
            }),
        }
    }
}

impl From<Role> for String {
    fn from(value: Role) -> Self {
        match value {
            Role::Member => "member",
            Role::Head => "head",
            Role::VicePresident => "VP",
            Role::President => "president",
            Role::Treasurer => "treasurer",
            Role::Secretary => "secretary",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub profile: Option<i32>,
    pub role: Role,
    pub bio: String,
    pub joined: Date,
    pub reported: i32,
    pub class: String,
    pub division: Option<DivisionId>,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub username: String,
}

impl TryFrom<MemberSql> for Member {
    type Error = SqlConversionError;

    fn try_from(other: MemberSql) -> Result<Self, SqlConversionError> {
        Ok(Self {
            id: other.id,
            name: other.name,
            profile: other.profile,
            role: other.role.try_into()?,
            bio: other.bio,
            joined: other.joined.into(),
            reported: other.reported,
            class: other.class,
            division: other.division,
            email: other.email,
            password: other.password,
            phone_number: other.phone_number,
            username: other.username,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MemberPreview {
    pub profile: Option<i32>,
    pub role: Role,
    pub name: String,
    pub division: Option<i32>,
    pub id: i32,
}

impl TryFrom<(Option<i32>, String, String, Option<i32>, i32)> for MemberPreview {
    type Error = SqlConversionError;

    fn try_from(
        other: (Option<i32>, String, String, Option<i32>, i32),
    ) -> Result<Self, SqlConversionError> {
        Ok(Self {
            profile: other.0,
            role: other.1.try_into()?,
            name: other.2,
            division: other.3,
            id: other.4,
        })
    }
}

#[derive(Queryable)]
pub struct MemberSql {
    pub id: i32,
    pub name: String,
    pub profile: Option<i32>,
    pub role: String,
    pub bio: String,
    pub joined: time::Date,
    pub reported: i32,
    pub class: String,
    pub division: Option<i32>,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub username: String,
}

impl TryFrom<Member> for MemberSql {
    type Error = SqlConversionError;

    fn try_from(other: Member) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.id,
            name: other.name,
            profile: other.profile,
            role: other.role.into(),
            bio: other.bio,
            joined: other.joined.try_into()?,
            reported: other.reported,
            class: other.class,
            division: other.division,
            email: other.email,
            password: other.password,
            phone_number: other.phone_number,
            username: other.username,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewMember {
    pub name: String,
    pub joined: Date,
    pub class: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatedMember {
    pub name: String,
    pub profile: Option<i32>,
    pub role: String,
    pub bio: String,
    pub class: String,
    pub division: Option<i32>,
}

#[derive(Queryable)]
pub struct President {
    pub id: i32,
    pub start: time::Date,
    pub end: time::Date,
}

#[derive(Queryable)]
pub struct Program {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub made: time::Date,
    pub ended: Option<time::Date>,
    pub publicity: Publicity,
    pub profile: Option<i32>,
    pub president: i32,
    pub status: Status,
}

#[derive(Queryable)]
pub struct ProgramImage {
    pub program: i32,
    pub image: i32,
}

#[derive(Queryable)]
pub struct Reviews {
    pub id: i32,
    pub titile: String,
    pub body: String,
    pub evet: Option<i32>,
    pub program: Option<i32>,
}

#[derive(Queryable)]
pub struct Schedule {
    pub id: i32,
    pub agenda: String,
    pub start: std::time::SystemTime,
    pub length: PgInterval,
    pub location: String,
    pub details: Option<String>,
    pub event: i32,
}

pub enum Status {
    Planned,
    Wip,
    Mvp,
    Complete,
}

#[derive(Queryable)]
pub struct Visit {
    pub id: i32,
    pub time: std::time::SystemTime,
    pub forums: i32,
}

#[derive(Queryable)]
pub struct WorkOnEvent {
    pub event: i32,
    pub division: i32,
}

#[derive(Queryable)]
pub struct WorkOnProgram {
    pub program: i32,
    pub division: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailVer {
    pub code: u32,
    pub email: String,
}

#[derive(Clone, Debug)]
enum SqlConversionType {
    ToSql,
    FromSql,
}

#[derive(Debug, Clone)]
pub struct SqlConversionError {
    type_name: String,
    conversion_type: SqlConversionType,
    message: String,
}

impl fmt::Display for SqlConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let conversion_type = match self.conversion_type {
            SqlConversionType::ToSql => "to",
            SqlConversionType::FromSql => "from",
        };
        write!(
            f,
            "Error coverting type {} {} sql\n{}",
            self.type_name, conversion_type, self.message
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SignInError {
    InccorrectPassword(String),
    MemberWithUsernameDoesntExist(String),
    DatabaseError(String),
}

impl fmt::Display for SignInError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SignInError::InccorrectPassword(member) =>
                    format!("Password for '{}' is incorrect", member),
                SignInError::MemberWithUsernameDoesntExist(member) =>
                    format!("Member with username '{}' doesn't exist", member),
                SignInError::DatabaseError(member) => format!("An error occurred when trying to retrive member with username '{}'\n Note: this is does not mean that '{0}' doesn't exist, there was simply an error in the database", member)
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInResponse(pub Result<Member, SignInError>);
