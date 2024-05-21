use jni::objects::JObject;
use jni::sys::jint;
use jni::JNIEnv;

static CLASS_ENGINE: &str = "com/google/nearby/presence/engine/PresenceDiscoveryResult";
static CLASS_BUILDER: &str = "com/google/nearby/presence/engine/PresenceDiscoveryResult$Builder";

pub fn discovery_result_builder_new<'a>(env: &mut JNIEnv<'a>, medium: jint) -> JObject<'a> {
    env.call_static_method(
        CLASS_ENGINE,
        "toBuilder",
        "(I)Lcom/google/nearby/presence/engine/PresenceDiscoveryResult$Builder;",
        &[medium.into()],
    )
    .unwrap()
    .l()
    .unwrap()
}

pub fn discovery_result_builder_add_action<'a>(env: &mut JNIEnv<'a>, action: jint, builder: JObject<'a>) {
    env.call_method(
        builder,
        "addAction",
        "(I)V",
        &[action.into()],
    ).unwrap();
}
