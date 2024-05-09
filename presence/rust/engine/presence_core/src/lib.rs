pub mod ble_scan_provider;
pub mod client_provider;

use crate::ble_scan_provider::{BleScanProvider, BleScanner, PresenceBleScanResult};
use crate::client_provider::{DiscoveryCallback, ClientProvider};
use client_provider::{
    DiscoveryResult, PresenceDiscoveryCondition, PresenceDiscoveryRequest, PresenceIdentityType,
    PresenceMeasurementAccuracy,
};
use log::{info, log};
use tokio::runtime::Builder;
use tokio::sync::mpsc;

pub enum ProviderEvent {
    PresenceDiscoveryRequest(PresenceDiscoveryRequest),
    BleScanResult(PresenceBleScanResult),
}

pub struct PresenceEngine {
    // Receive events from Providers.
    provider_rx: mpsc::Receiver<ProviderEvent>,
    client_provider: ClientProvider,
    ble_scan_provider: BleScanProvider,
}

impl PresenceEngine {
    pub fn new(
        provider_tx: mpsc::Sender<ProviderEvent>,
        provider_rx: mpsc::Receiver<ProviderEvent>,
        discovery_callback: Box<dyn DiscoveryCallback>,
        ble_scanner: Box<dyn BleScanner>,
    ) -> Self {
        let client_provider = ClientProvider::new(provider_tx.clone(), discovery_callback);
        let ble_scan_provider = BleScanProvider::new(provider_tx, ble_scanner);
        Self {
            provider_rx,
            client_provider,
            ble_scan_provider,
        }
    }

    pub fn get_client_provider(&self) -> &ClientProvider {
        &self.client_provider
    }

    pub fn get_ble_scan_provider(&self) -> &BleScanProvider {
        &self.ble_scan_provider
    }
    pub fn run(&mut self) {
        info!("Presence Engine run.");
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
            info!("loop to receive provider events.");
            if let Some(event) = self.provider_rx.recv().await {
                match event {
                    ProviderEvent::PresenceDiscoveryRequest(request) => {
                        info!("received discovery request: {:?}.", request);
                        self.ble_scan_provider.start_ble_scan(request);
                    }
                    ProviderEvent::BleScanResult(result) => {
                        info!("received BLE scan result: {:?}.", result);
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
