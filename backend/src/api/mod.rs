/// Blog and blog posts
mod blog;
/// Division and worked on event / programs
mod division;
/// Events, reviews, improvements and schedule
mod event;
/// Forum post and forum plus visits
mod forum;
/// Images plus program and event images
mod images;
/// Member and president
mod member;
/// Program, reviews and improvemens
mod program;

use actix_web::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(member::config));
}