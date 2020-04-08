pub mod git;
pub mod githost;

use std::sync::Mutex;
use tokio::{runtime::Runtime, sync::mpsc};
use std::future::Future;

pub type ActionCallback = Box<dyn Fn(String) -> ()>;

pub enum ActionResult {
    Result(Result<String, String>),
    Callback(String),
}

pub type SendActionResult = dyn Fn(ActionResult) -> ();

pub type Action = dyn Fn(&str, &SendActionResult) -> dyn Future<Output=()>;

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

    We'd have preferred not blocking, but Android JVM can't handle callbacks from arbitrary threads.
    But still it isn't too bad. We're being called by Java threadpool threads.

    A future update could be to attach the threadpool threads to the JVM on Android.
    This can potentially avoid blocking on the action.
*/

pub async fn handle_async(
    action: &str,
    args: &str,
    callbacks: Callbacks,
    runtime: &Mutex<Runtime>,
) {
    let maybe_action_handler = git::get_async_handler(action).or(githost::get_async_handler(action));

    match maybe_action_handler {
        Some(Action) => {
            let (tx, mut rx) = mpsc::unbounded_channel::<ActionResult>();

            let send = |result: ActionResult| ();
            let boxed_send: Box<dyn Fn(ActionResult) -> ()> = Box::new(send);

            let found_handler = loop {
                let msg = rx.recv().await;
                match msg {
                    Some(ActionResult::Result(Ok(msg_txt))) => {
                        (callbacks.ok)(msg_txt);
                        break true;
                    },
                    Some(ActionResult::Result(Err(msg_txt))) => {
                        (callbacks.err)(msg_txt);
                        break true;
                    },
                    Some(ActionResult::Callback(msg_txt)) => {
                        (callbacks.callback)(msg_txt);
                    },
                    None => {
                        break false;
                    }
                }
            };
        },
        None => {
            (callbacks.err)(format!(
                "{{ ok: false, error: \"The sync action {action} was unhandled.\" }}",
                action = action
            ));
        }
    }

    // tokio::spawn(move  || {
    //     let found_handler = git::handle_async(action, args, &boxed_send)
    //         || githost::handle_async(action, args, &boxed_send);
    //     if (!found_handler) {
    //         tx.send(ActionResult::NotFound);
    //     }
    // });


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
