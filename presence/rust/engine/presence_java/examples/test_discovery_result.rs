use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use log::info;
use presence_java::java_discovery_result::DiscoveryResultBuilder;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_TestDiscoveryResult_testDiscoveryResult
(env: JNIEnv,
 _class: JClass,
) {
    println!("================== Print Test Discovery Result.");
    env_logger::init();
    info!("================== Test Discovery Result.");
    let builder = DiscoveryResultBuilder::new(env.get_java_vm().unwrap());
    builder.add_data_element(1, vec![1, 2, 3, 4]);
    builder.build();
}