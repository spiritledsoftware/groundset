use axum::Router;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

pub fn app(users: HashMap<u64, User>) -> Router {
    todo!("implement the user lookup route with Axum 0.8")
}
