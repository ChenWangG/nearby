use jni::JavaVM;

static DISCOVERY_RESULT_CLASS: &str = "com/google/nearby/presence/DiscoveryResult";
static BUILD_SIGNATURE: &str = "([B)Lcom/google/nearby/presence/DiscoveryResult;";

pub fn new(jvm: JavaVM, service_data: Vec<u8>) {
    let mut env = jvm.get_env().unwrap();
    // TODO: figure out how to pass byte array.
    // Note: need to cast u8 to i8 to FFI byte array.
    let java_discovery_result = env.call_static_method(
        DISCOVERY_RESULT_CLASS,
        "build",
        BUILD_SIGNATURE,
        &[],
    )
        .unwrap()
        .l()
        .unwrap();
}