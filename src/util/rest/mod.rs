use crate::util::env::{
    get_api_url,
    get_client_token
};

use reqwest::blocking::{
    Client as ReqwestClient,
    Response,
};

use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum RequestError {
    HttpClientError(ReqwestError),
    MissingEnvironmentVariable(String),
    InvalidMethod(String),
}

impl From<ReqwestError> for RequestError {
    fn from(error: ReqwestError) -> Self {
        RequestError::HttpClientError(error)
    }
}

pub fn get(path: &str) -> Result<Response, RequestError> {
    perform_request("GET", path, None)
}

pub fn post(path: &str, body: &str) -> Result<Response, RequestError> {
    perform_request("POST", path, Some(body))
}

// UNUSED YET
// pub fn patch(path: &str, body: &str) -> Result<Response, RequestError> {
//     perform_request("PATCH", path, Some(body))
// }

// pub fn put(path: &str, body: &str) -> Result<Response, RequestError> {
//     perform_request("PUT", path, Some(body))
// }

fn perform_request(
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<Response, RequestError> {
    let base_url = get_api_url()
        .map_err(|_| RequestError::MissingEnvironmentVariable("API URL".to_owned()))?;
    
    let token = get_client_token()
        .map_err(|_| RequestError::MissingEnvironmentVariable("CLIENT TOKEN".to_owned()))?;

    let client = ReqwestClient::new();
    let url = format!("{}/{}", base_url, path);

    let request = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PATCH" => client.patch(&url),
        "PUT" => client.put(&url),
        _ => return Err(RequestError::InvalidMethod(method.to_owned())),
    };

    request
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .body(body.unwrap_or("").to_owned())
        .send()
        .map_err(|op| RequestError::HttpClientError(op))
}