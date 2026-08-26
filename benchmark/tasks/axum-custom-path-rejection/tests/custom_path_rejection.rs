use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use axum_custom_path_rejection::{User, app};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::collections::HashMap;
use tower::ServiceExt;

fn users() -> HashMap<u64, User> {
    [(
        7,
        User {
            id: 7,
            name: "Ada".into(),
        },
    )]
    .into()
}

async fn assert_json(uri: &str, status: StatusCode, expected: Value) {
    let response = app(users())
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), status);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(serde_json::from_slice::<Value>(&body).unwrap(), expected);
}

#[tokio::test]
async fn returns_the_matching_user() {
    assert_json("/users/7", StatusCode::OK, json!({"id": 7, "name": "Ada"})).await;
}

#[tokio::test]
async fn returns_a_json_error_for_an_unknown_user() {
    assert_json(
        "/users/99",
        StatusCode::NOT_FOUND,
        json!({"error": "user not found"}),
    )
    .await;
}

#[tokio::test]
async fn replaces_the_path_rejection_with_a_json_error() {
    assert_json(
        "/users/not-a-number",
        StatusCode::BAD_REQUEST,
        json!({"error": "invalid user id"}),
    )
    .await;
}
