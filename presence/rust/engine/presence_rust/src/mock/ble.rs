use std::thread;
use ble::{BleScanRequest, BleScanResult, ScanCallback, Scanner};
use crate::scan_provider::ble_scan_provider::UUID;

pub struct BleScanner;

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        println!("Mock BLE BleScanner start.");
        assert_eq!(request.uuid(), UUID);
        thread::scope(|scope| {
            let system_thread = scope.spawn(|| {
                callback.on_update(BleScanResult::new(10, vec![1, 2, 3]));
            });
            system_thread.join();
        });
    }
}