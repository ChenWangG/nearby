use std::sync::mpsc;
use std::thread;

use presence_core::ble_scan_provider::{BleScanner, ScanRequest};
use presence_core::client_provider::{
    DiscoveryCallback, DiscoveryResult, PresenceDiscoveryCondition, PresenceDiscoveryRequest,
    PresenceIdentityType, PresenceMeasurementAccuracy,
};
use presence_core::PresenceEngine;

struct MockDiscoveryCallback {}

impl DiscoveryCallback for MockDiscoveryCallback {
    fn on_device_update(&self, result: DiscoveryResult) {}
}

unsafe impl Send for MockDiscoveryCallback {}

struct MockBleScanner {
    pub scan_request_tx: mpsc::Sender<ScanRequest>,
}

impl BleScanner for MockBleScanner {
    fn start_ble_scan(&self, request: ScanRequest) {
        self.scan_request_tx.send(request).unwrap();
    }
}

unsafe impl Send for MockBleScanner {}
#[test]
fn test_engine() {
    let (scan_request_tx, scan_request_rx) = mpsc::channel();
    let mut engine = PresenceEngine::new(
        Box::new(MockDiscoveryCallback {}),
        Box::new(MockBleScanner { scan_request_tx }),
    );

    thread::scope(|scope| {
        let engine_thread = scope.spawn(|| engine.run());

        /*
        let condition = PresenceDiscoveryCondition {
            action: 100,
            identity_type: PresenceIdentityType::Private,
            measurement_accuracy: PresenceMeasurementAccuracy::Unknown,
        };
        let request = PresenceDiscoveryRequest::new(1, Vec::from([condition]));
        engine.set_discovery_request(request);
         */

        engine_thread.join();
    });


    let request = scan_request_rx.recv().unwrap();
    println!("received request {:?}", request);

}
