# Task: Custom Axum path rejection

Implement `app` in `src/lib.rs`.

Requirements:

- A `GET` request such as `/users/7` returns status `200` and the matching `User` as JSON.
- An unknown user returns status `404` and `{"error":"user not found"}` as JSON.
- An ID that cannot be parsed as `u64` returns status `400` and `{"error":"invalid user id"}` as JSON.
- Use Axum's `Path<u64>` extractor for ID parsing, but replace its normal rejection response.
- Keep the public `User` type and `app` signature unchanged.
- Do not add dependencies.

Run `cargo test` and `cargo clippy --all-targets -- -D warnings` before finishing.

