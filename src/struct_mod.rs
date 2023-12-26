use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ResponseResult {
     pub(crate) desc: String,
     pub(crate) success: bool,
     pub(crate) msg: String,
}

#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct RqStruct {
     pub(crate) desc: Option<String>,
     pub(crate) url: Option<String>,
     pub(crate) method: Option<String>,
     pub(crate) data: Option<HashMap<String,String>>,
     pub(crate) header: Option<HashMap<String,String>>,
     pub(crate) form: Option<String>,
     pub(crate) obsolete: Option<bool>,
}