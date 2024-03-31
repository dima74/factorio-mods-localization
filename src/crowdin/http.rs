use std::ops::Deref;

use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::crowdin::StorageId;
use crate::myenv::{CROWDIN_API_KEY, CROWDIN_PROJECT_ID};

const BASE_URL: &str = "https://api.crowdin.com/api/v2";

#[derive(Deserialize)]
pub struct DataWrapper<T> { pub data: T }

#[derive(Deserialize)]
pub struct IdResponse { pub id: i64 }

#[derive(Deserialize)]
pub struct UnitResponse {}

async fn send_request<T: DeserializeOwned>(
    crowdin_path: &str,
    http_method: Method,
    before_send: impl FnOnce(RequestBuilder) -> RequestBuilder
) -> T {
    let url = format!("{}/projects/{}{}", BASE_URL, CROWDIN_PROJECT_ID.deref(), crowdin_path);
    let request = reqwest::Client::new()
        .request(http_method, &url)
        .bearer_auth(CROWDIN_API_KEY.deref());
    let request = before_send(request);

    let response = request.send().await.unwrap();
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        let response_text = response.text().await.unwrap();
        panic!("Request to {} failed with code {}, response: `{}`", url, status.as_u16(), response_text);
    }

    response.json::<DataWrapper<T>>().await.unwrap().data
}

async fn crowdin_get<Res: DeserializeOwned>(
    path: &str,
    before_send: impl FnOnce(RequestBuilder) -> RequestBuilder
) -> Res {
    send_request(path, Method::GET, before_send).await
}

pub async fn crowdin_get_empty_query<Res: DeserializeOwned>(path: &str) -> Res {
    crowdin_get(path, |request| request).await
}

pub async fn crowdin_get_pagination<T: DeserializeOwned>(path: &str, before_send: impl Fn(RequestBuilder) -> RequestBuilder) -> Vec<T> {
    const LIMIT: usize = 500;
    let mut result = Vec::new();
    for i in (0..).step_by(LIMIT) {
        let items: Vec<T> = crowdin_get(path, |request| {
            let request = before_send(request);
            let request = request.query(&[("offset", i)]);
            let request = request.query(&[("limit", LIMIT)]);
            request
        }).await;
        if items.is_empty() { break; }
        result.extend(items);
    }
    result
}

pub async fn crowdin_get_pagination_empty_query<T: DeserializeOwned>(path: &str) -> Vec<T> {
    crowdin_get_pagination(path, |request| request).await
}

pub async fn crowdin_post<Req: Serialize, Res: DeserializeOwned>(path: &str, data: Req) -> Res {
    send_request(path, Method::POST, |request| request.json(&data)).await
}

pub async fn crowdin_post_empty_body<Res: DeserializeOwned>(path: &str) -> Res {
    send_request(path, Method::POST, |request| request).await
}

pub async fn crowdin_put<Req: Serialize, Res: DeserializeOwned>(method: &str, data: Req) -> Res {
    send_request(method, Method::PUT, |request| request.json(&data)).await
}

// https://developer.crowdin.com/api/v2/#operation/api.storages.post
pub async fn upload_file_to_storage(file_content: String, file_name: &str) -> StorageId {
    let url = format!("{}/storages", BASE_URL);
    reqwest::Client::new()
        .post(&url)
        .body(file_content)
        .header("Crowdin-API-FileName", file_name)
        .bearer_auth(CROWDIN_API_KEY.deref())
        .send().await.unwrap()
        .json::<DataWrapper<IdResponse>>().await.unwrap()
        .data.id
}
