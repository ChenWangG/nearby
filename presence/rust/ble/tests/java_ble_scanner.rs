use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use ble::BleScanner;

struct JavaBleScanner {
    ble_scanner: BleScanner,
}
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_test_TestBleScanner_newTestBleScanner
(_env: JNIEnv,
 _class: JClass) -> jlong {
  0
}