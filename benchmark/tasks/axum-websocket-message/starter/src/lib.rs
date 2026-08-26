use axum::extract::ws::Message;

pub fn reply(message: Message) -> Option<Message> {
    todo!("handle Axum 0.8 WebSocket message payloads")
}
