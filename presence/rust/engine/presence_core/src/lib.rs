pub mod ble_scan_provider;
pub mod client_provider;

use crate::ble_scan_provider::{BleScanProvider, BleScanner, PresenceScanResult, ScanRequest};
use crate::client_provider::{ClientProvider, DiscoveryCallback};
use client_provider::{
    DiscoveryResult, PresenceDiscoveryCondition, PresenceDiscoveryRequest, PresenceIdentityType,
    PresenceMeasurementAccuracy,
};
use log::{debug, info, log};
use tokio::runtime::Builder;
use tokio::sync::mpsc;

enum ProviderEvent {
    DiscoveryRequest(PresenceDiscoveryRequest),
    BleScanResult(PresenceScanResult),
}

pub struct PresenceEngine {
    // Receive events from Providers.
    provider_rx: mpsc::Receiver<ProviderEvent>,
    client_provider: ClientProvider,
    ble_scan_provider: BleScanProvider,
}

impl PresenceEngine {
    pub fn new(
        discovery_callback: Box<dyn DiscoveryCallback>,
        ble_scanner: Box<dyn BleScanner>,
    ) -> Self {
        info!("Create Presence Engine.");
        let (provider_tx, provider_rx) = mpsc::channel::<ProviderEvent>(100);
        Self {
            provider_rx,
            client_provider: ClientProvider::new(provider_tx.clone(), discovery_callback),
            ble_scan_provider: BleScanProvider::new(provider_tx, ble_scanner),
        }
    }

    pub fn set_discovery_request(&self, request: PresenceDiscoveryRequest) {
        self.client_provider.set_discovery_request(request);
    }

    pub fn on_scan_result(&self, result: PresenceScanResult) {
        self.ble_scan_provider.on_scan_result(result);
    }

    pub fn run(&mut self) {
        info!("Run Presence Engine.");
        Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async move {
                self.poll_providers().await;
            });
    }

    async fn poll_providers(&mut self) {
        // loop to receive events from Providers and process the event according to its type.
        loop {
            if let Some(event) = self.provider_rx.recv().await {
                match event {
                    ProviderEvent::DiscoveryRequest(request) => {
                        debug!("received a discovery request: {:?}.", request);
                        let actions = request
                            .conditions
                            .iter()
                            .map(|condition| condition.action)
                            .collect();
                        self.ble_scan_provider
                            .start_ble_scan(ScanRequest::new(request.priority, actions));
                    }
                    ProviderEvent::BleScanResult(result) => {
                        debug!("received a BLE scan result: {:?}.", result);
                        let mut discovery_result = DiscoveryResult::new(result.medium);
                        for action in result.actions {
                            discovery_result.add_action(action);
                        }
                        self.client_provider.on_device_update(discovery_result);
                    }
                }
            }
        }
    }
}
