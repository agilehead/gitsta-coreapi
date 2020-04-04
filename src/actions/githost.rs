pub mod account;
pub mod provider;

pub async fn handle(action: &str, args: &str) -> Option<Result<String, String>> {
    match action {
        "githost_check_username_availability" => {
            Some(account::githost_check_username_availability(args).await)
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
