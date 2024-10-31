use std::sync::mpsc;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::jlong;
use jni::{JNIEnv, JavaVM};
use presence_rust::client::{Client, DiscoveryCallback, DiscoveryResult};
use presence_rust::Runtime;

struct Callback {}

impl DiscoveryCallback for Callback {
    fn on_update(&self, result: DiscoveryResult) {
        print!("DiscoveryCallback on_update");
    }
}

// Placeholder for both client and runtime to be "owned" by the host in Java.
struct PresenceRust {
    client: Client,
    runtime: Runtime<Callback>,
}

impl PresenceRust {
    pub fn test_ptr(&self) {
       println!("PresentRust test ptr.");
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_Presence_newPresence
(_env: JNIEnv,
 _class: JClass) -> jlong {
    let callback = Callback {};
    let (client, runtime) = presence_rust::create(callback);
    let presence_rust = PresenceRust{client, runtime};
    Box::into_raw(Box::new(presence_rust)) as jlong
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
    (*presence_rust_ptr).test_ptr();
}

