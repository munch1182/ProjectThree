use std::fs;

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_repr::*;

use super::basebean::BaseBean;

const PATH: &'static str = "test_apidoc.json";

pub async fn query_apidoc() -> Json<BaseBean<ApiDoc>> {
    if let Ok(s) = fs::read_to_string(PATH) {
        if let Ok(b) = serde_json::from_str::<ApiDoc>(&s) {
            return Json(BaseBean::success().data(b).clone());
        }
    }
    Json(BaseBean::error1())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiDoc {
    pub name: String,      // 存放名称,即存放路径
    pub baseurl: String,   // 项目基础url
    pub status: ApiStatus, // 项目状态

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Api>, // 接口
}

impl ApiDoc {
    pub fn new(name: String, baseurl: String) -> Self {
        Self {
            name,
            baseurl,
            status: ApiStatus::Offline,
            api: None,
        }
    }

    pub fn set_api(&mut self, api: Api) -> &mut Self {
        self.api = Some(api);
        return self;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    pub url: String, // 属于此对象的url部分, 完整的url会加上所属部分的api的url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // 接口名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>, // 接口描述

    #[serde(skip_serializing_if = "Option::is_none")]
    pub req: Option<Vec<Parameter>>, // 请求参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<Vec<Parameter>>, // 响应参数

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Api>>, // 当此api对象是公共路径部分时, 此参数才有值
}

impl Api {
    fn new(url: String) -> Self {
        Self {
            url,
            name: None,
            desc: None,
            req: None,
            res: None,
            children: None,
        }
    }

    fn addreq(&mut self, req: Parameter) -> &mut Self {
        if let Some(r) = &mut self.req {
            r.push(req)
        } else {
            self.req = Some(vec![req]);
        }
        return self;
    }

    fn addres(&mut self, res: Parameter) -> &mut Self {
        if let Some(r) = &mut self.req {
            r.push(res)
        } else {
            self.res = Some(vec![res]);
        }
        return self;
    }

    fn add(&mut self, api: &Api) -> &mut Self {
        if let Some(apis) = &mut self.children {
            apis.push(api.clone());
        } else {
            self.children = Some(vec![api.clone()]);
        }
        return self;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    p_name: String,  // 参数名称
    p_type: ApiType, // 值类型
    optional: bool,  // 是否是可选参数
}

impl Parameter {
    pub fn must(p_name: String, p_type: ApiType) -> Self {
        Self {
            p_name,
            p_type,
            optional: false,
        }
    }
    pub fn opt(p_name: String, p_type: ApiType) -> Self {
        Self {
            p_name,
            p_type,
            optional: true,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ApiType {
    Str = 0,
    Int = 1,
    Long = 2,
    Double = 3,
    U8 = 4,
    Json = 5,
    Array = 6,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ApiStatus {
    Offline = 0,    // 未上线
    Online = 1,     // 正在线上使用
    Deprecated = 2, // 上线过但已被废弃
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let baseurl = String::from("http://127.0.0.1/v1");
        let mut apidoc = ApiDoc::new("test_apidoc.json".to_string(), baseurl);

        let mut cmd_api = Api::new("/cmd".to_string());

        {
            let mut cmd_set_api = Api::new("/set".to_string());
            cmd_set_api.addreq(Parameter::must("env".to_string(), ApiType::Str));

            let mut cmd_remove_api = Api::new("/remove".to_string());
            cmd_remove_api.addreq(Parameter::must("env".to_string(), ApiType::Str));

            cmd_api.add(&cmd_set_api);
            cmd_api.add(&cmd_remove_api);
        }

        apidoc.set_api(cmd_api);

        println!("{:#?}", apidoc);

        let json = serde_json::to_string_pretty(&apidoc).unwrap();
        println!("{}", json);

        fs::write(PATH, json).unwrap();
    }
}
