mod valve;
use serde::Serialize;
use tokio::sync::mpsc::Sender;

use crate::gpio_controller::task::TxGpioControllerMessage;

pub async fn start(valve_channel: Sender<TxGpioControllerMessage>) -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .manage(valve_channel)
        .mount("/valve/", valve::get_routes())
        .launch()
        .await?;
    Ok(())
}

#[derive(Debug, Serialize)]

struct Message {
    status: Status,
    message: String,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
enum Status {
    Success,
    Failure,
}
