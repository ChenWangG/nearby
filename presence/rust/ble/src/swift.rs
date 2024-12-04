use crate::{BleScanRequest, BleScanner, ScanCallback, Scanner, SwiftStartBleScan};
use log::info;

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        info!("BleScanner.start()");
        (self.swift_start_ble_scan)();
    }
}

impl BleScanner {
    pub fn new(swift_start_ble_scan: SwiftStartBleScan) -> Self {
        info!("BleScanner.new()");
        BleScanner { swift_start_ble_scan }
    }
}