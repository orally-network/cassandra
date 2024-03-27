use crate::types::UpdateMetadata;
use crate::types::{Metadata, STATE};
use crate::utils::canister::validate_caller;
use crate::utils::memory::Cbor;
use candid::candid_method;
use ic_cdk::{query, update};

use crate::Result;

#[candid_method]
#[query]
fn get_metadata() -> Metadata {
    STATE.with(|s| s.borrow().metadata.get().0.clone())
}

#[candid_method]
#[update]
fn update_metadata(update_metadata_args: UpdateMetadata) -> Result<()> {
    validate_caller()?;
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let mut metadata = state.metadata.get().0.clone();
        metadata.update(update_metadata_args);
        state.metadata.set(Cbor(metadata)).unwrap();
    });

    Ok(())
}
