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
use std::sync::{Arc, RwLock};

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
    let store = Arc::new(RwLock::new(TicketStore::new()));
    Router::new()
        .route("/create", post(create_ticket))
        .route("/get", get(get_ticket))
        .route("/update", post(patch_ticket))
        .with_state(store)
}

#[axum::debug_handler]
async fn create_ticket(
    State(store): State<Arc<RwLock<TicketStore>>>,
    Json(draft): Json<TicketDraft>,
) -> (StatusCode, Json<TicketId>) {
    let id = store.write().unwrap().add_ticket(draft);
    (StatusCode::CREATED, Json(id))
}

#[axum::debug_handler]
async fn get_ticket(
    State(store): State<Arc<RwLock<TicketStore>>>,
    Json(id): Json<TicketId>,
) -> (StatusCode, Json<Arc<RwLock<Ticket>>>) {
    let reader = store.read().unwrap();
    let ticket = reader.get(id).unwrap();
    (StatusCode::OK, Json(ticket))
}

#[axum::debug_handler]
async fn patch_ticket(
    State(store): State<Arc<RwLock<TicketStore>>>,
    Json(patch): Json<TicketPatch>,
) -> StatusCode {
    let reader = store.read().unwrap();
    let arc = reader.get(patch.id).unwrap();
    let mut ticket = arc.write().unwrap();

    if let Some(title) = patch.title {
        ticket.title = title;
    }
    if let Some(description) = patch.description {
        ticket.description = description;
    }
    if let Some(status) = patch.status {
        ticket.status = status;
    }

    StatusCode::OK
}
