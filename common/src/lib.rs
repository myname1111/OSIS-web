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

#[derive(Queryable)]
pub struct Division {
    pub id: i32,
    pub name: String
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

#[derive(Queryable)]
pub struct Forums {
    pub id: i32,
    pub title: String,
    pub member: i32,
}

#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub image: Vec<u8>,
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
    type Error = time::error::ComponentRange;

    fn try_from(other: Date) -> Result<time::Date, time::error::ComponentRange> {
        Self::from_ordinal_date(other.year, other.ordinal)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", time::Date::try_from(*self).unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub profile: Option<i32>, // Replace this with Option<Image>
    pub role: String,
    pub bio: String,
    pub joined: Date,
    pub reported: i32,
    pub class: String,
    pub division: Option<i32>,
    // add pub profile Vec<image> or Vec<Vec<u8>>
}

impl From<MemberSql> for Member {
    fn from(other: MemberSql) -> Self {
        Self {
            id: other.id,
            name: other.name,
            profile: other.profile,
            role: other.role,
            bio: other.bio,
            joined: other.joined.into(),
            reported: other.reported,
            class: other.class,
            division: other.division,
        }
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
}

impl TryFrom<Member> for MemberSql {
    type Error = time::error::ComponentRange;

    fn try_from(other: Member) -> Result<Self, Self::Error> {
        Ok(Self {
            id: other.id,
            name: other.name,
            profile: other.profile,
            role: other.role,
            bio: other.bio,
            joined: other.joined.try_into()?,
            reported: other.reported,
            class: other.class,
            division: other.division,
        })
    }
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
