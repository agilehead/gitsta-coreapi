pub mod account;
pub mod provider;
use crate::actions::{AsyncAction, SyncAction, AsyncActionResultSend};
use std::future::Future;

pub fn get_async_handler<'a>(
    action: &str,
) -> Option<Box<AsyncAction>> {
    match action {
        "githost_check_username_availability" => {
            Some(Box::new(|arg: &str, send: &AsyncActionResultSend| {
                Box::new(account::githost_check_username_availability(arg, send))
            }))
        }
        _ => None,
    }
}

pub fn get_sync_handler<'a>(action: &'a str) -> Option<Box<SyncAction>> {
    match action {
        "githost_get_provider_sync" => Some(Box::new(provider::githost_get_provider_sync)),
        _ => None,
    }
}
