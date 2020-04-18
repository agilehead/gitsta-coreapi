pub mod clone;
use crate::actions::{Action, ActionResultSend};
use std::future::Future;

pub fn get_async_handler<'a>(
    action: &str,
) -> Option<Box<Action>> {
    match action {
        "clone_over_http" => Some(Box::new(|arg: &str, send: &ActionResultSend| {
            Box::new(clone::clone_over_http(arg, send))
        })),
        _ => None,
    }
}

pub fn handle_sync(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        _ => None,
    }
}
