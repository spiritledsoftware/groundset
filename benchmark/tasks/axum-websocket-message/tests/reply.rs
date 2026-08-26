use axum::extract::ws::Message;
use axum_websocket_message::reply;

#[test]
fn prefixes_text_messages() {
    assert_eq!(
        reply(Message::text("hello")),
        Some(Message::text("echo: hello"))
    );
}

#[test]
fn preserves_binary_messages() {
    let message = Message::binary(vec![1, 2, 3]);
    assert_eq!(reply(message.clone()), Some(message));
}

#[test]
fn answers_ping_messages() {
    assert_eq!(
        reply(Message::Ping(vec![1, 2, 3].into())),
        Some(Message::Pong(vec![1, 2, 3].into()))
    );
}

#[test]
fn ignores_pong_messages() {
    assert_eq!(reply(Message::Pong(vec![1].into())), None);
}

#[test]
fn preserves_close_messages() {
    let message = Message::Close(None);
    assert_eq!(reply(message.clone()), Some(message));
}
