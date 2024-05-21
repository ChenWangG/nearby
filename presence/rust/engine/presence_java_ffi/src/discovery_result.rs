use jni::objects::JObject;
use jni::sys::jint;
use jni::JNIEnv;

static CLASS_ENGINE: &str =  "com/google/nearby/presence/engine/PresenceDiscoveryResult";
static TO_BUILDER_SIGNATURE: &str = "(I)Lcom/google/nearby/presence/engine/PresenceDiscoveryResult$Builder;";
pub struct DiscoveryResultBuilder<'a> {
    builder: JObject<'a>,
}

impl<'a> DiscoveryResultBuilder<'a> {
    pub fn new(env: &mut JNIEnv<'a>, medium: i32) -> DiscoveryResultBuilder<'a> {
        let builder = discovery_result_builder_new(env, medium as jint);
        Self { builder }
    }

    pub fn add_action(&mut self, env: &mut JNIEnv<'a>, action: i32) {
        discovery_result_builder_add_action(env, action as jint, &self.builder);
    }

    pub fn debug(&mut self, env: &mut JNIEnv<'a>) {
        discovery_result_builder_debug(env, &self.builder);
    }
}

pub fn discovery_result_builder_new<'a>(env: &mut JNIEnv<'a>, medium: jint) -> JObject<'a> {
    env.call_static_method(
        CLASS_ENGINE,
        "toBuilder",
        TO_BUILDER_SIGNATURE,
        &[medium.into()],
    )
    .unwrap()
    .l()
    .unwrap()
}

pub fn discovery_result_builder_add_action<'a>(
    env: &mut JNIEnv<'a>,
    action: jint,
    builder: &JObject<'a>,
) {
    env.call_method(builder, "addAction", "(I)V", &[action.into()])
        .unwrap();
}

pub fn discovery_result_builder_debug<'a>(env: &mut JNIEnv<'a>, builder: &JObject<'a>) {
    let no_use: jint = 32;
    env.call_method(builder, "debug", "(I)V", &[no_use.into()])
        .unwrap();
}
