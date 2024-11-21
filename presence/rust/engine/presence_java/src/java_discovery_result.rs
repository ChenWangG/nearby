use jni::JavaVM;
use jni::objects::{GlobalRef, JByteArray, JObject, JValue};
use jni::sys::jint;

static DISCOVERY_RESULT_CLASS: &str = "com/google/nearby/presence/DiscoveryResult";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/presence/DiscoveryResult$Builder;";

static BUILDER_CLASS: &str = "com.google.nearby.presence.DiscoveryResult$Builder";
static ADD_DE_SIGNATURE: &str = "(I[B)V";

pub struct DiscoveryResultBuilder {
    jvm: JavaVM,
    builder: GlobalRef,
}

impl DiscoveryResultBuilder {
    pub fn new(jvm: JavaVM) -> Self {
        let mut env = jvm.get_env().unwrap();
        let builder_local = env.call_static_method(
            DISCOVERY_RESULT_CLASS,
            "newBuilder",
            BUILD_SIGNATURE,
            &[],
        )
            .unwrap()
            .l()
            .unwrap();
        let builder = env.new_global_ref(builder_local).unwrap();
        Self{ jvm, builder }
    }

    pub fn add_data_element(&self, de_type: u32, content: Vec<u8>) {
        let mut env = self.jvm.get_env().unwrap();
        let data_elements: JByteArray = env.byte_array_from_slice(&*content).unwrap();
        let de_type = de_type as jint;
        env.call_method(
            self.builder.as_obj(),
            "addDataElement",
            ADD_DE_SIGNATURE,
            &[de_type.into(), JValue::Object(&data_elements)]
        ).expect("Failed to add DE.");
    }
}