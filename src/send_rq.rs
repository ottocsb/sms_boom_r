use std::time::Duration;
use crate::struct_mod::{ResponseResult, RqStruct};

pub async fn sed_rq(rq_body: RqStruct, time_out: u64, phone: &str) -> ResponseResult<String> {
    let client = reqwest::Client::new();
    let url = rq_body.url.unwrap().replace("[phone]", phone);
    let mut request_builder = client.request(
        reqwest::Method::from_bytes(rq_body.method.unwrap().as_bytes()).unwrap(),
        url,
    );

    request_builder = request_builder.timeout(Duration::from_secs(time_out));

    // 添加 headers
    if let Some(headers) = rq_body.header {
        for (key, value) in headers {
            request_builder = request_builder.header(key, value.replace("[phone]", phone));
        }
    }

    // 添加 data
    if let Some(mut data) = rq_body.data {
        for (_, value) in data.iter_mut() {
            *value = value.replace("[phone]", phone);
        }
        if rq_body.form.is_some() {
            let form_data = serde_urlencoded::to_string(data).unwrap();
            request_builder = request_builder.body(form_data);
        } else {
            let json_data = serde_json::json!(data);
            request_builder = request_builder.json(&json_data);
        }
    }

    match request_builder.send().await {
        Ok(response) => {
            ResponseResult::new(rq_body.desc.unwrap(), true, Some(response.text().await.unwrap()))
        }

        Err(err) => {
            ResponseResult::new(rq_body.desc.unwrap(), false, Some(err.to_string()))
        }
    }
}