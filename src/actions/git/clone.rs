use git2::Repository;

struct Repo<'a> {
  username: &'a str,
  password: &'a str,
  url: &'a str,
}

pub async fn clone_over_http<'a>(args: &str) -> Result<String, String> {
  // let url = "https://github.com/alexcrichton/git2-rs";
  // let repo = Repository::clone(url, "/path/to/a/repo");
  
  Ok(r#"{ result: true }"#.to_owned())
}
