use jni::objects::JObject;
use jni::sys::jint;
use jni::{JNIEnv, JavaVM};
use presence_core::client_provider::DiscoveryResult;

static CLASS_ENGINE: &str = "com/google/nearby/presence/engine/PresenceDiscoveryResult";
static TO_BUILDER_SIGNATURE: &str =
    "(I)Lcom/google/nearby/presence/engine/PresenceDiscoveryResult$Builder;";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/presence/engine/PresenceDiscoveryResult;";
pub struct PresenceDiscoveryResultBuilder<'a> {
    jvm: &'a JavaVM,
    builder: JObject<'a>,
}

impl<'a> PresenceDiscoveryResultBuilder<'a> {
    pub fn new(jvm: &'a JavaVM, medium: i32) -> PresenceDiscoveryResultBuilder<'a> {
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

    pub fn from_discovery_result(jvm: &'a JavaVM, discovery_result: DiscoveryResult) -> Self {
        let mut builder = PresenceDiscoveryResultBuilder::new(jvm, discovery_result.medium as i32);
        for action in discovery_result.device.actions {
            builder.add_action(action);
        }
        builder
    }

    pub fn build(&mut self) -> JObject<'a> {
        self.jvm
            .get_env()
            .unwrap()
            .call_method(&self.builder, "build", BUILD_SIGNATURE, &[])
            .unwrap()
            .l()
            .unwrap()
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
