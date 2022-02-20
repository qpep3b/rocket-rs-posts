use rocket::serde::{Deserialize, Serialize};
use crate::schema::posts;

#[derive(Deserialize, Clone)]
pub struct PostCreateData<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct PostCreate<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub author_id: i32,
}

#[derive(Queryable, Serialize, Clone)]
pub struct PostDetail {
    pub post_id: i32,
    pub title: String,
    pub content: String,
    pub author: String,
}

#[derive(Queryable, Serialize, Clone)]
pub struct PostListItem {
    pub post_id: i32,
    pub title: String,
    pub author: String,
}
