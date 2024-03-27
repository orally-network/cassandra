use crate::{
    get_metadata,
    methods::HTTP_CYCLES,
    types::auth_response::{AuthMethod, AuthResponse},
    utils::{canister, errors::AuthError},
};
use candid::candid_method;
use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
    },
    update,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{retry_until_success, utils::http::transform_ctx};

const USER_INFO_USER_ID: &str = "id";
const USER_INFO_NAME: &str = "name";
const USER_INFO_EMAIL: &str = "email";
const USER_INFO_AVATAR: &str = "avatar_url";
const USER_INFO_COMPANY: &str = "company";

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

/// GitHub OAuth2.0
/// Accepts an authorization code and exchanges it for an access token and id token
#[candid_method]
#[update]
async fn github_auth(authorization_code: String) -> crate::Result<AuthResponse> {
    canister::validate_caller()?;

    let token_response = exchange_token(authorization_code).await?;
    let user_info = get_user_info(token_response.access_token).await?;

    let user_id = user_info
        .get(USER_INFO_USER_ID)
        .expect("should have id")
        .as_u64()
        .expect("all users should have id")
        .into();

    let name = user_info
        .get(USER_INFO_NAME)
        .expect("should have name")
        .as_str()
        .unwrap_or_default() // Not all users have set a name in their GitHub account
        .to_string();

    let email = user_info
        .get(USER_INFO_EMAIL)
        .expect("should have email")
        .as_str()
        .unwrap_or_default() // Not all users have set an email in their GitHub account
        .to_string();

    let avatar = user_info
        .get(USER_INFO_AVATAR)
        .expect("should have avatar")
        .as_str()
        .unwrap_or_default() // Not all users have set an avatar in their GitHub account
        .to_string();

    let company = user_info
        .get(USER_INFO_COMPANY)
        .expect("should have company")
        .as_str()
        .unwrap_or_default() // Not all users have set a company in their GitHub account
        .to_string();

    let auth_method = AuthMethod::GitHub;

    let auth_response =
        AuthResponse::new(user_id, name, email, avatar, company, auth_method).await?;

    Ok(auth_response)
}

async fn exchange_token(authorization_code: String) -> Result<TokenResponse, AuthError> {
    let client_id = get_metadata!(github_oauth_client_id);
    let client_secret = get_metadata!(github_oauth_client_secret);

    let params = format!(
        "client_id={}&code={}&client_secret={}",
        urlencoding::encode(&client_id),
        urlencoding::encode(&authorization_code),
        urlencoding::encode(&client_secret)
    );

    let root_url = format!("https://github.com/login/oauth/access_token?{}", params);

    let request = CanisterHttpRequestArgument {
        method: HttpMethod::POST,
        url: root_url.to_string(),
        body: None,
        headers: vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/x-www-form-urlencoded".to_string(),
            },
            HttpHeader {
                name: "Content-Length".to_string(),
                value: "0".to_string(),
            },
        ],
        max_response_bytes: None,
        transform: Some(transform_ctx()),
    };

    let (response,) = retry_until_success!(http_request(request.clone(), HTTP_CYCLES))
        .map_err(|(_, err)| AuthError::HttpError(err))?;

    let response_str = String::from_utf8(response.body.clone()).expect("should be able to parse");

    let token_response = serde_qs::from_str::<TokenResponse>(&response_str)
        .map_err(|_| AuthError::FailedToExchangeToken)?;

    Ok(token_response)
}

async fn get_user_info(access_token: String) -> Result<Value, AuthError> {
    let root_url = "https://api.github.com/user".to_string();

    let request = CanisterHttpRequestArgument {
        method: HttpMethod::GET,
        url: root_url.to_string(),
        body: None,
        headers: vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/x-www-form-urlencoded".to_string(),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {}", access_token),
            },
        ],
        max_response_bytes: None,
        transform: Some(transform_ctx()),
    };

    let (response,) = retry_until_success!(http_request(request.clone(), HTTP_CYCLES))
        .map_err(|(_, err)| AuthError::HttpError(err))?;

    let response_str = String::from_utf8(response.body.clone()).expect("should be able to parse");

    let user_info = serde_json::from_str::<Value>(&response_str)
        .map_err(|err| AuthError::FailedToGetUserInfo(err.to_string()))?;

    Ok(user_info)
}
