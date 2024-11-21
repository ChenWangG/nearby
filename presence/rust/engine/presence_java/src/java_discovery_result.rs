use jni::JavaVM;
use jni::objects::{GlobalRef, JByteArray, JObject, JValue};
use jni::sys::{jint, JNIEnv};
use presence_rust::client::DiscoveryResult;

static DISCOVERY_RESULT_CLASS: &str = "com/google/nearby/presence/DiscoveryResult";
static NEW_BUILDER_SIGNATURE: &str = "()Lcom/google/nearby/presence/DiscoveryResult$Builder;";

static BUILDER_CLASS: &str = "com.google.nearby.presence.DiscoveryResult$Builder";
static ADD_DE_SIGNATURE: &str = "(I[B)V";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/presence/DiscoveryResult;";

pub fn from_discovery_result(jvm: JavaVM, result: DiscoveryResult) -> GlobalRef {
    let mut builder = DiscoveryResultBuilder::new(jvm);
    for de in result.data_elements() {
        builder.add_data_element(de.de_type, de.content.clone());
    }
    builder.build()
}
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
            NEW_BUILDER_SIGNATURE,
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

    pub fn build(&self) -> GlobalRef {
        let mut env = self.jvm.get_env().unwrap();
        let local_ref = env.call_method(
            self.builder.as_obj(),
            "build",
            BUILD_SIGNATURE,
            &[]
        ).unwrap().l().unwrap();
        env.new_global_ref(local_ref).unwrap()
    }
}