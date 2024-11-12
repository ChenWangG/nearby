use std::thread;
use ble::{BleScanRequest, BleScanResult, ScanCallback, Scanner};
use crate::scan_provider::ble_scan_provider::UUID;

pub const SERVICE_DATA: &[u8] = &[1, 2, 3];

pub struct BleScanner;

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        println!("Mock BLE Scanner start.");
        assert_eq!(request.uuid(), UUID);
        assert_eq!(request.priority(), 100);
        thread::scope(|scope| {
            let system_thread = scope.spawn(|| {
                callback.on_update(BleScanResult::new(10, Vec::from(SERVICE_DATA)));
            });
            system_thread.join();
        });
    }
}