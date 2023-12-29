mod struct_mod;
mod route_fun;
mod send_rq;

use send_rq::sed_rq;

use route_fun::{get_json, test_api};
use struct_mod::{ RqStruct};
use tower_http::cors::{Any, CorsLayer};

use std::{env, fs};
use std::time::Duration;
use axum::{
    routing::{get, post},
    response::Html,
    http::Method,
    Router,
};

use indicatif::{ProgressBar, ProgressStyle};
use tokio::time::sleep;
use regex::Regex;

#[tokio::main]
async fn main() {

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    let mode = if args[1] == "--server" { "server mode" } else { "release mode" };
    println!("{}", mode);
    // 检查是否为调试模式
    if args[1] == "--server" {
        let cors = CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow requests from any origin
            .allow_origin(Any);


        let app = Router::new()
            .route("/", get(|| async { Html("<h1>Hello,Word!</h1>") }))
            .route("/getJsonData", get(get_json))
            .route("/testApi", post(test_api))
            .layer(cors);


        // 运行hyper  http服务 localhost:3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:2024").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {

        // 检查是否提供了手机号
        if args.len() < 2 {
            println!("命令行模式至少提供手机号！");
            std::process::exit(1);
        }

        let phone = &args[1];
        // 创建一个新的正则表达式
        let re = Regex::new(r"^1[3-9]\d{9}$").unwrap();

        // 检查手机号是否匹配正则表达式
        if !re.is_match(phone) {
            println!("手机号格式不正确！");
            std::process::exit(1);
        }

        // 获取循环次数，未提供则默认为1
        let num = args.get(2).unwrap_or(&"1".to_string()).parse::<u64>().unwrap_or(1);
        // 获取循环间隔，未提供则默认为10
        let time_out = args.get(3).unwrap_or(&"10".to_string()).parse::<u64>().unwrap_or(10);
        // 获取请求超时时长，未提供则默认为10
        let request_timeout = args.get(4).unwrap_or(&"10".to_string()).parse::<u64>().unwrap_or(10);

        env_rq(phone.clone(), num, time_out, request_timeout).await
    }
}

async fn env_rq(phone: String, num: u64, time_out: u64, request_timeout: u64) {
    println!("目标号码: {}", phone);
    println!("循环次数: {}", num);
    println!("循环间隔: {}", time_out);
    println!("请求超时时长: {}", request_timeout);

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
            println!("请求结果：\n{:#?}", result);
        }

        println!("-------------------");

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

}
