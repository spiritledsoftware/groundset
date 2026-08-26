# Task: Axum WebSocket reply

Implement `reply` in `src/lib.rs`.

Requirements:

- A text message containing `hello` returns a text message containing `echo: hello`.
- A binary message returns the same binary payload.
- A ping returns a pong with the same payload.
- A pong returns `None`.
- A close message returns the same close message.
- Keep the public `reply` signature unchanged.
- Do not add dependencies.

Run `cargo test` and `cargo clippy --all-targets -- -D warnings` before finishing.

