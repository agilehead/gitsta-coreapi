pub mod git;
pub mod githost;

use std::sync::Mutex;
use tokio::{runtime::Runtime};

pub type ActionCallback = Box<dyn Fn(String) -> ()>;

pub struct Callbacks {
    pub ok: ActionCallback,
    pub err: ActionCallback,
    pub callback: ActionCallback,
}


pub fn run_action(action: &str, args: &str, runtime: &Mutex<Runtime>, callbacks: Callbacks) {
    /*
        Run the task on a threadpool thread and block.

        We'd have preferred not blocking, but Android JVM can't handle callbacks from arbitrary threads. But still it isn't too bad. We're being called by Java threadpool threads.

        A future update could be to attach the threadpool threads to the JVM on Android.
        This can potentially avoid blocking on the action.
    */
    let action_result = runtime.lock().unwrap().block_on(async {
        handle(action, args, &callbacks.callback)
    });
}

pub async fn handle(action: &str, args: &str, callback: &ActionCallback) -> Result<String, String> {
    let result = git::handle(action, args, callback)
        .await
        .or(githost::handle(action, args, callback).await);
    handle_impl(action, args, result)
}

pub fn handle_sync(action: &str, args: &str) -> Result<String, String> {
    let result = git::handle_sync(action, args).or(githost::handle_sync(action, args));
    handle_impl(action, args, result)
}

fn handle_impl(
    action: &str,
    args: &str,
    result: Option<Result<String, String>>,
) -> Result<String, String> {
    match result {
        Some(Ok(result_success)) => Ok(format!(
            "{{ ok: true, result: {result} }}",
            result = result_success
        )),
        Some(Err(err)) => Err(format!("{{ error: {err}.\" }}", err = err)),
        None => Err(format!(
            "{{ ok: false, error: \"Missing action {action}.\" }}",
            action = action
        )),
    }
}
