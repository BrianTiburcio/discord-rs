use std::env::{var, set_var, VarError};

fn _get_key_name_prefixed(key: &str) -> String {
    format!("DISCORD_RS_{key}")
}

pub fn _set_client_token(token: &str) {
    set_var(_get_key_name_prefixed("CLIENT_TOKEN"), token);
}

pub fn _get_client_token() -> Result<String, VarError> {
    var(_get_key_name_prefixed("CLIENT_TOKEN"))
}

pub fn _set_api_url(api_version: &u8) {
    set_var(
        _get_key_name_prefixed("DISCORD_API_URL"), 
        format!("https://discord.com/api/v{api_version}")
    );
}

pub fn _get_api_url() -> Result<String, VarError> {
    var(_get_key_name_prefixed("DISCORD_API_URL"))
}