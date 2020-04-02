pub mod account;

pub async fn handle(action: &str, args: &str) -> Option<Result<String, String>> {
  match action {
    "check_username_availability" => Some(account::check_username_availability(args).await),
    _ => None,
  }
}
