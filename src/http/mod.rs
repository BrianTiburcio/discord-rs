use reqwest::Error as ReqwestError;
use reqwest::blocking::{
    Client as ReqwestClient,
    Response,
};

use crate::util::env::{
    _get_api_url,
    _get_client_token
};

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
    let base_url = _get_api_url()
        .map_err(|_| RequestError::MissingEnvironmentVariable("API URL".to_owned()))?;
    
    let token = _get_client_token()
        .map_err(|_| RequestError::MissingEnvironmentVariable("CLIENT TOKEN".to_owned()))?;

    // Access the package version and repository URL from environment variables
    const BOT_VERSION: &str = env!("CARGO_PKG_VERSION");
    const BOT_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

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
        // https://discord.com/developers/docs/reference#authentication-example-bot-token-authorization-header
        .header("Authorization", format!("Bot {}", token))
        // https://discord.com/developers/docs/reference#user-agent-user-agent-example
        .header("User-Agent", format!("DiscordBot ({}, v{})", BOT_REPOSITORY, BOT_VERSION))
        .header("Content-Type", "application/json")
        .body(body.unwrap_or("").to_owned())
        .send()
        .map_err(RequestError::HttpClientError)
}