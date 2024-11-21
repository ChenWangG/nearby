use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::jlong;
use log::info;
use presence_java::java_discovery_result::DiscoveryResultBuilder;

static TEST_DISCOVERY_RESULT_CLASS: &str = "TestDiscoveryResult";
static ON_RESULT_SIGNATURE: &str = "(Lcom/google/nearby/presence/DiscoveryResult;)V";

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_TestDiscoveryResult_testDiscoveryResult
(mut env: JNIEnv,
 _class: JClass,
) {
    println!("================== Print Test Discovery Result.");
    env_logger::init();
    info!("================== Test Discovery Result.");
    let builder = DiscoveryResultBuilder::new(env.get_java_vm().unwrap());
    builder.add_data_element(5, vec![1, 2, 3, 4]);

    env.call_static_method(
        TEST_DISCOVERY_RESULT_CLASS,
        "onResult",
        ON_RESULT_SIGNATURE,
        &[JValue::Object(builder.build().as_obj())],
    )
        .unwrap();
}