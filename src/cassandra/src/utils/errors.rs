use candid::CandidType;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug, CandidType, Deserialize)]
pub enum CassandraError {
    #[error("Auth error: {0}")]
    AuthError(#[from] AuthError),
    #[error("Utils error: {0}")]
    UtilsError(#[from] UtilsError),
}

#[derive(CandidType, Deserialize, Debug, Error)]
pub enum AuthError {
    #[error("Failed to sign: {0}")]
    FailedToSign(String),
    #[error("Failed to get user info: {0}")]
    FailedToGetUserInfo(String),
    #[error("Utils error: {0}")]
    UtilsError(#[from] UtilsError),
    #[error("Invalid jwt: {0}")]
    InvalidJwt(String),
    #[error("Failed to exchange token: The supplied authorization code is invalid or in the wrong format.")]
    FailedToExchangeToken,
    #[error("Http error: {0}")]
    HttpError(String),
}

#[derive(Error, Debug, CandidType, PartialEq, Deserialize)]
pub enum UtilsError {
    #[error("Invalid address format: {0}")]
    InvalidAddressFormat(String),
    #[error("Invalid signature format")]
    InvalidSignatureFormat,
    #[error("From hex error: {0}")]
    FromHexError(String),
    #[error("Failed to get cassandra evm address: {0}")]
    FailedtoGetCassandraEVMAddress(String),
    #[error("Not a controller")]
    NotAController,
}
