use candid::CandidType;
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId, SignWithEcdsaArgument};
use ic_web3_rs::{ethabi::Token, ic::get_eth_addr, signing::keccak256};
use serde::{Deserialize, Serialize};

use crate::{
    get_metadata,
    utils::{
        encoding::encode_packed,
        errors::{AuthError, UtilsError},
        signature::{get_eth_v, sign},
    },
};

#[derive(Serialize, Deserialize, Debug, CandidType, Clone)]
pub struct AuthResponse {
    pub user_id: u128,
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub company: String,
    pub auth_method: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone)]
pub enum AuthMethod {
    GitHub,
    Google,
}

impl ToString for AuthMethod {
    fn to_string(&self) -> String {
        match self {
            AuthMethod::GitHub => "github".to_string(),
            AuthMethod::Google => "google".to_string(),
        }
    }
}

impl AuthResponse {
    pub async fn new(
        user_id: u128,
        name: String,
        email: String,
        avatar: String,
        company: String,
        auth_method: AuthMethod,
    ) -> Result<Self, AuthError> {
        let encode_packed = encode_packed(&[
            Token::Uint(user_id.into()),
            Token::String(name.clone()),
            Token::String(email.clone()),
            Token::String(avatar.clone()),
            Token::String(company.clone()),
            Token::String(auth_method.to_string()),
        ])
        .expect("should encode packed");

        let sign_data = keccak256(&encode_packed);
        let key_name = get_metadata!(key_name);

        let derivation_path = vec![ic_cdk::id().as_slice().to_vec()];

        let call_args = SignWithEcdsaArgument {
            message_hash: sign_data.to_vec(),
            derivation_path: derivation_path.clone(),
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name.clone(),
            },
        };

        let mut signature = sign(call_args)
            .await
            .map_err(|(_, msg)| AuthError::FailedToSign(msg))?
            .0
            .signature;

        let pub_key = get_eth_addr(None, Some(derivation_path), key_name)
            .await
            .map_err(|err| UtilsError::FailedtoGetCassandraEVMAddress(err))?;

        signature.push(get_eth_v(&signature, &sign_data, &pub_key)?);

        Ok(Self {
            user_id,
            name,
            email,
            avatar,
            company,
            auth_method: auth_method.to_string(),
            signature: hex::encode(&signature),
        })
    }
}
