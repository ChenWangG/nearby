pub mod client_provider;

use tokio::sync::mpsc;
use tokio::runtime::Builder;
use crate::client_provider::{PresenceClientProvider, PresenceDiscoveryCallback};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PresenceIdentityType {
    Private = 0,
    Trusted,
    Public,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PresenceMeasurementAccuracy {
    Unknown = 0,
    CoarseAccuracy,
    BestAvailable,
}
/// Struct to hold an action, identity type and their associated discovery condition.
#[derive(Clone, Copy, Debug)]
pub struct PresenceDiscoveryCondition {
    pub action: u32,
    pub identity_type: PresenceIdentityType,
    pub measurement_accuracy: PresenceMeasurementAccuracy,
}

#[derive(Debug)]
/// Struct to send a discovery request to the Engine.
pub struct PresenceDiscoveryRequest {
    pub priority: i32,
    pub conditions: Vec<PresenceDiscoveryCondition>,
}

pub struct PresenceDiscoveryResult {}

// pub type PresenceDiscoveryCallback = fn(i32);

pub enum ProviderEvent {
    PresenceDiscoveryRequest(PresenceDiscoveryRequest),
}

pub trait PresenceBleProvider {
    // TODO: refactor to use BLE scan request and callback.
    fn start_ble_scan(&mut self, request: &PresenceDiscoveryRequest);
}

pub struct PresenceEngine {
    // Receive events from Providers.
    provider_rx: mpsc::Receiver<ProviderEvent>,
    client_provider: PresenceClientProvider,
    ble_provider: Box<dyn PresenceBleProvider>,
}

impl PresenceEngine {
    pub fn new(provider_tx: mpsc::Sender<ProviderEvent>,
               provider_rx: mpsc::Receiver<ProviderEvent>,
               discovery_callback: Box<dyn PresenceDiscoveryCallback>,
               ble_provider: Box<dyn PresenceBleProvider> ) -> Self {
        let client_provider = PresenceClientProvider::new(provider_tx, discovery_callback);
        Self { provider_rx, client_provider, ble_provider }
    }

    pub fn get_client_provider(&self) -> &PresenceClientProvider {
        &self.client_provider
    }

    pub fn run(&mut self) {
        println!("Presence Engine run.");
        Builder::new_current_thread()
            .build()
            .unwrap().block_on(async move {
                self.poll_providers().await; });
    }

    async fn poll_providers(&mut self) {
        // loop to receive events from Providers and process the event according to its type.
        loop {
            println!("loop to receive provider events.");
            if let Some(event) = self.provider_rx.recv().await {
                match event {
                    ProviderEvent::PresenceDiscoveryRequest(request) => {
                        println!("received discovery request: {:?}.", request);
                    }
                }
            }
        }
    }
}
