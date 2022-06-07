use rocket::{get, routes, serde::json::Json, State};
use tokio::sync::mpsc::Sender;

use crate::gpio_controller::task::TxGpioControllerMessage;

use super::{Message, Status};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![open, close]
}

#[get("/open")]
async fn open(valve_channel: &State<Sender<TxGpioControllerMessage>>) -> Json<Message> {
    send_valve_command(
        valve_channel,
        TxGpioControllerMessage::SetHigh,
        "Valve Opened".to_string(),
        "Failed to open valve".to_string(),
    )
    .await
}

#[get("/close")]
async fn close(valve_channel: &State<Sender<TxGpioControllerMessage>>) -> Json<Message> {
    send_valve_command(
        valve_channel,
        TxGpioControllerMessage::SetLow,
        "Valve closed".to_string(),
        "Failed to close valve".to_string(),
    )
    .await
}

async fn send_valve_command(
    valve_channel: &State<Sender<TxGpioControllerMessage>>,
    command: TxGpioControllerMessage,
    message_success: String,
    message_failure: String,
) -> Json<Message> {
    match valve_channel.send(command).await {
        Ok(_) => Json(Message {
            status: Status::Success,
            message: message_success,
            error: None,
        }),
        Err(e) => Json(Message {
            status: Status::Failure,
            message: message_failure,
            error: Some(format!("{}", e)),
        }),
    }
}
