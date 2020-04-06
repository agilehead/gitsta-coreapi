pub mod git;
pub mod githost;

use std::sync::Mutex;
use tokio::{runtime::Runtime, sync::mpsc};

pub type ActionCallback = Box<dyn Fn(String) -> ()>;

pub enum ActionResult {
    Result(Result<String, String>),
    Callback(String),
}

pub type SendActionResult = Box<dyn Fn(ActionResult) -> ()>;

pub struct Callbacks {
    pub ok: ActionCallback,
    pub err: ActionCallback,
    pub callback: ActionCallback,
}

/*
    Async actions can do one of three things.
    1. Return an Ok(result). This closes the callback context and no further responses are allowed.
    2. Return an Err(err).. This closes the callback context and no further responses are allowed.
    3. Return a Callback(data). This doesn't close the channel.
*/
pub async fn handle_async(
    action: &str,
    args: &str,
    runtime: &Mutex<Runtime>,
    callbacks: Callbacks,
) {
    let (tx, mut rx) = mpsc::unbounded_channel::<ActionResult>();

    let send = |result: ActionResult| ();
    let boxed_send: Box<dyn Fn(ActionResult) -> ()> = Box::new(send);

    let found_handler = git::handle_async(action, args, &boxed_send)
        || githost::handle_async(action, args, &boxed_send);

    if (found_handler) {
        loop {
            let msg = rx.recv().await;
            match msg {
                Some(ActionResult::Result(Ok(msg_txt))) => {
                    (callbacks.ok)(msg_txt);
                    break;
                }
                Some(ActionResult::Result(Err(msg_txt))) => {
                    (callbacks.err)(msg_txt);
                    break;
                }
                Some(ActionResult::Callback(msg_txt)) => {
                    (callbacks.callback)(msg_txt);
                }
                None => {
                    break;
                }
            }
        }
    } else {
        (callbacks.err)(format!(
            "{{ ok: false, error: \"The sync action {action} was unhandled.\" }}",
            action = action
        ));
    }
}

/*
    Sync actions can do one of three things.
    1. Return an Ok(result). This closes the callback context and no further responses are allowed.
    2. Return an Err(err).. This closes the callback context and no further responses are allowed.
*/
pub fn handle_sync(action: &str, args: &str) -> Result<String, String> {
    let result = git::handle_sync(action, args).or(githost::handle_sync(action, args));
    match result {
        Some(Ok(result_success)) => Ok(format!(
            "{{ ok: true, result: {result} }}",
            result = result_success
        )),
        Some(Err(err)) => Err(format!("{{ error: {err}.\" }}", err = err)),
        None => Err(format!(
            "{{ ok: false, error: \"The sync action {action} was unhandled.\" }}",
            action = action
        )),
    }
}
