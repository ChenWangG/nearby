use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::{JavaVM, jint};

static CLASS_ENGINE: &str = "com/google/nearby/presence/engine/PresenceEngine";

pub fn discovery_result_builder_new<'a>(env: &mut JNIEnv<'a>, medium: jint) -> JObject<'a> {
    env.call_static_method(
        CLASS_ENGINE,
        "getDiscoveryResultBuilder",
        "(I)Lcom/google/nearby/presence/engine/PresenceDiscoveryResult$Builder;",
        &[medium.into()],
    ).unwrap().l().unwrap()
}