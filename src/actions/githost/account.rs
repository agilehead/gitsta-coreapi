use crate::actions::{ActionResult, ActionResultSend};

pub async fn githost_check_username_availability<'a>(args: &'a str, send: &'a ActionResultSend) {
    let result = boom(args).await;
    //send(ActionResult::Result(result));
}

async fn boom(username: &str) -> Result<String, String> {
    return match username {
        "admin" => Err("false".to_owned()),
        _ => Ok("true".to_owned()),
    };
}
