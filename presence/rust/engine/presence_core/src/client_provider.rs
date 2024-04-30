use tokio::sync::mpsc;
use crate::{PresenceClientProvider, PresenceDiscoveryCallback, PresenceDiscoveryRequest, PresenceDiscoveryResult, ProviderEvent};


pub struct PresenceClient {
    provider_event_tx: mpsc::Sender<ProviderEvent>,
    discovery_callback: PresenceDiscoveryCallback,
}

impl PresenceClient {
   pub fn new( provider_event_tx: mpsc::Sender<ProviderEvent>,
               discovery_callback: PresenceDiscoveryCallback) -> Self {
      Self { provider_event_tx, discovery_callback }
   }
}
impl PresenceClientProvider for PresenceClient {
    fn set_request(&self, request: PresenceDiscoveryRequest) {
        if let Err(e) = self.provider_event_tx.blocking_send(ProviderEvent::PresenceDiscoveryRequest(request)) {
            println!("Provider callback send error: {}", e);
        } else {
            println!("Provider callback sent an event.");
        }
    }

    fn on_device_updated(&self, result: PresenceDiscoveryResult) {
        todo!()
    }
}