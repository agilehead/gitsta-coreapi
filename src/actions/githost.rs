pub mod account;
pub mod provider;
use crate::actions::Action;

pub fn get_async_handler(action: &str) -> Option<&Action> {
    match action {
        // "githost_check_username_availability" => Some(account::githost_check_username_availability),
        _ => None,
    }
}

pub fn handle_sync(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        "githost_get_provider_sync" => Some(provider::githost_get_provider_sync()),
        _ => None,
    }
}
