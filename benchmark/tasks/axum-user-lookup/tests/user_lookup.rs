use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use axum_user_lookup::{User, app};
use http_body_util::BodyExt;
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

async fn get(uri: &str) -> axum::response::Response {
    app(users())
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn returns_the_matching_user_as_json() {
    let response = get("/users/7").await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
        serde_json::json!({"id": 7, "name": "Ada"})
    );
}

#[tokio::test]
async fn returns_an_empty_404_for_an_unknown_user() {
    let response = get("/users/99").await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert!(
        response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .is_empty()
    );
}

#[tokio::test]
async fn lets_axum_reject_an_invalid_user_id() {
    assert_eq!(
        get("/users/not-a-number").await.status(),
        StatusCode::BAD_REQUEST
    );
}
