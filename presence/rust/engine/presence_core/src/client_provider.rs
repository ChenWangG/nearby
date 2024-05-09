use crate::ProviderEvent;
use log::{debug, error, info};
use tokio::sync::mpsc;

pub trait DiscoveryCallback {
    fn on_device_update(&self, result: DiscoveryResult);
}

pub struct ClientProvider {
    provider_event_tx: mpsc::Sender<ProviderEvent>,
    discovery_callback: Box<dyn DiscoveryCallback>,
}

impl ClientProvider {
    pub fn new(
        provider_event_tx: mpsc::Sender<ProviderEvent>,
        discovery_callback: Box<dyn DiscoveryCallback>,
    ) -> Self {
        Self {
            provider_event_tx,
            discovery_callback,
        }
    }
    pub fn set_discovery_request(&self, request: PresenceDiscoveryRequest) {
        if let Err(e) = self
            .provider_event_tx
            .blocking_send(ProviderEvent::DiscoveryRequest(request))
        {
            error!("Provider callback send error: {}", e);
        } else {
            debug!("Provider callback sent an event.");
        }
    }

    pub fn on_device_update(&self, result: DiscoveryResult) {
        info!("on_device_updated.");
        self.discovery_callback.on_device_update(result);
    }
}

// The enum is annotated by repr(C) to pass through FFI.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum PresenceIdentityType {
    Private = 0,
    Trusted,
    Public,
}

// The enum is annotated by repr(C) to pass through FFI.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum PresenceMeasurementAccuracy {
    Unknown = 0,
    CoarseAccuracy,
    BestAvailable,
}

/// Struct to hold an action, identity type and their associated discovery condition.
#[derive(Clone, Copy, Debug)]
pub struct PresenceDiscoveryCondition {
    pub action: i32,
    pub identity_type: PresenceIdentityType,
    pub measurement_accuracy: PresenceMeasurementAccuracy,
}

#[derive(Debug)]
/// Struct to send a discovery request to the Engine.
pub struct PresenceDiscoveryRequest {
    pub priority: i32,
    pub conditions: Vec<PresenceDiscoveryCondition>,
}

// The enum is annotated by repr(C) to pass through FFI.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum PresenceMedium {
    Unknown = 0,
    BLE,
    WiFiRTT,
    UWB,
    MDNS,
}

pub struct DiscoveryResult {
    pub medium: PresenceMedium,
    pub actions: Vec<i32>,
}

impl DiscoveryResult {
    pub fn new(medium: PresenceMedium) -> Self {
        Self {
            medium,
            actions: Vec::new(),
        }
    }
    pub fn add_action(&mut self, action: i32) {
        self.actions.push(action);
    }
}
