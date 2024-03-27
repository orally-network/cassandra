mod canister;
mod github_auth;
mod google_auth;

/// Check [Gas and cycles cost](https://internetcomputer.org/docs/current/developer-docs/gas-cost#special-features)] for formula
const NODES_IN_SUBNET: u128 = 34; // Not a default value, but bigger - better
const MAX_RESPONSE_BYTES: u128 = 2 * 1024 * 1024; // 2MB
const CYCLES_FOR_RESPONSE: u128 = MAX_RESPONSE_BYTES * 800 * NODES_IN_SUBNET;
const BASE_CYCLES: u128 = (3_000_000 + 60_000 * NODES_IN_SUBNET) * NODES_IN_SUBNET;
const HTTP_CYCLES: u128 = BASE_CYCLES + CYCLES_FOR_RESPONSE;
