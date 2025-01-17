#![allow(unused_imports)]

use std::sync::Arc;

use crate::grpc::api_model::NOT_FOUND;
use crate::{
    common::appdata::AppShareData,
    config::core::{ConfigActor, ConfigCmd, ConfigKey, ConfigResult},
    grpc::{
        api_model::{
            BaseResponse, ConfigPublishRequest, ConfigQueryRequest, ConfigQueryResponse,
            ERROR_CODE, SUCCESS_CODE,
        },
        nacos_proto::Payload,
        PayloadHandler, PayloadUtils,
    },
};
use actix::prelude::Addr;
use async_trait::async_trait;

pub struct ConfigQueryRequestHandler {
    app_data: Arc<AppShareData>,
}

impl ConfigQueryRequestHandler {
    pub fn new(app_data: Arc<AppShareData>) -> Self {
        Self { app_data }
    }
}

#[async_trait]
impl PayloadHandler for ConfigQueryRequestHandler {
    async fn handle(
        &self,
        request_payload: crate::grpc::nacos_proto::Payload,
        _request_meta: crate::grpc::RequestMeta,
    ) -> anyhow::Result<Payload> {
        let body_vec = request_payload.body.unwrap_or_default().value;
        let request: ConfigQueryRequest = serde_json::from_slice(&body_vec)?;
        let cmd = ConfigCmd::GET(ConfigKey::new(
            &request.data_id,
            &request.group,
            &request.tenant,
        ));
        let mut response = ConfigQueryResponse {
            request_id: request.request_id,
            ..Default::default()
        };
        match self.app_data.config_addr.send(cmd).await {
            Ok(res) => {
                //let res:ConfigResult = res.unwrap();
                let r: ConfigResult = res.unwrap();
                match r {
                    ConfigResult::DATA(content, md5) => {
                        //v.to_owned()
                        response.result_code = SUCCESS_CODE;
                        response.content = content;
                        response.tag = request.tag;
                        response.md5 = Some(md5);
                    }
                    _ => {
                        response.result_code = ERROR_CODE;
                        response.error_code = NOT_FOUND;
                        response.message = Some("config data not exist".to_owned());
                    }
                }
                Ok(PayloadUtils::build_payload(
                    "ConfigQueryResponse",
                    serde_json::to_string(&response)?,
                ))
            }
            Err(err) => {
                response.result_code = ERROR_CODE;
                response.error_code = ERROR_CODE;
                response.message = Some(err.to_string());
                Ok(PayloadUtils::build_payload(
                    "ErrorResponse",
                    serde_json::to_string(&response)?,
                ))
            }
        }
    }
}
