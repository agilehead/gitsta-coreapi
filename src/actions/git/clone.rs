use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use std::path::{Path, PathBuf};

struct Repo<'a> {
  username: &'a str,
  password: &'a str,
  url: &'a str,
}

pub async fn clone_over_http<'a>(args: &str) -> Result<String, String> {
  let url = "https://github.com/alexcrichton/git2-rs";
  //let repo = Repository::clone(url, "/home/jeswin/temp/lalala");
  
  let mut co = CheckoutBuilder::new();
  let mut fo = FetchOptions::new();
  //fo.remote_callbacks(cb);
  RepoBuilder::new()
      .fetch_options(fo)
      .with_checkout(co)
      //.clone(&args.arg_url, Path::new(&args.arg_path))?;
      .clone(url, Path::new("/sdcard/boomer"));

  Ok(r#"{ result: true, checked_out: 2 }"#.to_owned())
}
