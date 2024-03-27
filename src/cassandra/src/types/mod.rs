use std::cell::RefCell;

use candid::CandidType;
use ic_stable_structures::StableCell;
use serde::{Deserialize, Serialize};

use crate::{memory::VMemory, utils::memory::Cbor};

pub mod auth_response;

#[derive(Serialize, Deserialize, Debug, Default, CandidType, Clone)]
pub struct Metadata {
    pub key_name: String,

    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,

    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
}

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct UpdateMetadata {
    pub key_name: Option<String>,

    pub google_oauth_client_id: Option<String>,
    pub google_oauth_client_secret: Option<String>,
    pub google_oauth_redirect_url: Option<String>,

    pub github_oauth_client_id: Option<String>,
    pub github_oauth_client_secret: Option<String>,
}

impl Metadata {
    pub fn update(&mut self, update: UpdateMetadata) {
        if let Some(key_name) = update.key_name {
            self.key_name = key_name;
        }

        if let Some(google_oauth_client_id) = update.google_oauth_client_id {
            self.google_oauth_client_id = google_oauth_client_id;
        }
        if let Some(google_oauth_client_secret) = update.google_oauth_client_secret {
            self.google_oauth_client_secret = google_oauth_client_secret;
        }
        if let Some(google_oauth_redirect_url) = update.google_oauth_redirect_url {
            self.google_oauth_redirect_url = google_oauth_redirect_url;
        }

        if let Some(github_oauth_client_id) = update.github_oauth_client_id {
            self.github_oauth_client_id = github_oauth_client_id;
        }
        if let Some(github_oauth_client_secret) = update.github_oauth_client_secret {
            self.github_oauth_client_secret = github_oauth_client_secret;
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_metadata")]
    pub metadata: StableCell<Cbor<Metadata>, VMemory>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

fn init_metadata() -> StableCell<Cbor<Metadata>, VMemory> {
    let metadata = Cbor(Metadata::default());
    StableCell::init(crate::memory::get_metadata_memory(), metadata).unwrap()
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_metadata(),
        }
    }
}
