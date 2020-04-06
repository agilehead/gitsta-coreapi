pub mod clone;
use crate::actions::{ActionResult, SendActionResult};


pub fn handle_async(
    action: &str,
    args: &str,
    send: &SendActionResult,
) -> bool {
    match action {
        "clone_over_http" => {
            clone::clone_over_http(args, send);
            true
        },
        _ => false,
    }
}

pub fn handle_sync(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        _ => None,
    }
}
