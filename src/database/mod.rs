use rocket_sync_db_pools::diesel;

#[database("posts")]
pub struct Db(diesel::PgConnection);

pub mod users;
