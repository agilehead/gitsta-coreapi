use crate::actions::{ActionResult, SendActionResult};

pub async fn githost_check_username_availability<'a>(
    username: &'a str,
    send: &SendActionResult,
) {
    let result = boom(username).await;
    send(ActionResult::Result(result));
}

async fn boom<'a>(username: &'a str) -> Result<String, String> {
    return match username {
        "admin" => Err("false".to_owned()),
        _ => Ok("true".to_owned()),
    };
}
