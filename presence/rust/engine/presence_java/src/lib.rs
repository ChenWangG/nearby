mod java_discovery_result;

use std::ptr::null_mut;
use jni::objects::{GlobalRef, JClass, JObject};
use jni::sys::jlong;
use jni::{JNIEnv, JavaVM};

use presence_rust::client::{Client, DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
use ble::Scanner;
use presence_rust::Runtime;

use log::{debug, info};
use log::LevelFilter;
use android_logger::Config;

#[cfg(feature = "mock")]
use presence_rust::mock::ble::BleScanner;
#[cfg(not(feature = "mock"))]
use ble::BleScanner;

static ON_DISCOVERY_SIGNATURE: &str = "(J)V";

struct Callback {
    jvm: JavaVM,
    presence_java: GlobalRef,
}

impl DiscoveryCallback for Callback {
    fn on_update(&self, result: DiscoveryResult) {
        info!("(PresenceRust) DiscoveryCallback on_update");
        info!("(PresenceRust) {}", result.service_data().len());
        let addr = 1 as jlong;
        self.jvm.get_env().unwrap().call_method(
            self.presence_java.as_obj(),
            "onDiscovery",
            ON_DISCOVERY_SIGNATURE,
            &[addr.into()],
        )
            .unwrap();
    }
}

// Placeholder for both client and runtime to be "owned" by the host in Java.
struct PresenceRust {
    client: *mut Client,
    runtime: *mut Runtime<Callback, BleScanner>,
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_Presence_newPresence
(env: JNIEnv,
 _class: JClass) -> jlong {
    println!("[Rust] Println newTestBleScanner before logging started.");
    #[cfg(target_os = "linux")]
    env_logger::init();
    #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));
    info!("[PresenceRust] new Presence Rust.");
    let ble_scanner = BleScanner::new(env.get_java_vm().unwrap());
    let (client, runtime) = presence_rust::create(ble_scanner);
    let presence_rust = PresenceRust{client: Box::into_raw(Box::new(client)), runtime: Box::into_raw(Box::new(runtime))};
    Box::into_raw(Box::new(presence_rust)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_Presence_start
(mut env: JNIEnv,
 presence_java: JObject,
 presence_rust_ptr: jlong,
) {
    println!("Presence Rust start.");

    let callback = Callback{
        jvm: env.get_java_vm().unwrap(),
        presence_java: env.new_global_ref(presence_java).unwrap(),
    };

    let presence_rust_ptr = presence_rust_ptr as *mut PresenceRust;
    let mut runtime = Box::from_raw((*presence_rust_ptr).runtime);
    (*presence_rust_ptr).runtime = null_mut();

    runtime.set_discovery_callback(callback);
    runtime.start();

    // let mut presence_java = env.new_global_ref(presence_java).unwrap();

    /*
    let addr = 1 as jlong;
    env.call_method(
        callback.presence_java.as_obj(),
        "onDiscovery",
        ON_DISCOVERY_SIGNATURE,
        &[addr.into()],
    )
        .unwrap();
     */
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_Presence_stop
(mut env: JNIEnv,
 presence_java: JObject,
 presence_rust_ptr: jlong,
) {
    let presence_rust_ptr = presence_rust_ptr as *mut PresenceRust;
    (*(*presence_rust_ptr).client).stop();
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_Presence_setRequest
(_env: JNIEnv,
 _class: JClass,
presence_rust_ptr: jlong,
request_ptr: jlong
) {
   println!("Presence Rust set request.");
    let presence_rust_ptr = presence_rust_ptr as *mut PresenceRust;
    (*(*presence_rust_ptr).client).set_request(DiscoveryRequest{ priority: 100 });
}

