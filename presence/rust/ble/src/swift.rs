use crate::{BleScanRequest, BleScanResult, BleScanner, ScanCallback, ScanResult, Scanner, SwiftStartBleScan};
use log::info;

impl BleScanner {
    pub fn new(swift_start_ble_scan: SwiftStartBleScan) -> Self {
        info!("BleScanner.new()");
        BleScanner { swift_start_ble_scan, callback: None  }
    }
}

impl Scanner for BleScanner {
    fn start(&mut self, request: BleScanRequest, callback: impl ScanCallback) {
        info!("BleScanner.start()");
        self.callback = Some(Box::new(callback));
        (self.swift_start_ble_scan)(std::ptr::null_mut());
    }
}

#[no_mangle]
pub unsafe extern "C" fn ble_scanner_on_result(ble_scanner: *mut BleScanner,
                                               scan_result: *mut ScanResult) {
    info!("ble_scanner_on_result.");
    let ble_scan_result = BleScanResult::new(1, (*scan_result).service_data.clone());
    (*ble_scanner).callback.as_mut().expect("scan not started.").on_update(ble_scan_result);
}

#[no_mangle]
pub unsafe extern "C" fn scan_result_new(data: *const u8, len: u32) -> *mut ScanResult {
    // TODO: delete clone.
    let service_data = std::slice::from_raw_parts(data, len as usize).to_vec().clone();
    Box::into_raw(Box::new(ScanResult::new(service_data)))
}

