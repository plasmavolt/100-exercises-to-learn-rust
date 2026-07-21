use axum_test::TestServer;
use outro_08::*;
use serde_json::json;

fn create_test_server() -> TestServer {
    TestServer::new(create_app())
}

#[tokio::test]
async fn test_ticket() {
    let server = create_test_server();

    let create_response = server
        .post(&"/create")
        .json(&json!({
            "title": "test",
            "description": "lorem ipsum",
        }))
        .await;
    let id = create_response.json::<outro_08::store::TicketId>();
    create_response.assert_status_success();

    let get_response = server.get(&"/get").json(&id).await;
    get_response.assert_status_ok();
    get_response.assert_json(&json!({
        "id": 0,
        "title": "test",
        "description": "lorem ipsum",
        "status": "ToDo",
    }));

    let patch_response = server
        .post(&"/update")
        .json(&json!({
            "id": id,
            "title": "new title",
            "status": "Done",
        }))
        .await;
    patch_response.assert_status_ok();
}
