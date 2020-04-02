pub mod git;
pub mod githost;

pub async fn handle(action: &str, args: &str) -> Result<String, String> {
  let result = git::handle(action, args)
    .await
    .or(githost::handle(action, args).await);
  match result {
    Some(result) => result,
    None => Err(format!(
      "{{ error: \"Missing action {action}.\" }}",
      action = action
    )),
  }
}
