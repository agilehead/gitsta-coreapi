use crate::actions;

pub async fn githost_check_username_availability<'a>(
    username: &'a str,
    callback: &actions::ActionCallback,
) -> Result<String, String> {
    return boom(username).await;
}

async fn boom<'a>(username: &'a str) -> Result<String, String> {
    return match username {
        "admin" => Err("false".to_owned()),
        _ => Ok("true".to_owned()),
    };
}
