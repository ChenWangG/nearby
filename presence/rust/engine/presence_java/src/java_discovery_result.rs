use jni::JavaVM;
use jni::objects::{JByteArray, JValue};
use jni::sys::jint;

static DISCOVERY_RESULT_CLASS: &str = "com/google/nearby/presence/DiscoveryResult";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/presence/DiscoveryResult$Builder;";

static BUILDER_CLASS: &str = "com.google.nearby.presence.DiscoveryResult$Builder";
static ADD_DE_SIGNATURE: &str = "(I[B)V";
pub fn new_builder(jvm: JavaVM) {
    let mut env = jvm.get_env().unwrap();
    let builder = env.call_static_method(
        DISCOVERY_RESULT_CLASS,
        "newBuilder",
        BUILD_SIGNATURE,
        &[],
    )
        .unwrap()
        .l()
        .unwrap();
    let buf = vec![1, 2, 3];
    let data_elements : JByteArray = env.byte_array_from_slice(&*buf).unwrap();
    // let mut data_elements = env.new_byte_array(1).unwrap();
    let de_type : jint = 1;
    env.call_method(
        builder,
        "addDataElement",
        ADD_DE_SIGNATURE,
        &[de_type.into(), JValue::Object(&data_elements)]
    ).expect("Failed to add DE.");
}