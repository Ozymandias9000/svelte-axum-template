use crate::db::create_instance;
use axum::{response::IntoResponse, Json};
use axum_sessions::async_session::serde_json::json;

/// imitating an API response
#[allow(clippy::unused_async)]
pub async fn handler() -> impl IntoResponse {
    tracing::info!("Seeking api data");

    let db = match create_instance().await {
        Ok(client) => client,
        Err(error) => panic!("noooo {}", error),
    };

    // struct User {
    //     first_name: String,
    //     last_name: String,
    //     age: u32,
    // }

    let result = match db.query::<String, _>("SELECT User.firstName", &()).await {
        Ok(res) => res,
        Err(error) => {
            panic!("noooo {}", error)
        }
    };

    Json(
        json!({"result": "ok", "result": result, "message": "You've reached the backend API by using a valid token."}),
    )
}
