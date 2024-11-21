use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use log::info;
use presence_java::java_discovery_result;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_TestDiscoveryResult_testDiscoveryResult
(env: JNIEnv,
 _class: JClass,
) {
    println!("================== Print Test Discovery Result.");
    env_logger::init();
    info!("================== Test Discovery Result.");
    let _builder = java_discovery_result::new_builder(env.get_java_vm().unwrap());
}