pub mod account;
pub mod provider;
use crate::actions::{Action, ActionResultSend};
use std::future::Future;

pub fn get_async_handler<'a>(
    action: &str,
) -> Option<Box<Action>> {
    match action {
        "githost_check_username_availability" => {
            Some(Box::new(|arg: &str, send: &ActionResultSend| {
                Box::new(account::githost_check_username_availability(arg, send))
            }))
        }
        _ => None,
    }
}

pub fn handle_sync(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        "githost_get_provider_sync" => Some(provider::githost_get_provider_sync()),
        _ => None,
    }
}
