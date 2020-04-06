pub mod clone;
use tokio::sync::mpsc::UnboundedSender;
use crate::actions::ActionResult;


pub fn handle_async(
    action: &str,
    args: &str,
    tx: &UnboundedSender<ActionResult>,
) -> bool {
    match action {
        "clone_over_http" => {
            clone::clone_over_http(args, tx);
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
