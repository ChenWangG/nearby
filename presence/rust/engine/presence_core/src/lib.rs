pub mod ble_scan_provider;
pub mod client_provider;

use log::{debug, info};

use crate::ble_scan_provider::{BleScanCallback, BleScanner, PresenceScanResult, ScanRequest};
use crate::client_provider::{ClientProvider, Device, DiscoveryCallback, PresenceMedium};
use client_provider::{DiscoveryResult, PresenceDiscoveryRequest};

use tokio::runtime::Builder;
use tokio::sync::mpsc;

const PROVIDER_EVENT_CHANNEL_BUF_SIZE: usize = 100;

pub enum ProviderEvent {
    DiscoveryRequest(PresenceDiscoveryRequest),
    ScanResult(PresenceScanResult),
    Stop,
}

pub struct PresenceEngine<T> {
    pub engine: Engine<T>,
    pub client_provider: ClientProvider,
    pub ble_scan_callback: BleScanCallback,
}

impl<T> PresenceEngine<T> {
    pub fn new(
        platform: T,
        discovery_callback: Box<dyn DiscoveryCallback<T>>,
        ble_scanner: Box<dyn BleScanner>,
    ) -> Self {
        info!("Create Presence Engine.");
        let (provider_tx, provider_rx) =
            mpsc::channel::<ProviderEvent>(PROVIDER_EVENT_CHANNEL_BUF_SIZE);
        Self {
            engine: Engine::new(platform, provider_rx, discovery_callback, ble_scanner),
            client_provider: ClientProvider::new(provider_tx.clone()),
            ble_scan_callback: BleScanCallback::new(provider_tx),
        }
    }
}
pub struct Engine<T> {
    // The platform is Platform specific and opaque to core.
    // It s passed through the core from client to platform system APIs.
    // TODO: we have to consume T here and pass &T into discovery_callback.on_device_update().
    // We cannot pass &'a T directly in presence_java_ffi when discovery_callback is boxed
    // since reference lifetime in boxed object need to be static.
    // Consider using generic instead of trait here to define the callback interfaces.
    // i.e. discovery_callback is a type D which implements the DiscoveryCallback trait.
    // This way, discovery_callback is not boxed any more and should take &'a T.
    platform: T,
    // Receive events from Providers.
    provider_rx: mpsc::Receiver<ProviderEvent>,
    discovery_callback: Box<dyn DiscoveryCallback<T>>,
    ble_scanner: Box<dyn BleScanner>,
}
// TODO: make Engine moveable.
unsafe impl<T> Send for Engine<T> {}

impl<T> Engine<T> {
    pub fn test_discovery_callback(&mut self) {
       self.discovery_callback.on_device_update(
           &mut self.platform,
           DiscoveryResult::new(PresenceMedium::BLE, client_provider::Device::new(Vec::from([100]))));
    }
    pub fn new(
        platform: T,
        provider_rx: mpsc::Receiver<ProviderEvent>,
        discovery_callback: Box<dyn DiscoveryCallback<T>>,
        ble_scanner: Box<dyn BleScanner>,
    ) -> Self {
        Self {
            platform,
            provider_rx,
            discovery_callback,
            ble_scanner,
        }
    }
    pub fn run(&mut self) {
        info!("Run Presence Engine.");
        Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async {
                self.poll_providers().await;
            });
    }

    async fn poll_providers(&mut self) {
        // loop to receive events from Providers and process the event according to its type.
        println!("pll providers");
        while let Some(event) = self.provider_rx.recv().await {
            match event {
                ProviderEvent::DiscoveryRequest(request) => {
                    self.process_discovery_request(request);
                }
                ProviderEvent::ScanResult(result) => self.process_scan_result(result),
                ProviderEvent::Stop => {
                    info!("Engine stopped");
                    break;
                }
            }
        }
    }

    fn process_discovery_request(&self, request: PresenceDiscoveryRequest) {
        debug!("received a discovery request: {:?}.", request);
        let actions = request
            .conditions
            .iter()
            .map(|condition| condition.action)
            .collect();
        self.ble_scanner
            .start_ble_scan(ScanRequest::new(request.priority, actions));
    }

    fn process_scan_result(&mut self, scan_result: PresenceScanResult) {
        debug!("received a BLE scan result: {:?}.", scan_result);
        self.discovery_callback
            .on_device_update(&mut (self.platform), DiscoveryResult::new(
                scan_result.medium,
                Device::new(scan_result.actions),
            ));
    }
}

#[cfg(test)]
mod tests {
    use crate::ble_scan_provider::{BleScanner, ScanRequest};
    use crate::client_provider::{
        DiscoveryCallback, DiscoveryResult, PresenceDiscoveryCondition, PresenceDiscoveryRequest,
        PresenceIdentityType, PresenceMeasurementAccuracy,
    };
    use crate::PresenceEngine;

    struct Platform {}

    struct MockDiscoveryCallback {}

    impl DiscoveryCallback<Platform> for MockDiscoveryCallback {
        fn on_device_update(&self, platform: &mut Platform, result: DiscoveryResult) {}
    }

    struct MockBleScanner {}

    impl BleScanner for MockBleScanner {
        fn start_ble_scan(&self, request: ScanRequest) {
            assert_eq!(request.priority, 1);
            assert_eq!(request.actions.len(), 1);
            assert_eq!(request.actions[0], 100);
        }
    }
    #[test]
    fn test_process_discovery_request() {
        let presence_engine = PresenceEngine::new(
            Platform{},
            Box::new(MockDiscoveryCallback {}),
            Box::new(MockBleScanner {}),
        );
        let condition = PresenceDiscoveryCondition {
            action: 100,
            identity_type: PresenceIdentityType::Private,
            measurement_accuracy: PresenceMeasurementAccuracy::Unknown,
        };
        let request = PresenceDiscoveryRequest::new(1, Vec::from([condition]));
        presence_engine.engine.process_discovery_request(request);
    }
}
