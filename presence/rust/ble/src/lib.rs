mod scan_result;
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
#[derive(Clone, Debug)]
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
pub trait ScanCallback: std::marker::Sync + Send + 'static {
    fn on_update(&self, result: BleScanResult);
}
pub trait Scanner: Send + 'static {
    fn start(&mut self, request: BleScanRequest, callback: impl ScanCallback);
}

pub struct ScanResult {
    service_data: Vec<u8>,
}

impl ScanResult {
    pub fn new(service_data: Vec<u8>) -> Self {
        ScanResult { service_data }
    }

    pub fn service_data(&self) -> &Vec<u8> {
        &(self.service_data)
    }
}


// pub type SwiftStartBleScan = fn(* mut std::os::raw::c_void);
// pub type SwiftStartBleScan = fn(*mut dyn ScanCallback);
#[cfg(target_os = "ios")]
mod swift;
#[cfg(target_os = "ios")]
pub type SwiftStartBleScan = fn(* mut std::ffi::c_void);
// Two pointers: first for the IosBleScanner, second to pass the callback.
#[cfg(target_os = "ios")]
pub struct BleScanner {
    swift_start_ble_scan: SwiftStartBleScan,
    //  TODO: pass callback through start().
    callback: Option<Box<dyn ScanCallback>>,
}

#[cfg(target_os = "android")]
pub struct BleScanner {
    jvm: JavaVM,
    java_ble_scanner: GlobalRef,
}
#[cfg(target_os = "android")]
mod android;
