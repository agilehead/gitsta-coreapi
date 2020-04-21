use crate::actions::{AsyncActionResult, AsyncActionResultSend};

pub async fn githost_check_username_availability<'a>(args: &'a str, send: &'a AsyncActionResultSend) {
    let result = boom(args).await;
    //send(ActionResult::Result(result));
}

async fn boom(username: &str) -> Result<String, String> {
    return match username {
        "admin" => Err("false".to_owned()),
        _ => Ok("true".to_owned()),
    };
}
