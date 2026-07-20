// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.
use axum::{extract::State, http::StatusCode, routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

#[tokio::main]
async fn main() {
    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn create_app() -> Router {
    let store = Arc::new(TicketStore::new());
    Router::new()
        .route("/create", post(create_ticket))
        .route("/get", get(get_ticket))
        .route("/update", post(patch_ticket))
        .with_state(store)
}

#[axum::debug_handler]
async fn create_ticket(
    store: State<Arc<TicketStore>>,
    Json(draft): Json<TicketDraft>,
) -> (StatusCode, Json<TicketId>) {
    todo!();
    (StatusCode::CREATED, Json(todo!()))
}

#[axum::debug_handler]
async fn get_ticket(
    store: State<Arc<TicketStore>>,
    Json(id): Json<TicketId>,
) -> (StatusCode, Json<Ticket>) {
    todo!();
    (StatusCode::OK, Json(todo!()))
}

#[axum::debug_handler]
async fn patch_ticket(
    store: State<Arc<TicketStore>>,
    Json(patch): Json<TicketPatch>,
) -> StatusCode {
    todo!();
    StatusCode::OK
}
