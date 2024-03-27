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
use jwt::{Header, Token, Unverified};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{retry_until_success, utils::http::transform_ctx};

const NAME_CLAIM: &str = "name";
const EMAIL_CLAIM: &str = "email";
const AVATAR_CLAIM: &str = "picture";
const COMPANY_CLAIM: &str = "hd";
const USER_ID_CLAIM: &str = "sub";

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
    scope: String,
    id_token: String,
    refresh_token: Option<String>,
}

/// Google OAuth2.0
/// Accepts an authorization code and exchanges it for an access token and id token
#[candid_method]
#[update]
async fn google_auth(authorization_code: String) -> crate::Result<AuthResponse> {
    canister::validate_caller()?;
    let token_response = exchange_token(authorization_code).await?;

    let jwt = get_jwt(&token_response.id_token)?;
    let claims = jwt.claims();

    let user_id = claims
        .get(USER_ID_CLAIM)
        .expect("should have user id claim")
        .as_str()
        .expect("all users should have user id")
        .parse()
        .unwrap();

    let name = claims
        .get(NAME_CLAIM)
        .expect("should have name claim")
        .as_str()
        .unwrap_or_default() // Not all users have set a name in their Google account
        .to_string();
    let email = claims
        .get(EMAIL_CLAIM)
        .expect("should have email claim")
        .as_str()
        .expect("all users should have email")
        .to_string();
    let avatar = claims
        .get(AVATAR_CLAIM)
        .expect("should have avatar claim")
        .as_str()
        .unwrap_or_default() // Not all users have an avatar
        .to_string();
    let company = claims
        .get(COMPANY_CLAIM)
        .expect("should have company claim")
        .as_str()
        .unwrap_or_default() // Not all users have a company domain
        .to_string();

    let auth_method = AuthMethod::Google;

    Ok(AuthResponse::new(user_id, name, email, avatar, company, auth_method).await?)
}

async fn exchange_token(authorization_code: String) -> Result<TokenResponse, AuthError> {
    // TODO: Move these to env variables
    let redirect_url = get_metadata!(google_oauth_redirect_url);
    let client_id = get_metadata!(google_oauth_client_id);
    let client_secret = get_metadata!(google_oauth_client_secret);

    let params = format!(
        "grant_type={}&redirect_uri={}&client_id={}&code={}&client_secret={}",
        "authorization_code",
        urlencoding::encode(&redirect_url),
        urlencoding::encode(&client_id),
        urlencoding::encode(&authorization_code),
        urlencoding::encode(&client_secret)
    );
    let root_url = format!("https://oauth2.googleapis.com/token?{}", params);

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

    let token_response = serde_json::from_str::<TokenResponse>(&response_str)
        .map_err(|_| AuthError::FailedToExchangeToken)?;

    Ok(token_response)
}

fn get_jwt(access_token: &str) -> Result<Token<Header, Value, Unverified>, AuthError> {
    let jwt_token = Token::parse_unverified(&access_token)
        .map_err(|err| AuthError::InvalidJwt(err.to_string()))?;

    Ok(jwt_token)
}
