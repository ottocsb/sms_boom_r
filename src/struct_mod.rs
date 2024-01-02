use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ResponseResult<T: Serialize> {
    pub(crate) desc: String,
    pub(crate) result: bool,
    pub(crate) data: Option<T>,
}

impl<T> ResponseResult<T>
    where T: Serialize
{
    pub fn new(desc: String, result: bool, data: Option<T>) -> Self {
        Self {
            desc,
            result,
            data,
        }
    }
}

// 创建实例 添加new方法
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RqStruct {
    pub(crate) desc: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) method: Option<String>,
    pub(crate) data: Option<HashMap<String, String>>,
    pub(crate) header: Option<HashMap<String, String>>,
    pub(crate) form: Option<String>,
    pub(crate) obsolete: Option<bool>,
}


/// 返回体
/// example:
/// ```json
/// {
///   "code": 0,
///   "msg": "OK",
///   "data": { }
/// }
/// ```
#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// 实现Response
impl<T> Response<T>
    where
        T: Serialize,
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
    pub fn ok(data: T) -> Self {
        Self::new(200, "OK".to_string(), Some(data))
    }
    pub fn err(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}