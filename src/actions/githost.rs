pub mod account;
pub mod provider;
use crate::actions::{ActionResult, SendActionResult};

pub fn handle_async(
    action: &str,
    args: &str,
    send: &SendActionResult,
) -> bool {
    match action {
        "githost_check_username_availability" => {
            account::githost_check_username_availability(args, send);
            true
        }
        _ => false,
    }
}

pub fn handle_sync(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        "githost_get_provider_sync" => Some(provider::githost_get_provider_sync()),
        _ => None,
    }
}
