mod scan_result;

use jni::{JNIEnv, JavaVM};
use jni::objects::{GlobalRef, JClass, JObject};
use jni::sys::jlong;
use log::info;

static BLE_CLASS: &str = "com/google/nearby/ble/BleScanner";
static BUILD_SIGNATURE: &str =
    "()Lcom/google/nearby/ble/BleScanner;";
static START_SIGNATURE: &str = "(J)V";
pub struct BleScanRequest {
    uuid: String,
    priority: i32,
}

impl BleScanRequest {
    pub fn new(uuid: String, priority: i32) -> Self {
       BleScanRequest{ uuid,  priority }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }
    pub fn priority(&self) -> i32 {
        self.priority
    }
}
#[derive(Clone)]
pub struct BleScanResult {
    tx_power: i32,
    service_data: Vec<u8>,
}

impl BleScanResult {
    pub fn new(tx_power: i32, service_data: Vec<u8>) -> Self {
        BleScanResult { tx_power, service_data }
    }

    pub fn tx_power(&self) -> i32 {
        self.tx_power
    }

    pub fn service_data(&self) -> &Vec<u8> {
        &self.service_data
    }
}

// TODO: avoid static lifetime? This is eventually owned by system API.
// Pass type T: ScanCallback instead of impl ScanCallback.
pub trait ScanCallback: std::marker::Sync + 'static {
  fn on_update(&self, result: BleScanResult);
}
pub trait Scanner : Send + 'static {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback);
}

pub struct ScanCallbackBox {
    scan_callback: Box<dyn ScanCallback>,
}

pub struct BleScanner {
    jvm: JavaVM,
    java_ble_scanner: GlobalRef,
}

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        let scan_callback_box = ScanCallbackBox { scan_callback: Box::new(callback)};
        let callback_ptr = Box::into_raw(Box::new(scan_callback_box)) as jlong;
        info!("BleScanner Lib start with callback addr: {}.", callback_ptr);
        let mut env = self.jvm.get_env().unwrap();
        env.call_method(
            self.java_ble_scanner.as_obj(),
            "start",
            START_SIGNATURE,
            &[callback_ptr.into()],
        ).unwrap();
    }
}

impl BleScanner {
    pub fn new(jvm: JavaVM) -> Self {
        let mut env = jvm.get_env().unwrap();
        let ble_scanner = env.call_static_method(
            BLE_CLASS,
            "build",
            BUILD_SIGNATURE,
            &[],
        )
            .unwrap()
            .l()
            .unwrap();
        let java_ble_scanner =env.new_global_ref(ble_scanner).unwrap();
        Self { jvm, java_ble_scanner }

    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_ble_BleScanner_onScanResult
(mut env: JNIEnv,
 _class: JClass,
 callback_ptr: jlong,
 result: jlong,
) {
    info!("BleScanner_onScanResult with callback_ptr: {}.", callback_ptr);
    let scan_callback_box_ptr = callback_ptr as *mut ScanCallbackBox;
    &(*scan_callback_box_ptr).scan_callback.on_update(BleScanResult::new(1, vec!(1)));
}