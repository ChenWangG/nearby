use std::ptr::null_mut;
use ble::BleScanner;
use ble::SwiftStartBleScan;
use log::{info, LevelFilter};
use oslog::OsLogger;

#[no_mangle]
pub extern "C" fn ble_scanner_new(swift_start_ble_scan: SwiftStartBleScan) -> *mut BleScanner {
    // TODO toggle on metadata in Xcode log view to show the log source.
    OsLogger::new("com.google.test_ble_scanner")
        .level_filter(LevelFilter::Debug)
        .init()
        .unwrap();
    info!("new Rust BLE Scanner.");
    Box::into_raw(Box::new(BleScanner::new(swift_start_ble_scan)))
}

#[no_mangle]
pub unsafe extern "C" fn ble_scanner_start(ble_scanner: *mut BleScanner) {
    info!("start scanner.");
    ((*ble_scanner).swift_start_ble_scan)();
}

#[no_mangle]
pub unsafe extern "C" fn rust_object_test() {
}