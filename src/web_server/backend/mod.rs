mod valve;

use rocket::fs::{relative, FileServer};
use rocket::Build;
use rocket::Rocket;
#[cfg(test)]
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc::Sender;

use crate::gpio_controller::task::TxGpioControllerMessage;

pub fn rocket(valve_channel: Sender<TxGpioControllerMessage>) -> Rocket<Build> {
    rocket::build()
        .manage(valve_channel)
        .mount("/valve/", valve::get_routes())
        .mount(
            "/",
            FileServer::from(relative!("/src/web_server/frontend/build")),
        )
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Eq, PartialEq, Deserialize))]
struct Message {
    status: Status,
    message: String,
    error: Option<String>,
}
impl Message {
    #[cfg(test)]
    pub fn new(status: Status, message: String, error: Option<String>) -> Message {
        Message {
            status,
            message,
            error,
        }
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Eq, PartialEq, Deserialize))]
pub enum Status {
    Success,
    Failure,
}
