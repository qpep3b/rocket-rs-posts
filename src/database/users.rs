use diesel::{self, prelude::*, QueryDsl};
use crate::schema::users;
use crate::users::models::{User};


//pub fn get_user_by_id(conn: diesel::PgConnection, id: u16) -> Option<User> {
 //   users::table.find(1).get_result::<User>(&conn);
//}
