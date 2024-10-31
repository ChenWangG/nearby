use jni::objects::{JClass, JObject, JValue};
use jni::sys::jlong;
use jni::{JNIEnv, JavaVM};
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_Presence_newPresence
(_env: JNIEnv,
 _class: JClass) -> jlong {
    let value = 1 as jlong;
    value
}