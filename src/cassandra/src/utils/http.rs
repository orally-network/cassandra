use candid::CandidType;
use ic_cdk::{
    api::management_canister::http_request::{
        HttpResponse, TransformArgs, TransformContext, TransformFunc,
    },
    query,
};
use serde::Deserialize;

use crate::get_metadata;

#[query]
pub fn transform(response: TransformArgs) -> HttpResponse {
    HttpResponse {
        status: response.response.status,
        body: response.response.body,
        headers: Vec::new(),
    }
}

pub fn wrap_url(url: &str) -> String {
    format!(
        "{}{}",
        get_metadata!(orally_wrapper),
        urlencoding::encode(url)
    )
}

pub fn transform_ctx() -> TransformContext {
    TransformContext {
        function: TransformFunc(candid::Func {
            principal: ic_cdk::api::id(),
            method: "transform".into(),
        }),
        context: vec![],
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
