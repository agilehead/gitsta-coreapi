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
use tokio::{runtime::Runtime, sync::mpsc};

mod actions;

/*
    This class leaks like titanic.
    1. Release all the nativeside strings
    2. Release the Java Context instance once async action is complete.
*/

struct JMethodPtr {
    ptr: *mut c_void,
}

unsafe impl Send for JMethodPtr {}

lazy_static! {
    static ref RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new().unwrap());
    static ref SUCCESS_METHODID: Mutex<JMethodPtr> = Mutex::new(JMethodPtr {
        ptr: 0 as *mut c_void
    });
    static ref ERROR_METHODID: Mutex<JMethodPtr> = Mutex::new(JMethodPtr {
        ptr: 0 as *mut c_void
    });
    static ref CALLBACK_METHODID: Mutex<JMethodPtr> = Mutex::new(JMethodPtr {
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

    cache_method_id(&env, "success", &SUCCESS_METHODID);
    cache_method_id(&env, "error", &ERROR_METHODID);
    cache_method_id(&env, "callback", &CALLBACK_METHODID);
}

fn cache_method_id(env: &JNIEnv, method_name: &str, mutex: &Mutex<JMethodPtr>) {
    let method_id = env
        .get_method_id(
            "com/gitsta/gitstaapp/plugins/GitstaCoreApiBridge/GitstaCoreApiCallContext",
            method_name,
            "(Ljava/lang/String;)V",
        )
        .unwrap();

    let mut method_id_ptr = mutex.lock().unwrap();
    *method_id_ptr = JMethodPtr {
        ptr: method_id.into_inner() as *mut c_void,
    };
}

/*
    Handles async calls from the app.
    Sends tasks to the event loop created during init via mpsc channels.

    We convert the args into a string and push that to the args channel, along with the global ref of the passed in 'this' pointer. The event loop thread will pick it up from the channel.
*/
#[no_mangle]
pub unsafe extern "C" fn Java_com_gitsta_gitstaapp_plugins_GitstaCoreApiBridge_GitstaCoreApiCallContext_coreapicall(
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

    let success_method_id = JMethodID::from(SUCCESS_METHODID.lock().unwrap().ptr as jmethodID);
    let error_method_id = JMethodID::from(ERROR_METHODID.lock().unwrap().ptr as jmethodID);
    let callback_method_id = JMethodID::from(CALLBACK_METHODID.lock().unwrap().ptr as jmethodID);

    // TODO: Need to drop this.
    let _gref_thiz = env.new_global_ref(thiz);

    let ret = JavaType::Primitive(Primitive::Void);

    // let create_java_cb = move |method_id: JMethodID| {
    //     move |result: String| {
    //         let ret = JavaType::Primitive(Primitive::Void);
    //         let cb_arg = env.new_string(result).unwrap();
    //         let _result = env.call_method_unchecked(
    //             thiz,
    //             method_id,
    //             ret,
    //             &[JValue::Object(JObject::from(cb_arg))],
    //         );
    //     }
    // };

    let callbacks = actions::Callbacks {
        ok: Box::new(|x: String| ()),
        err: Box::new(|x: String| ()),
        callback: Box::new(|x: String| ()),
    };

    actions::run_action(str_action, str_args, &RUNTIME, callbacks);

    // spawn the task on a threadpool thread
    // let action_result = RUNTIME
    //     .lock()let action_result = RUNTIME
    //     .lock()
    //     .unwrap()
    //     .unwrap()
    //     .block_on(async { actions::handle(str_action, str_args).await });

    // match action_result {
    //     Ok(result) => {
    //         let cb_arg = env.new_string(result).unwrap();
    //         let _result = env.call_method_unchecked(
    //             thiz,
    //             success_method_id,
    //             ret,
    //             &[JValue::Object(JObject::from(cb_arg))],
    //         );
    //     }
    //     Err(err) => {
    //         let cb_arg = env.new_string(err).unwrap();
    //         let _result = env.call_method_unchecked(
    //             thiz,
    //             error_method_id,
    //             ret,
    //             &[JValue::Object(JObject::from(cb_arg))],
    //         );
    //     }
    // };

    true
}

/*
    coreapicallsync:
    Handles sync calls from the app. Does not use threads. The caller is not expected to use a threadpool either.
*/
#[no_mangle]
pub unsafe extern "C" fn Java_com_gitsta_gitstaapp_plugins_GitstaCoreApiBridge_GitstaCoreApiBridgePlugin_coreapicallsync(
    env: JNIEnv,
    _: JObject,
    action: JString,
    args: JString,
) -> jstring {
    // Get a native string from the passed in JString
    let raw_action = env.get_string(action).unwrap().as_ptr();
    let native_action = CString::from(CStr::from_ptr(raw_action));
    let str_action = native_action.to_str().unwrap();

    let raw_input_ptr = env.get_string(args).unwrap().as_ptr();
    let native_args = CString::from(CStr::from_ptr(raw_input_ptr));
    let str_args = native_args.to_str().unwrap();

    let action_result = actions::handle_sync(str_action, str_args);

    // Result may be an error

    let result = match action_result {
        Ok(result) => result,
        Err(err) => err,
    };

    let output = env.new_string(result).unwrap();
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
