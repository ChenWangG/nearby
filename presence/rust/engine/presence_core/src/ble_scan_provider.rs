use crate::ProviderEvent;
use log::{debug, error, info};
use tokio::sync::mpsc;
use crate::client_provider::{PresenceDiscoveryRequest, PresenceMedium};

#[derive(Debug)]
pub struct PresenceBleScanResult {
    pub medium: PresenceMedium,
    pub actions: Vec<i32>,
}

impl PresenceBleScanResult {
    pub fn new(medium: PresenceMedium, actions: Vec<i32>) -> Self {
        Self {medium, actions}
    }
}

pub trait BleScanner {
    fn start_ble_scan(&self, request: PresenceDiscoveryRequest);
}

pub struct BleScanProvider {
    provider_event_tx: mpsc::Sender<ProviderEvent>,
    ble_scanner: Box<dyn BleScanner>,
}

impl BleScanProvider {
    pub fn new(
        provider_event_tx: mpsc::Sender<ProviderEvent>,
        ble_scanner: Box<dyn BleScanner>,
    ) -> Self {
        Self {
            provider_event_tx,
            ble_scanner,
        }
    }
    // TODO: replace PresenceDiscoveryRequest with BleScanRequest.
    pub fn start_ble_scan(&self, request: PresenceDiscoveryRequest) {
        debug!("BLE Scan Provider starts BLE scan.");
        self.ble_scanner.start_ble_scan(request);
    }

    pub fn on_scan_result(&self, result: PresenceBleScanResult) {
        if let Err(e) = self
            .provider_event_tx
            .blocking_send(ProviderEvent::BleScanResult(result))
        {
            error!("BLE scan Provider callback send error: {}", e);
        } else {
            debug!("BLE scan Provider callback sent an event.");
        }
    }
}
