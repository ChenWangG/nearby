use std::sync::mpsc;
use jni::objects::{GlobalRef, JClass, JObject, JValue};
use jni::sys::jlong;
use jni::{JNIEnv, JavaVM};
use presence_rust::client::{Client, DiscoveryCallback, DiscoveryResult};
use presence_rust::Runtime;

static ON_DISCOVERY_SIGNATURE: &str = "(J)V";

struct Callback {
    jvm: JavaVM,
    presence_java: GlobalRef,
}

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
    pub fn client(&mut self) -> &mut Client {
        &mut self.client
    }
    pub fn runtime(&mut self) -> &mut Runtime<Callback> {
        &mut self.runtime
    }
    pub fn test_ptr(&self) {
       println!("PresentRust test ptr.");
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_Presence_newPresence
(_env: JNIEnv,
 _class: JClass) -> jlong {
    let (client, runtime) = presence_rust::create();
    let presence_rust = PresenceRust{client, runtime};
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

    /*
    let presence_rust_ptr = presence_rust_ptr as *mut PresenceRust;
    (*presence_rust_ptr).runtime().set_discovery_callback(callback);
     */

    // let mut presence_java = env.new_global_ref(presence_java).unwrap();

    let addr = 1 as jlong;
    env.call_method(
        callback.presence_java.as_obj(),
        "onDiscovery",
        ON_DISCOVERY_SIGNATURE,
        &[addr.into()],
    )
        .unwrap();
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

