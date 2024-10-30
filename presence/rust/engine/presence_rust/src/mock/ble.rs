use ble::{BleScanRequest, ScanCallback, Scanner};

pub struct BleScanner;

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        println!("Mock BLE BleScanner start.");
    }
}