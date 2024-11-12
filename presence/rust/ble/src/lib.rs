use jni::{JNIEnv, JavaVM};
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

pub trait ScanCallback: std::marker::Sync {
  fn on_update(&self, result: BleScanResult);
}
pub trait Scanner : Send + 'static {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback);
}

pub struct BleScanner {}

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        println!("BleScanner Lib start.");
    }
}