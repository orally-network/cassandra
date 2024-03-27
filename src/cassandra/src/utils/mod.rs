use crate::log;

pub mod canister;
pub mod encoding;
pub mod errors;
pub mod http;
pub mod macros;
pub mod memory;
pub mod nat;
pub mod signature;
pub mod time;

pub fn set_custom_panic_hook() {
    _ = std::panic::take_hook(); // clear custom panic hook and set default
    let old_handler = std::panic::take_hook(); // take default panic hook

    // set custom panic hook
    std::panic::set_hook(Box::new(move |info| {
        log!("PANIC OCCURRED: {:#?}", info);
        old_handler(info);
    }));
}
