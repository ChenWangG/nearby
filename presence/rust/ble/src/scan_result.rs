use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JByteArray;
use jni::sys::jlong;
use log::info;
use crate::{BleScanResult, ScanCallbackBox};
use jni::objects::ReleaseMode;

pub struct ScanResult {
    service_data: Vec<u8>,
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_ble_ScanResult_toRustScanResult
(mut env: JNIEnv,
 _class: JClass,
 service_data: JByteArray
) -> jlong {
    info!("To Rust ScanResult.");
    let elements = env.get_array_elements(&service_data, ReleaseMode::NoCopyBack).unwrap();
    let elem0 = elements.get(0).unwrap() as &i8;
    println!("elem0 is {}", elem0);
    0
}
