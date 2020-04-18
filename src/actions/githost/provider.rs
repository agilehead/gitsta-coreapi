use crate::actions;

pub fn githost_get_provider_sync(args: &str) -> Result<String, String> {
    Ok(r#""git.gitsta.com""#.to_owned())
}
