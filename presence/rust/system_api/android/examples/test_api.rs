use jni::JNIEnv;

use system_api_android::BLE;
pub struct TestEngine;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_api_TestApi_startScan(
    mut env: JNIEnv,
) {
    println!("Start scan in Rust API.");
}