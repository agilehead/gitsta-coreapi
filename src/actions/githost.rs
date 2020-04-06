pub mod account;
pub mod provider;
use tokio::sync::mpsc::UnboundedSender;
use crate::actions::ActionResult;

pub fn handle_async(
    action: &str,
    args: &str,
    tx: &UnboundedSender<ActionResult>,
) -> bool {
    match action {
        "githost_check_username_availability" => {
            account::githost_check_username_availability(args, tx);
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
