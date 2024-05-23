use jni::objects::{JObject, JValue};
use jni::sys::jint;
use jni::{JNIEnv, JavaVM};

static CLASS_ENGINE: &str = "com/google/nearby/presence/engine/PresenceDiscoveryResult";
static TO_BUILDER_SIGNATURE: &str =
    "(I)Lcom/google/nearby/presence/engine/PresenceDiscoveryResult$Builder;";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/presence/engine/PresenceDiscoveryResult;";
pub struct DiscoveryResultBuilder<'a> {
    jvm: &'a JavaVM,
    builder: JObject<'a>,
}

impl<'a> DiscoveryResultBuilder<'a> {
    pub fn new(jvm: &'a JavaVM, medium: i32) -> DiscoveryResultBuilder<'a> {
        let mut env = jvm.get_env().unwrap();
        let builder = discovery_result_builder_new(&mut env, medium as jint);
        Self { jvm, builder }
    }

    pub fn add_action(&mut self, action: i32) {
        discovery_result_builder_add_action(
            &mut self.jvm.get_env().unwrap(),
            action as jint,
            &self.builder,
        );
    }

    pub fn build(&mut self) -> JObject<'a> {
        let empty: [JValue; 0] = [];
        self.jvm
            .get_env()
            .unwrap()
            .call_method(&self.builder, "build", BUILD_SIGNATURE, &[])
            .unwrap()
            .l()
            .unwrap()
    }

    pub fn debug(&mut self, env: &mut JNIEnv<'a>) {
        jobject_debug(env, &self.builder);
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

pub fn jobject_debug<'a>(env: &mut JNIEnv<'a>, jobject: &JObject<'a>) {
    let empty: [JValue; 0] = [];
    env.call_method(jobject, "debug", "()V", &empty).unwrap();
}
