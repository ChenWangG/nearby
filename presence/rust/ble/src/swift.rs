use crate::{BleScanRequest, BleScanResult, BleScanner, ScanCallback, ScanResult, Scanner, SwiftStartBleScan};
use log::info;
use  std::ffi::c_void;

impl BleScanner {
    pub fn new(ios_presence: *mut c_void, swift_start_ble_scan: SwiftStartBleScan) -> Self {
        info!("BleScanner.new()");
        BleScanner { ios_presence, swift_start_ble_scan }
    }
}

impl Scanner for BleScanner {
    fn start(&mut self, request: BleScanRequest, callback: impl ScanCallback) {
        info!("BleScanner.start()");
        // let callback = (Box::new(callback));
        (self.swift_start_ble_scan)(self.ios_presence, Box::into_raw(Box::new(callback)) as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ble_scanner_on_result(scan_callback: *mut dyn ScanCallback,
                                               scan_result: *mut ScanResult) {
    info!("ble_scanner_on_result.");
    let ble_scan_result = BleScanResult::new(1, (*scan_result).service_data.clone());
    (*scan_callback).on_update(ble_scan_result);
}

#[no_mangle]
pub unsafe extern "C" fn scan_result_new(data: *const u8, len: u32) -> *mut ScanResult {
    // TODO: delete clone.
    let service_data = std::slice::from_raw_parts(data, len as usize).to_vec().clone();
    Box::into_raw(Box::new(ScanResult::new(service_data)))
}

