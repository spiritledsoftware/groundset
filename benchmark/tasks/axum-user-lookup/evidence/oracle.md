# Evidence packet

The repository pins Axum 0.8.9. The following excerpts apply to that version.

## Route captures

**Claim:** Axum 0.8 captures one path segment with `/{key}`. The older `/:key` form panics during route registration unless compatibility checks are disabled.

**Source:** Axum 0.8.9 `Router::route` documentation  
https://docs.rs/axum/0.8.9/axum/routing/struct.Router.html#method.route

> Paths can contain segments like `/{key}` which matches any single segment and will store the value captured at `key`.

## Typed path extraction

**Claim:** `Path<T>` extracts captures and deserializes them. A single capture can be extracted directly into its target type.

**Source:** Axum 0.8.9 `Path` documentation  
https://docs.rs/axum/0.8.9/axum/extract/path/struct.Path.html

> If the path contains only one parameter, then you can omit the tuple.

## Router state

**Claim:** A handler reads router state with `State<S>`, and `Router::with_state` supplies that state. The state type must implement `Clone + Send + Sync + 'static`.

**Source:** Axum 0.8.9 `State` documentation  
https://docs.rs/axum/0.8.9/axum/extract/struct.State.html#with-router

## Handler responses

**Claim:** A handler may return `Result<T, StatusCode>` when `T` implements `IntoResponse`. `Json<T>` and `StatusCode` both implement `IntoResponse`.

**Sources:** Axum 0.8.9 response and error-handling documentation  
https://docs.rs/axum/0.8.9/axum/response/trait.IntoResponse.html  
https://docs.rs/axum/0.8.9/axum/error_handling/index.html
