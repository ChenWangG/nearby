//! JNI bindings for the presence_core crate.
//!
mod discovery_result;

extern crate jni;

use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jint, jlong};
use jni::{JNIEnv, JavaVM};
use presence_core::ble_scan_provider::{BleScanner, ScanRequest};
use presence_core::client_provider::{DiscoveryCallback, DiscoveryResult};
use presence_core::PresenceEngine;

use crate::discovery_result::{jobject_debug, DiscoveryResultBuilder};

static ON_DISCOVERY_SIGNATURE: &str =
    "(Lcom/google/nearby/presence/engine/PresenceDiscoveryResult;)V";
static ON_START_SIGNATURE: &str = "(J)V";

struct Platform<'a> {
    jvm: JavaVM,
    j_object: &'a JObject<'a>,
}
struct JavaBleScanner {}

impl BleScanner for JavaBleScanner {
    fn start_ble_scan(&self, request: ScanRequest) {
        println!("BleScanner start ble scan.");
    }
}
struct JavaDiscoveryCallback {}

impl DiscoveryCallback<Platform<'_>> for JavaDiscoveryCallback {
    fn on_device_update(&self, platform: &Platform, result: DiscoveryResult) {
        // let mut env = platform.attach_current_thread().unwrap();
        println!("DiscoveryCallback on device update.");
        let mut builder = DiscoveryResultBuilder::new(&platform.jvm, 19);
        builder.add_action(20);
        builder.add_action(21);
        let result = builder.build();
        let mut env = platform.jvm.get_env().unwrap();
        jobject_debug(&mut env, &result);
        env.call_method(
            platform.j_object,
            "onDiscovery",
            ON_DISCOVERY_SIGNATURE,
            &[JValue::Object(&result)],
        )
        .unwrap();
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_Engine_start(
    mut env: JNIEnv,
    _class: JClass,
    engine: jlong,
    object: JObject,
) {
    println!("Engine Run.");
    let platform = Platform {
        jvm: env.get_java_vm().unwrap(),
        j_object: &object,
    };

    let mut presence_engine = PresenceEngine::new(
        platform,
        Box::new(JavaDiscoveryCallback {}),
        Box::new(JavaBleScanner {}),
    );


    presence_engine.engine.test_discovery_callback();

    // Note, Box::into_raw() below also works while it holds the Engine in heap forever,
    // which violates the lifetime annotation within this function.
    // let engine_ptr = Box::into_raw(Box::new(presence_engine));
    let addr = &mut presence_engine as *mut PresenceEngine<Platform> as jlong;
    env.call_method(
        &object,
        "onStart",
        ON_START_SIGNATURE,
        &[addr.into()],
    ).unwrap();

    presence_engine.engine.run();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_google_nearby_presence_engine_Engine_debug(
    _env: JNIEnv,
    _class: JClass,
    engine: jlong,
) {
    unsafe {
        let engine_ptr = engine as *mut PresenceEngine<Platform>;
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
        let _ = Box::from_raw(engine as *mut PresenceEngine<Platform>);
    }
}
