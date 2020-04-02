pub mod clone;

pub async fn handle(action: &str, args: &str) -> Option<Result<String, String>> {
  match action {
    "clone_over_http" => Some(clone::clone_over_http(args).await),
    _ => None
  }
}
