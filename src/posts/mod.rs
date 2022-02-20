use rocket::fairing::AdHoc;
use crate::posts::routes::*;

pub mod routes;
pub mod models;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("posts_stage", |rocket| async {
        rocket.mount("/posts", routes![posts_list, post_detail, post_create])
    })
}