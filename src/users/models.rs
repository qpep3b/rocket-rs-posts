use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};
use diesel::prelude::*;
use crate::schema::users;
use crate::database;

#[derive(Debug)]
pub enum AuthError {
    LoginError,
    FailedDbConnect,
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: i32,
    pub login: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<User, Self::Error> {
        match request.guard::<database::Db>().await {
            Outcome::Success(conn) => {
                // ToDo: use cookies for auth
                match conn.run(|c| {
                    users::table.select(
                        (users::user_id, users::login)
                    ).find(1).get_result::<User>(c)
                }).await {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Failure((Status::Unauthorized, AuthError::LoginError))
                }
            },
            _ => Outcome::Failure((Status::InternalServerError, AuthError::FailedDbConnect))
        }
    }
}