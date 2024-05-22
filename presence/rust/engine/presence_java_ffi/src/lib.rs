//! JNI bindings for the presence_core crate.
//!
mod discovery_result;

extern crate jni;

use jni::objects::{JClass, JObject, JValue, JValueGen};
use jni::sys::{jint, jlong};
use jni::JNIEnv;

use crate::discovery_result::{DiscoveryResultBuilder, jobject_debug};

static ON_DISCOVERY_SIGNATURE: &str = "(Lcom/google/nearby/presence/engine/PresenceDiscoveryResult;)V";

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
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_build(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = PresenceTestEngine { id: 101 };
    Box::into_raw(Box::new(engine)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_PresenceEngine_presenceEngineRun(
    mut env: JNIEnv,
    _class: JClass,
    engine: jlong,
    object: JObject,
) {
    let mut builder = DiscoveryResultBuilder::new(&mut env, 19);
    builder.add_action(&mut env, 20);
    builder.add_action(&mut env, 21);
    builder.debug(&mut env);
    let result = builder.build(&mut env);
    jobject_debug(&mut env, &result);

    env.call_method(object, "onDiscovery",ON_DISCOVERY_SIGNATURE, &[JValue::Object(&result)])
        .unwrap();

    unsafe {
        let engine_ptr = engine as *mut PresenceTestEngine;
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
