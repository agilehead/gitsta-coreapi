use crate::actions;

pub fn githost_get_provider_sync() -> Result<String, String> {
    return Ok(r#""git.gitsta.com""#.to_owned());
}
