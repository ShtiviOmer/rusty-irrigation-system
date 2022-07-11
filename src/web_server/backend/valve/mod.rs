use rocket::{post, routes, serde::json::Json, State};
use tokio::sync::mpsc::Sender;

use crate::gpio_controller::task::TxGpioControllerMessage;

use super::{Message, Status};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![open, close]
}

#[post("/open")]
async fn open(valve_channel: &State<Sender<TxGpioControllerMessage>>) -> Json<Message> {
    send_valve_command(
        valve_channel,
        TxGpioControllerMessage::SetHigh,
        "Valve Opened".to_string(),
        "Failed to open valve".to_string(),
    )
    .await
}

#[post("/close")]
async fn close(valve_channel: &State<Sender<TxGpioControllerMessage>>) -> Json<Message> {
    send_valve_command(
        valve_channel,
        TxGpioControllerMessage::SetLow,
        "Valve Closed".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use tokio::sync::mpsc;

    #[test]
    fn test_open() {
        assert_valve_message(
            "/valve/open",
            crate::web_server::backend::Status::Success,
            TxGpioControllerMessage::SetHigh,
            "Valve Opened".to_string(),
        );
    }

    #[test]
    fn test_close() {
        assert_valve_message(
            "/valve/close",
            crate::web_server::backend::Status::Success,
            TxGpioControllerMessage::SetLow,
            "Valve Closed".to_string(),
        );
    }

    fn assert_valve_message(
        valve_uri: &str,
        status: crate::web_server::backend::Status,
        valve_message: TxGpioControllerMessage,
        message: String,
    ) {
        let (sender, mut receiver) = mpsc::channel(100);
        let server = crate::web_server::backend::start(sender);

        let client = Client::tracked(server).unwrap();
        let response = client.post(valve_uri).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let results: Message = response.into_json().unwrap();
        let expected = Message::new(status, message, None);
        assert_eq!(results, expected);
        let result_message = receiver.try_recv().unwrap();
        assert_eq!(result_message, valve_message);
    }
}
