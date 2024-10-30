    use ble::{BleScanRequest, ScanCallback, Scanner};

    pub struct BleScanner;

    pub struct TestBleScanner;

    impl Scanner for BleScanner {
        fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
            println!("Mock BLE BleScanner start.");
        }
    }