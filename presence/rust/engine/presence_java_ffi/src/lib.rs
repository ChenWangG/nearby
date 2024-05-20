//! JNI bindings for the presence_core crate.
//!
extern crate jni;

use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong};
use jni::JNIEnv;

pub struct PresenceTestEngine {
    id: i32,
}

impl PresenceTestEngine {
    pub fn run(&mut self) {
        println!("Engine run!")
    }
}

/// JNI bindings for `PresenceEngineNew` method in `com.google.nearby.presence.engine`.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_presenceEngineNew(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = PresenceTestEngine { id: 101 };
    Box::into_raw(Box::new(engine)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_presenceEngineRun(
    env: JNIEnv,
    _class: JClass,
    engine: jlong,
    callbacks: JObject,
) {
    let res: jint = 32;
    env.call_method(callbacks, "onDiscovery", "(I)V", &[res.into()])
        .unwrap();
    unsafe {
        let engine_ptr = engine as *mut PresenceTestEngine;
        (*engine_ptr).run();
        (*engine_ptr).run();
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_presenceEngineDebug(
    _env: JNIEnv,
    _class: JClass,
    engine: jlong,
) {
    unsafe {
        let engine_ptr = engine as *mut PresenceTestEngine;
        println!("Engine id {}", (*engine_ptr).id);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_presenceEngineFree(
    _env: JNIEnv,
    _class: JClass,
    engine: jlong,
) {
    unsafe {
        let _ = Box::from_raw(engine as *mut PresenceTestEngine);
    }
}
