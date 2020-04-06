use crate::actions::ActionResult;
use tokio::sync::mpsc::UnboundedSender;

pub async fn githost_check_username_availability<'a>(
    username: &'a str,
    tx: &UnboundedSender<ActionResult>,
) {
    let result = boom(username).await;
    tx.send(ActionResult::Result(result));
}

async fn boom<'a>(username: &'a str) -> Result<String, String> {
    return match username {
        "admin" => Err("false".to_owned()),
        _ => Ok("true".to_owned()),
    };
}
