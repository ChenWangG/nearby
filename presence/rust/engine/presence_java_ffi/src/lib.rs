//! JNI bindings for the presence_core crate.
//!
mod discovery_result;

extern crate jni;

use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong};
use jni::{JavaVM, JNIEnv};
use presence_core::ble_scan_provider::{BleScanner, ScanRequest};
use presence_core::client_provider::{DiscoveryCallback, DiscoveryResult};
use presence_core::PresenceEngine;

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
pub extern "system" fn Java_com_google_nearby_presence_engine_Engine_build(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = PresenceTestEngine { id: 101 };
    Box::into_raw(Box::new(engine)) as jlong
}
struct JavaBleScanner {
}

impl BleScanner for JavaBleScanner {
    fn start_ble_scan(&self, request: ScanRequest) {
        println!("BleScanner start ble scan.");
    }
}
struct JavaDiscoveryCallback {
}

impl DiscoveryCallback<JavaVM> for JavaDiscoveryCallback {
    fn on_device_update(&self, platform: &mut JavaVM, result: DiscoveryResult) {
        // let mut env = platform.attach_current_thread().unwrap();
        let mut env = platform.get_env().unwrap();
        println!("DiscoveryCallback on device update.");
        let mut builder = DiscoveryResultBuilder::new(&mut env, 19);
        builder.add_action(&mut env, 20);
        builder.add_action(&mut env, 21);
        builder.debug(&mut env);
        let result = builder.build(&mut env);
        jobject_debug(&mut env, &result);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_Engine_run(
    mut env_1: JNIEnv,
    _class: JClass,
    engine: jlong,
    object: JObject,
) {
    println!("Engine Run.");
    let jvm = env_1.get_java_vm().unwrap();

    let mut presence_engine = PresenceEngine::new(
        jvm, Box::new( JavaDiscoveryCallback{}), Box::new(JavaBleScanner{}));

    presence_engine.engine.test_discovery_callback();

    /*
    let jvm_ref = &jvm;
    let mut env = jvm_ref.get_env().unwrap();

    let mut builder = DiscoveryResultBuilder::new(&mut env, 19);
    builder.add_action(&mut env, 20);
    builder.add_action(&mut env, 21);
    builder.debug(&mut env);
    let result = builder.build(&mut env);
    jobject_debug(&mut env, &result);
     */

    /*
    env.call_method(object, "onDiscovery",ON_DISCOVERY_SIGNATURE, &[JValue::Object(&result)])
        .unwrap();

    unsafe {
        let engine_ptr = engine as *mut PresenceTestEngine;
        (*engine_ptr).run();
    }
     */
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_Engine_debug(
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
pub extern "system" fn Java_com_google_nearby_presence_engine_Engine_free(
    _env: JNIEnv,
    _class: JClass,
    engine: jlong,
) {
    unsafe {
        let _ = Box::from_raw(engine as *mut PresenceTestEngine);
    }
}
