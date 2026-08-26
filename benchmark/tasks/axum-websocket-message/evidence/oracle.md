# Evidence packet

The repository pins Axum 0.8.9. The following excerpts apply to that version.

## Axum 0.8 WebSocket payload change

**Claim:** Axum 0.8 replaced the old `String` and `Vec<u8>` WebSocket variant payloads with `Utf8Bytes` and `Bytes`.

**Source:** Axum 0.8 changelog  
https://github.com/tokio-rs/axum/blob/axum-v0.8.9/axum/CHANGELOG.md#080

> `axum::extract::ws::Message` now uses `Bytes` in place of `Vec<u8>`, and a new `Utf8Bytes` type in place of `String`, for its variants.

## Message variants

**Claim:** In Axum 0.8.9, the variants are `Text(Utf8Bytes)`, `Binary(Bytes)`, `Ping(Bytes)`, `Pong(Bytes)`, and `Close(Option<CloseFrame>)`.

**Source:** Axum 0.8.9 `Message` documentation  
https://docs.rs/axum/0.8.9/axum/extract/ws/enum.Message.html

## Constructors

**Claim:** `Message::text(value)` accepts any value implementing `Into<Utf8Bytes>`. `Message::binary(value)` accepts any value implementing `Into<Bytes>`.

**Source:** Axum 0.8.9 `Message` documentation  
https://docs.rs/axum/0.8.9/axum/extract/ws/enum.Message.html#method.text

