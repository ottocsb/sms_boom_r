use std::fs;
use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use super::struct_mod::{Response, RqStruct};
use super::sed_rq;

pub async fn get_json() -> impl IntoResponse {
    let json_data = fs::read("api.json").expect("Unable_to_read_file");
    let json_value: serde_json::Value = serde_json::from_slice(&json_data).expect("Failed to parse JSON");
    Json(Response::ok(json_value))
}

#[derive(Deserialize, Serialize)]
pub struct TestApiParam {
    phone: String,
    data: RqStruct,
}

pub async fn test_api(Json(payload): Json<TestApiParam>) -> impl IntoResponse {
    println!("接收参数：\n{:#?}", payload.data);
    // 补全参数
    let res = sed_rq(payload.data, 10, payload.phone.as_str()).await;
    println!("返回结果：\n{:#?}", res);
    Json(res)
}
