use crate::actions;
use tokio::sync::mpsc::UnboundedSender;

pub fn githost_get_provider_sync() -> Result<String, String> {
    Ok(r#""git.gitsta.com""#.to_owned())
}
