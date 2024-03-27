use types::{Metadata, STATE};
use utils::{errors::CassandraError, memory::Cbor, set_custom_panic_hook};

mod memory;
mod methods;
mod migrations;
mod types;
mod utils;

#[ic_cdk::init]
fn init(
    key_name: String,
    orally_wrapper: String,
    google_oauth_client_id: String,
    google_oauth_client_secret: String,
    google_oauth_redirect_url: String,
    github_oauth_client_id: String,
    github_oauth_client_secret: String,
) {
    set_custom_panic_hook();

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state
            .metadata
            .set(Cbor(Metadata {
                key_name,
                orally_wrapper,
                google_oauth_client_id,
                google_oauth_client_secret,
                google_oauth_redirect_url,
                github_oauth_client_id,
                github_oauth_client_secret,
            }))
            .unwrap();
    });
}

// For candid file auto-generation
pub type Result<T> = std::result::Result<T, CassandraError>;
use self::types::UpdateMetadata;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use types::auth_response::AuthResponse;

// Candid file auto-generation
candid::export_service!();

/// Not a test, but a helper function to save the candid file
#[cfg(test)]
mod save_candid {

    use super::*;

    fn export_candid() -> String {
        __export_service()
    }

    #[test]
    fn update_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("src")
            .join("cassandra");
        println!("{}", dir.to_str().unwrap());
        write(dir.join("cassandra.did"), export_candid()).expect("Write failed.");
    }
}
