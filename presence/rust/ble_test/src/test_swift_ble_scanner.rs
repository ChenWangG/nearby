use std::ptr::null_mut;
use ble::BleScanner;
use ble::SwiftStartBleScan;

#[no_mangle]
pub extern fn ble_scanner_new(swift_start_ble_scan: SwiftStartBleScan) -> *mut BleScanner {
    Box::into_raw(Box::new(BleScanner::new(swift_start_ble_scan)))
}