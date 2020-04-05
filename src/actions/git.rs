pub mod clone;
use crate::actions;

pub async fn handle(action: &str, args: &str, callback: &actions::ActionCallback) -> Option<Result<String, String>> {
    match action {
        "clone_over_http" => Some(clone::clone_over_http(args, callback).await),
        _ => None,
    }
}

pub fn handle_sync(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        _ => None,
    }
}
