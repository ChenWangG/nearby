use std::ptr::null_mut;
use ble::{BleScanRequest, BleScanResult, BleScanner, ScanCallback, Scanner};
use ble::SwiftStartBleScan;
use log::{debug, info, LevelFilter};
use oslog::OsLogger;
struct TestBleCallback;

impl ScanCallback for TestBleCallback {
    fn on_update(&self, result: BleScanResult) {
        info!("on_upate: BleScanResult: {:?} ", result);
    }
}

#[no_mangle]
pub extern "C" fn ble_scanner_new(swift_start_ble_scan: SwiftStartBleScan) -> *mut BleScanner {
    // TODO toggle on metadata in Xcode log view to show the log source.
    OsLogger::new("com.google.test_ble_scanner")
        .level_filter(LevelFilter::Debug)
        .init()
        .unwrap();
    info!("[test_swift_ble_scanner] ble_scanner_new.");
    Box::into_raw(Box::new(BleScanner::new(swift_start_ble_scan)))
}

#[no_mangle]
pub unsafe extern "C" fn ble_scanner_start(ble_scanner: *mut BleScanner) {
    info!("[test_swift_ble_scanner] ble_scanner.start.");
    let scan_request = BleScanRequest::new(String::from("0000"), 1);
    (*ble_scanner).start(scan_request, TestBleCallback{});
}

#[no_mangle]
pub unsafe extern "C" fn rust_object_test() {
}