use std::env::{var, set_var, VarError};

const API_URL_KEY: &str = "API_URL";
const CLIENT_TOKEN_KEY: &str = "CLIENT_TOKEN";
const GATEWAY_URL_KEY: &str = "GATEWAY_URL";

fn _get_prefixed(key: &str) -> String {
    format!("__DISCORD_RS__{key}")
}

// CLIENT TOKEN
pub fn _set_client_token(token: &str) {
    set_var(_get_prefixed(CLIENT_TOKEN_KEY), token);
}

pub fn _get_client_token() -> Result<String, VarError> {
    var(_get_prefixed(CLIENT_TOKEN_KEY))
}

// REQUEST URLS
pub fn _set_request_urls(api_version: &u8) {
    // Set the URL for HTTP requests
    set_var(
        _get_prefixed(API_URL_KEY), 
        format!("https://discord.com/api/v{api_version}")
    );

    // Set the URL for the gateway websocket
    set_var(
        _get_prefixed(GATEWAY_URL_KEY), 
        format!("wss://gateway.discord.gg/?v={api_version}&encoding=json")
    );
}

pub fn _get_api_url() -> Result<String, VarError> {
    var(_get_prefixed(API_URL_KEY))
}

pub fn _get_gateway_url() -> Result<String, VarError> {
    var(_get_prefixed(GATEWAY_URL_KEY))
}