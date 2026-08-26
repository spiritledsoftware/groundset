# Task: Axum user lookup

Implement `app` in `src/lib.rs`.

Requirements:

- A `GET` request such as `/users/7` returns status `200` and the matching `User` as JSON.
- An unknown user returns status `404` with an empty body.
- Let Axum reject an ID that cannot be parsed as `u64`.
- Keep the public `User` type and `app` signature unchanged.
- Do not add dependencies.

Run `cargo test` and `cargo clippy --all-targets -- -D warnings` before finishing.
