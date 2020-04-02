pub async fn check_username_availability<'a>(username: &'a str) -> Result<String, String> {
  //return r#"{ action: "check_username_availability", result: true }"#;
  return boom(username).await;
}

async fn boom<'a>(username: &'a str) -> Result<String, String> {
  return Ok(r#"{ action: "check_username_availability", result: true }"#.to_owned());
}
