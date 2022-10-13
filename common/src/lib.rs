use diesel::pg::data_types::*;
use diesel::prelude::*;
use time::Date;

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
}

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub date: Date,
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

#[derive(Queryable)]
pub struct Member {
    pub id: i32,
    pub profile: i32,
    pub bio: String,
    pub joined: Date,
    pub reportted: i32,
    pub class: String,
    pub division: i32,
    pub head_of: Option<i32>,
}

#[derive(Queryable)]
pub struct President {
    pub id: i32,
    pub start: Date,
    pub end: Date,
}

#[derive(Queryable)]
pub struct Program {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub made: Date,
    pub ended: Option<Date>,
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
