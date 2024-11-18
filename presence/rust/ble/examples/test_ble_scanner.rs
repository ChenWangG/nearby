use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use ble::{BleScanRequest, BleScanResult, BleScanner, ScanCallback, Scanner};

use log::{debug, info};
use log::LevelFilter;
use android_logger::Config;

struct TestBleCallback;

impl ScanCallback for TestBleCallback {
    fn on_update(&self, result: BleScanResult) {
        info!("on_upate: BleScanResult.");
    }
}

struct TestBleScanner {
    ble_scanner: BleScanner,
}

impl TestBleScanner {
    pub fn start(&mut self) {
        info!("TestBleScanner sart.");
        let scan_request = BleScanRequest::new(String::from("0000"), 1);
        self.ble_scanner.start(scan_request, TestBleCallback{} );
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_ble_1test_TestBleScanner_newTestBleScanner
(env: JNIEnv,
 _class: JClass) -> jlong {
    println!("[Rust] Println newTestBleScanner before logging started.");
    #[cfg(target_os = "linux")]
    env_logger::init();
    #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));
    info!("[PresenceTest][Rust] newTestBleScanner.");
    let test_ble_scanner = TestBleScanner { ble_scanner: BleScanner::new(env.get_java_vm().unwrap()) };
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