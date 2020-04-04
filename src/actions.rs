pub mod git;
pub mod githost;

pub async fn handle(action: &str, args: &str) -> Result<String, String> {
    let result = git::handle(action, args)
        .await
        .or(githost::handle(action, args).await);
    handle_impl(action, args, result)
}

pub fn handle_sync(action: &str, args: &str) -> Result<String, String> {
    let result = git::handle_sync(action, args).or(githost::handle_sync(action, args));
    handle_impl(action, args, result)
}

fn handle_impl(
    action: &str,
    args: &str,
    result: Option<Result<String, String>>,
) -> Result<String, String> {
    match result {
        Some(Ok(result_success)) => Ok(format!(
            "{{ ok: true, result: {result} }}",
            result = result_success
        )),
        Some(Err(err)) => Err(format!("{{ error: {err}.\" }}", err = err)),
        None => Err(format!(
            "{{ ok: false, error: \"Missing action {action}.\" }}",
            action = action
        )),
    }
}
