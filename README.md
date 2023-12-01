# sms_boom_r

接口测试工具 or 短信轰炸

项目重写自 [SMSBombing](https://github.com/xiaoxuan6/SMSBombing)

`api.json` 为接口配置文件，可自行添加接口（感谢 [xiaoxuan6](https://github.com/xiaoxuan6) 的收集）

## 使用方法

```bash 
sms_boom_r 13245678901 1 10 10
```

### 运行参数

* `phone` 为手机号 必填

* `loop_count` 为循环次数，默认为 1，选填

* `sleep_time` 为每次循环间隔时间，单位s，默认为 10，可选

* `TIMEOUT_MS ` 为每次循环调用接口次数，单位s， 默认为 10，可选

## 开发说明

* 安装[rust](https://www.rust-lang.org/zh-CN/tools/install)环境
* 执行 `cargo run` 运行程序不要忘了参数哦
* 执行 `cargo build --release` 生成可执行文件

# 免责声明

若使用者滥用本项目，本人 无需承担 任何法律责任， 本程序仅供娱乐，源码全部开源，禁止滥用 和二次 贩卖盈利， 禁止用于商业用途！