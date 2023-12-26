mod write_to_file;
mod struct_mod;

use struct_mod::{ResponseResult, RqStruct};
use std::{env, fs};

use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::time::sleep;
use regex::Regex;
use std::error::Error;


#[tokio::main]
async fn main() {
    match get_env() {
        Ok((phone, num, time_out, request_timeout)) => {
            // 将env中的参数传递给函数
            env_rq(phone, num, time_out, request_timeout).await;
        }
        Err(e) => {
            println!("{}", e);
            // 终止程序 ...
            std::process::exit(1);
        }
    }
}


async fn env_rq(phone: String, num: u64, time_out: u64, request_timeout: u64) {
    println!("手机号是: {}", phone);
    println!("循环次数是: {}", num);
    println!("循环间隔是: {}", time_out);
    println!("请求超时时长是: {}", request_timeout);
    println!();
    let json_data = fs::read_to_string("api.json").expect("Unable to read file");

    let api_configs: Vec<RqStruct> = serde_json::from_str(&json_data).expect("Failed to deserialize JSON");

    // 根据num，来决定进行几次循环
    for i in 0..num {
        println!("第{}次循环", i + 1);
        println!("-------------------");

        for api_config in &api_configs {

            if api_config.obsolete.unwrap_or(false) {
                continue;
            }

            let result = sed_rq(api_config.clone(), time_out, phone.as_str()).await;
            println!("请求结果：{:#?}", result);
            println!();
        }

        println!("-------------------");
        println!();

        if num == 1 {
            break;
        }
        // 创建一个新的进度条
        let pb = ProgressBar::new(request_timeout);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").expect("Failed to set style")
            .progress_chars("#>-"));

        for _ in 0..request_timeout {
            pb.inc(1);
            sleep(Duration::from_secs(1)).await;
        }

        // 完成进度条
        pb.finish_with_message("done");
    }

    write_to_file::write_to_json(api_configs).unwrap();
}


async fn sed_rq(rq_body: RqStruct, time_out: u64, phone: &str) -> ResponseResult {
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
            ResponseResult { desc: rq_body.desc.unwrap(), success: true, msg: response.text().await.unwrap() }
        }
        Err(err) => {
            ResponseResult { desc: rq_body.desc.unwrap(), success: false, msg: err.to_string() }
        }
    }
}


fn get_env() -> Result<(String, u64, u64, u64), Box<dyn Error>> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了手机号
    if args.len() < 2 {
        return Err("至少提供手机号！".into());
    }

    let phone = &args[1];
    // 创建一个新的正则表达式
    let re = Regex::new(r"^1[3-9]\d{9}$")?;

    // 检查手机号是否匹配正则表达式
    if !re.is_match(phone) {
        return Err("手机号格式不正确！".into());
    }

    // 获取循环次数，如果用户没有提供，则默认为1
    let num = args.get(2).unwrap_or(&"1".to_string()).parse::<u64>()?;
    // 获取循环间隔，如果用户没有提供，则默认为10
    let time_out = args.get(3).unwrap_or(&"10".to_string()).parse::<u64>()?;
    // 获取请求超时时长，如果用户没有提供，则默认为10
    let request_timeout = args.get(4).unwrap_or(&"10".to_string()).parse::<u64>()?;

    Ok((phone.clone(), num, time_out, request_timeout))
}

