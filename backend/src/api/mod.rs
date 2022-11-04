/// Blog and blog posts
mod blog;
/// Division and worked on event / programs
mod division;
/// Events, reviews, improvements and schedule
mod event;
/// Forum post and forum plus visits
mod forum;
/// Member and president
mod member;
/// Program, reviews and improvemens
mod program;
/// Images
mod image;

use actix_web::*;

// TODO: make custom error response

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(member::config).configure(image::config).configure(division::cofig));
}
