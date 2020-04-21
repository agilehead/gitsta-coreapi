pub mod clone;
use crate::actions::{AsyncAction, SyncAction, AsyncActionResultSend};
use std::future::Future;

pub fn get_async_handler<'a>(
    action: &str,
) -> Option<Box<AsyncAction>> {
    match action {
        "clone_over_http" => Some(Box::new(|arg: &str, send: &AsyncActionResultSend| {
            Box::new(clone::clone_over_http(arg, send))
        })),
        _ => None,
    }
}

pub fn get_sync_handler<'a>(action: &str) -> Option<Box<SyncAction>> {
    match action {
        _ => None,
    }
}
