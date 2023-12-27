use std::fs;
use axum::Json;
use axum::response::IntoResponse;

pub async fn get_json() -> impl IntoResponse {
    let json_data = fs::read("api.json").expect("Unable_to_read_file");
    let json_value: serde_json::Value = serde_json::from_slice(&json_data).expect("Failed to parse JSON");
    Json(json_value)
}