use crate::client_provider::{PresenceDiscoveryRequest, PresenceMedium};
use crate::ProviderEvent;
use log::{debug, error, info};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct ScanRequest {
    pub priority: i32,
    pub actions: Vec<i32>,
}

impl ScanRequest {
    pub fn new(priority: i32, actions: Vec<i32>) -> Self {
        Self { priority, actions }
    }
}

#[derive(Debug)]
pub struct PresenceScanResult {
    pub medium: PresenceMedium,
    pub actions: Vec<i32>,
}

impl PresenceScanResult {
    pub fn new(medium: PresenceMedium, actions: Vec<i32>) -> Self {
        Self { medium, actions }
    }
}

pub trait BleScanner {
    fn start_ble_scan(&self, request: ScanRequest);
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
    pub fn start_ble_scan(&self, request: ScanRequest) {
        debug!("BLE Scan Provider starts BLE scan.");
        self.ble_scanner.start_ble_scan(request);
    }

    pub fn on_scan_result(&self, result: PresenceScanResult) {
        if let Err(e) = self
            .provider_event_tx
            .blocking_send(ProviderEvent::ScanResult(result))
        {
            error!("BLE scan Provider callback send error: {}", e);
        } else {
            debug!("BLE scan Provider callback sent an event.");
        }
    }
}
