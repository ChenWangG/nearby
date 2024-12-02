mod android;
mod scan_result;

use jni::objects::GlobalRef;
use jni::{JavaVM};

static BLE_CLASS: &str = "com/google/nearby/ble/BleScanner";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/ble/BleScanner;";
static START_SIGNATURE: &str = "(J)V";
pub struct BleScanRequest {
    uuid: String,
    priority: i32,
}

impl BleScanRequest {
    pub fn new(uuid: String, priority: i32) -> Self {
        BleScanRequest { uuid, priority }
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
        BleScanResult {
            tx_power,
            service_data,
        }
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
pub trait Scanner: Send + 'static {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback);
}

pub struct ScanCallbackBox {
    scan_callback: Box<dyn ScanCallback>,
}

pub struct BleScanner {
    jvm: JavaVM,
    java_ble_scanner: GlobalRef,
}
