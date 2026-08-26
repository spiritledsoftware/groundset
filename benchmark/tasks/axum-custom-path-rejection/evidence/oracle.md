# Evidence packet

The repository pins Axum 0.8.9. The following excerpts apply to that version.

## Route captures

**Claim:** Axum 0.8 captures one path segment with `/{key}`. The older `/:key` form panics during route registration unless compatibility checks are disabled.

**Source:** Axum 0.8.9 `Router::route` documentation  
https://docs.rs/axum/0.8.9/axum/routing/struct.Router.html#method.route

> Paths can contain segments like `/{key}` which matches any single segment and will store the value captured at `key`.

## Handling an extractor rejection in one handler

**Claim:** To replace one extractor's rejection response in one handler, receive the extractor as `Result<T, T::Rejection>`. For `Path<u64>`, the concrete rejection type is `axum::extract::rejection::PathRejection`.

**Sources:** Axum 0.8.9 extractor and `PathRejection` documentation  
https://docs.rs/axum/0.8.9/axum/extract/index.html#customizing-extractor-responses  
https://docs.rs/axum/0.8.9/axum/extract/rejection/enum.PathRejection.html

> Use `Result<T, T::Rejection>` as your extractor. This works well if you're only using the extractor in a single handler.

## Building JSON responses

**Claim:** `Json<T>` implements `IntoResponse`. A tuple whose first item is `StatusCode` and whose final item implements `IntoResponse` also implements `IntoResponse`. Calling `.into_response()` converts either form to the same concrete `Response` type when branches need different response shapes.

**Source:** Axum 0.8.9 `IntoResponse` documentation  
https://docs.rs/axum/0.8.9/axum/response/trait.IntoResponse.html

## Router state

**Claim:** A handler reads router state with `State<S>`, and `Router::with_state` supplies that state.

**Source:** Axum 0.8.9 `State` documentation  
https://docs.rs/axum/0.8.9/axum/extract/struct.State.html#with-router

