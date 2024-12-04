use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JByteArray;
use jni::sys::jlong;
use log::{debug, info};
use crate::{BleScanResult, ScanResult};
use jni::objects::ReleaseMode;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_ble_ScanResult_toRustScanResult
(mut env: JNIEnv,
 _class: JClass,
 service_data: JByteArray
) -> jlong {
    info!("To Rust ScanResult.");
    let elements = env.get_array_elements(&service_data, ReleaseMode::NoCopyBack).unwrap();
    let element_vec = elements.to_vec();

    let mut service_data = Vec::new();
    // TODO: the byte array contains i8, cast to u8 in Rust now. The binary value will not change.
    for element in element_vec {
        service_data.push(element as u8);
    }
    debug!("service data length: {}",  service_data.len());
    for data in &service_data {
        debug!("{}", data);
    }
    Box::into_raw(Box::new(ScanResult::new(service_data))) as jlong
}
