use tokio::sync::mpsc;
use crate::{PresenceDiscoveryRequest, PresenceDiscoveryResult, ProviderEvent};

pub trait PresenceDiscoveryCallback {
    fn on_device_updated(&self, result: PresenceDiscoveryResult);
}

pub struct PresenceClientProvider {
    provider_event_tx: mpsc::Sender<ProviderEvent>,
    discovery_callback: Box<dyn PresenceDiscoveryCallback>,
}

impl PresenceClientProvider {
   pub fn new( provider_event_tx: mpsc::Sender<ProviderEvent>,
               discovery_callback: Box<dyn PresenceDiscoveryCallback>) -> Self {
      Self { provider_event_tx, discovery_callback }
   }
    pub fn set_request(&self, request: PresenceDiscoveryRequest) {
        if let Err(e) = self.provider_event_tx.blocking_send(ProviderEvent::PresenceDiscoveryRequest(request)) {
            println!("Provider callback send error: {}", e);
        } else {
            println!("Provider callback sent an event.");
        }
    }

    pub fn on_device_updated(&self, result: PresenceDiscoveryResult) {
        todo!()
    }
}