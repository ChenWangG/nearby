use jni::JavaVM;

static DISCOVERY_RESULT_CLASS: &str = "com/google/nearby/presence/DiscoveryResult";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/presence/DiscoveryResult$Builder;";

pub fn new_builder(jvm: JavaVM) {
    let mut env = jvm.get_env().unwrap();
    env.call_static_method(
        DISCOVERY_RESULT_CLASS,
        "newBuilder",
        BUILD_SIGNATURE,
        &[],
    )
        .unwrap()
        .l()
        .unwrap();
}