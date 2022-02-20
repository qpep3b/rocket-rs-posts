use crate::posts::models::{PostDetail, PostListItem, PostCreateData};
use rocket::serde::{json::Json};
use crate::database::Db;
use diesel::prelude::*;
use crate::schema::posts;
use crate::schema::users;
use rocket::http::Status;
use diesel::result::Error;
use crate::users::models::User;


type ResponseResult<T> = std::result::Result<T, Status>;

fn error_status(error: Error) -> rocket::http::Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/?<page>&<page_size>")]
pub async fn posts_list(conn: Db, page: Option<i64>, page_size: Option<i64>) -> ResponseResult<Json<Vec<PostListItem>>> {
    let page_size = page_size.unwrap_or(10);
    let page = page.unwrap_or(1);
    let offset = (page - 1) * page_size;

    let result = conn.run(move |c| {
        posts::table.inner_join(users::table)
            .select((
                posts::post_id,
                posts::title,
                users::login,
            ))
            .limit(page_size)
            .offset(offset)
            .load::<PostListItem>(c)
    }).await;
    
    match result {
        Ok(posts) => {
            if posts.len() == 0 {
                return Result::Err(Status::NotFound)
            }
            Ok(Json(posts))
        },
        Err(err) => Err(error_status(err)),
    }
}

#[get("/<post_id>")]
pub async fn post_detail(post_id: i32, conn: Db) -> Option<Json<PostDetail>> {
    let result = conn.run(move |c| {
        posts::table
            .find(post_id)
            .inner_join(users::table)
            .select((
                posts::post_id,
                posts::title,
                posts::content,
                users::login,
            ))
            .get_result::<PostDetail>(c)
    }).await.ok()?;
    
    Some(Json(result))
}

// ToDo: write insert
#[post("/", data = "<post_data>")]
pub fn post_create(user: User, post_data: Json<PostCreateData<'_> >) -> Json<PostDetail> {

    let response_data = PostDetail {
        post_id: 1,
        author: String::from("some_author"),
        content: String::from(post_data.content),
        title: String::from(post_data.title),
    };
    Json(response_data)
}