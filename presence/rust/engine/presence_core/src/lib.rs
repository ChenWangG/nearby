pub mod ble_scan_provider;
pub mod client_provider;

use crate::ble_scan_provider::{BleScanProvider, BleScanner, PresenceScanResult, ScanRequest};
use crate::client_provider::{ClientProvider, Device, DiscoveryCallback};
use client_provider::{
    DiscoveryResult, PresenceDiscoveryCondition, PresenceDiscoveryRequest, PresenceIdentityType,
    PresenceMeasurementAccuracy,
};
use log::{debug, info, log};
use tokio::runtime::Builder;
use tokio::sync::mpsc;

const PROVIDER_EVENT_CHANNEL_BUF_SIZE: usize = 100;

enum ProviderEvent {
    DiscoveryRequest(PresenceDiscoveryRequest),
    ScanResult(PresenceScanResult),
    Stop,
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
        let (provider_tx, provider_rx) =
            mpsc::channel::<ProviderEvent>(PROVIDER_EVENT_CHANNEL_BUF_SIZE);
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

    pub fn stop(&self) {
        self.client_provider.stop();
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
        while let Some(event) = self.provider_rx.recv().await {
            match event {
                ProviderEvent::DiscoveryRequest(request) => {
                    self.process_discovery_request(request).await
                }
                ProviderEvent::ScanResult(result) => self.process_scan_result(result).await,
                ProviderEvent::Stop => {
                    info!("Engine stopped");
                    break;
                }
            }
        }
    }

    async fn process_discovery_request(&self, request: PresenceDiscoveryRequest) {
        debug!("received a discovery request: {:?}.", request);
        let actions = request
            .conditions
            .iter()
            .map(|condition| condition.action)
            .collect();
        self.ble_scan_provider
            .start_ble_scan(ScanRequest::new(request.priority, actions));
    }

    async fn process_scan_result(&self, scan_result: PresenceScanResult) {
        debug!("received a BLE scan result: {:?}.", scan_result);
        let discovery_result = self.client_provider.on_device_update(DiscoveryResult::new(
            scan_result.medium,
            Device::new(scan_result.actions),
        ));
    }
}
