use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use ble::BleScanner;

use log::{debug, info};

struct TestBleScanner {
    ble_scanner: BleScanner,
}

impl TestBleScanner {
    pub fn start(&mut self) {
       info!("TestBleScanner sart.")
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_ble_1test_TestBleScanner_newTestBleScanner
(_env: JNIEnv,
 _class: JClass) -> jlong {
    println!("[Rust] Println newTestBleScanner before logging started.");
    env_logger::init();
    info!("Rust newTestBleScanner.");
    let test_ble_scanner = TestBleScanner { ble_scanner: BleScanner{} };
    Box::into_raw(Box::new(test_ble_scanner)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_ble_1test_TestBleScanner_start
(_env: JNIEnv,
 _class: JClass,
 scanner_ptr : jlong
) {
    info!("Start BLE scanner.");
    let mut scanner = scanner_ptr as *mut TestBleScanner;
    &(*scanner).start();
}