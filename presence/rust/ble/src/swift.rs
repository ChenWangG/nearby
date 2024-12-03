use crate::{BleScanRequest, BleScanner, ScanCallback, Scanner, SwiftStartBleScan};

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        (self.swift_start_ble_scan)();
    }
}

impl BleScanner {
    pub fn new(swift_start_ble_scan: SwiftStartBleScan) -> Self {
        BleScanner { swift_start_ble_scan }
    }
}