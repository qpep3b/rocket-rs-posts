#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{request::Request, response::Response};
use std::sync::{mpsc, Mutex};

mod database;
mod users;
mod posts;
mod schema;
mod stats;

use crate::users::models::User;
use crate::database::Db;
use crate::stats::{StatMessage, StatsAggregator};

pub struct AppState {
    pub sender: Mutex<mpsc::Sender<StatMessage>>
}

#[get("/")]
fn index(user: User) -> Json<User> {
    Json(user)
}

pub struct StatsFairing;

#[rocket::async_trait]
impl Fairing for StatsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Stats appender",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let res_status = res.status().code;
        match req.rocket().state::<AppState>() {
            Some(state) => {
                let sndr = state.sender.lock().unwrap();
                sndr.send(StatMessage::ResponseSuccess).unwrap();
                if res_status < 200 || res_status >= 400 {
                    sndr.send(StatMessage::ResponseFail).unwrap();
                }
            },
            None => {}
        } 
    }
}

#[launch]
fn rocket() -> _ {
    let (sender, receiver) = mpsc::channel::<StatMessage>();

    let app_state = AppState { sender: Mutex::new(sender) };
    let mut stats_agg = StatsAggregator::new(5, receiver);

    std::thread::spawn(move || {
        stats_agg.run();
    });

    let stats_fairing = StatsFairing;

    rocket::build()
        .manage(app_state)
        .attach(Db::fairing())
        .attach(stats_fairing)
        .attach(posts::stage())
        .mount("/", routes![index])
}