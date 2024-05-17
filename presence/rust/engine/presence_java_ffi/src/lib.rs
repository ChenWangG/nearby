//! JNI bindings for the presence_core crate.
//!
extern crate jni;

use jni::objects::{JClass, JString, JValue};
use jni::sys::{_jobject, jint, jlong, jobject};
use jni::JNIEnv;

pub struct PresenceTestEngine {
    id: i32,
}

/// JNI bindings for `PresenceEngineNew` method in `com.google.nearby.presence.engine`.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_PresenceEngineNew(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jlong {
    let engine = PresenceTestEngine { id: 101 };
    Box::into_raw(Box::new(engine)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_PresenceEngineDebug(
env: JNIEnv,
_class: JClass,
engine: jlong) {
    unsafe {
        let engine_ptr = engine as *mut PresenceTestEngine;
        println!("Engine id {}", (*engine_ptr).id);
    }
}