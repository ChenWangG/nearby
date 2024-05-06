use log::info;
use tokio::sync::mpsc;
use crate::{PresenceDiscoveryRequest, ProviderEvent};

#[derive(Debug)]
pub struct BleScanResult {
    pub priority: i32,
}

pub trait BleScanner {
    fn start_ble_scan(&self, request: PresenceDiscoveryRequest);
}

pub struct BleScanProvider {
    provider_event_tx: mpsc::Sender<ProviderEvent>,
    ble_scanner: Box<dyn BleScanner>,
}

impl BleScanProvider {
    pub fn new(provider_event_tx: mpsc::Sender<ProviderEvent>, ble_scanner: Box<dyn BleScanner>) -> Self {
        Self { provider_event_tx, ble_scanner }
    }
    // TODO: replace PresenceDiscoveryRequest with BleScanRequest.
    pub fn start_ble_scan(&self, request: PresenceDiscoveryRequest) {
        info!("BLE Scan Provider starts BLE scan.");
        self.ble_scanner.start_ble_scan(request);
    }

    pub fn on_scan_result(&self, result: BleScanResult) {
        info!("on_scan_result");
        if let Err(e) = self.provider_event_tx.blocking_send(ProviderEvent::BleScanResult(result)) {
            info!("Provider callback send error: {}", e);
        } else {
            info!("Provider callback sent an event.");
        }
    }
}