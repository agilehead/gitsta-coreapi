pub mod clone;
use crate::actions::Action;

pub fn get_async_handler(action: &str) -> Option<&Action> {
    match action {
        "clone_over_http" => Some(Box::new(|arg: &str| {
            Box::new(clone::clone_over_http(arg))
        })),
        _ => None,
    }
}

pub fn handle_sync(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        _ => None,
    }
}
