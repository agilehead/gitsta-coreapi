#![cfg(target_os = "android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;

use jni::objects::{JMethodID, JObject, JString, JValue};
use jni::signature::{JavaType, Primitive};
use jni::sys::jmethodID;
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use tokio::runtime::Runtime;

mod actions;

struct JMethodPtr {
    ptr: *mut c_void,
}

unsafe impl Send for JMethodPtr {}

lazy_static! {
    static ref RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new().unwrap());
    static ref METHODID: Mutex<JMethodPtr> = Mutex::new(JMethodPtr {
        ptr: 0 as *mut c_void
    });
}

/*
    Called when the plugin initializes.
    Runs async tasks on a thread pool. Tasks are queued via mpsc channels.
*/
#[no_mangle]
pub unsafe extern "C" fn Java_com_gitsta_gitstaapp_plugins_GitstaCoreApiBridge_GitstaCoreApiBridgePlugin_coreapiinit(
    env: JNIEnv,
    _: JObject,
) {
    // I read somewhere that the method_id below is only valid if the class is not unloaded. So we're gonna hold on to the class though we don't need it.
    let clazz = env
        .find_class("com/gitsta/gitstaapp/plugins/GitstaCoreApiBridge/GitstaCoreApiCallContext")
        .unwrap();

    // We don't ever need to drop this.
    let _gref_clazz = env.new_global_ref(*clazz);

    // Gotta cache this.
    let method_id = env
        .get_method_id(
            "com/gitsta/gitstaapp/plugins/GitstaCoreApiBridge/GitstaCoreApiCallContext",
            "callback",
            "(Ljava/lang/String;)V",
        )
        .unwrap();

    let mut method_id_ptr = METHODID.lock().unwrap();
    *method_id_ptr = JMethodPtr {
        ptr: method_id.into_inner() as *mut c_void,
    };
}

/*
    Handles sync calls from the app.
    Does not use the threadpool.
*/
#[no_mangle]
pub unsafe extern "C" fn Java_com_gitsta_gitstaapp_plugins_GitstaCoreApiBridge_GitstaCoreApiBridgePlugin_coreapicall(
    env: JNIEnv,
    _: JObject,
    action: JString,
    args: JString,
) -> jstring {
    let raw_action = env.get_string(action).unwrap().as_ptr();
    let native_action = CString::from(CStr::from_ptr(raw_action));
    let str_action = native_action.to_str().unwrap();
    
    let raw_input_ptr = env.get_string(args).unwrap().as_ptr();
    let native_args = CString::from(CStr::from_ptr(raw_input_ptr));
    let str_args = native_args.to_str().unwrap();

    let output = env
        .new_string("sync-call:".to_owned() + str_args)
        .unwrap();

    output.into_inner()
}

/*
    Free strings allocated on the native end.
*/
#[no_mangle]
pub extern "C" fn Java_com_gitsta_gitstaapp_plugins_GitstaCoreApiBridge_GitstaCoreApiCallContext_free(
    s: *mut c_char,
) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

/*
    Handles async calls from the app.
    Sends tasks to the event loop created during init via mpsc channels.

    We convert the args into a string and push that to the args channel, along with the global ref of the passed in 'this' pointer. The event loop thread will pick it up from the channel.
*/
#[no_mangle]
pub unsafe extern "C" fn Java_com_gitsta_gitstaapp_plugins_GitstaCoreApiBridge_GitstaCoreApiCallContext_coreapicallasync(
    env: JNIEnv,
    thiz: JObject,
    action: JString,
    args: JString,
) -> bool {
    // Get a native string from the passed in JString
    let raw_action = env.get_string(action).unwrap().as_ptr();
    let native_action = CString::from(CStr::from_ptr(raw_action));
    let str_action = native_action.to_str().unwrap();

    let raw_input_ptr = env.get_string(args).unwrap().as_ptr();
    let native_args = CString::from(CStr::from_ptr(raw_input_ptr));
    let str_args = native_args.to_str().unwrap();

    let method_id = env
        .get_method_id(
            "com/gitsta/gitstaapp/plugins/GitstaCoreApiBridge/GitstaCoreApiCallContext",
            "callback",
            "(Ljava/lang/String;)V",
        )
        .unwrap();

    let method_id = JMethodID::from(METHODID.lock().unwrap().ptr as jmethodID);

    // spawn the root task
    let action_result = RUNTIME
        .lock()
        .unwrap()
        .block_on(async { 
            actions::handle(str_action, str_args).await 
        });

    // Result may be an error
    let result = match action_result {
        Ok(result) => result,
        Err(e) => format!("{{ error: {e} }}", e = e)
    };

    let ret = JavaType::Primitive(Primitive::Void);

    let cb_arg = env.new_string(result).unwrap();
    
    let _result = env.call_method_unchecked(
        thiz,
        method_id,
        ret,
        &[JValue::Object(JObject::from(cb_arg))],
    );

    true
}
