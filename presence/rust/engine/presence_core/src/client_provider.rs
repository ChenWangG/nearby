use crate::ProviderEvent;
use log::{debug, error};
use tokio::sync::mpsc;

// Implemented by the client to receive discovery results.
pub trait DiscoveryCallback<T> {
    fn on_device_update(&self, platform: &T, result: DiscoveryResult);
}

// Bridge a client with the Engine.
// Receives discovery requests through set_discovery_request().
// Returns discovery results through the discovery_callback.
#[derive(Debug)]
pub struct ClientProvider {
    provider_event_tx: mpsc::Sender<ProviderEvent>,
}

impl ClientProvider {
    pub fn new(provider_event_tx: mpsc::Sender<ProviderEvent>) -> Self {
        Self { provider_event_tx }
    }
    pub fn set_discovery_request(&self, request: PresenceDiscoveryRequest) {
        println!("set discovery request.");
        self.send_event(ProviderEvent::DiscoveryRequest(request));
    }

    pub fn stop(&self) {
        self.send_event(ProviderEvent::Stop);
    }
    fn send_event(&self, event: ProviderEvent) {
        if let Err(e) = self.provider_event_tx.blocking_send(event) {
            error!("Provider callback send error: {}", e);
        } else {
            debug!("Provider callback sent an event.");
        }
    }
}

// The enum is annotated by repr(C) to pass through FFI.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum PresenceIdentityType {
    Private = 1,
    Trusted,
    Public,
}

impl PresenceIdentityType {
    pub fn from_i32(value: i32) -> PresenceIdentityType {
        match value {
            1 => PresenceIdentityType::Private,
            2 => PresenceIdentityType::Trusted,
            3 => PresenceIdentityType::Public,
            _ => panic!("Unknown PresenceIdentityType value: {}", value),
        }
    }
}

// The enum is annotated by repr(C) to pass through FFI.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum PresenceMeasurementAccuracy {
    Unknown = 0,
    CoarseAccuracy,
    BestAvailable,
}
impl PresenceMeasurementAccuracy {
    pub fn from_i32(value: i32) -> PresenceMeasurementAccuracy {
        match value {
            0 => PresenceMeasurementAccuracy::Unknown,
            1 => PresenceMeasurementAccuracy::CoarseAccuracy,
            2 => PresenceMeasurementAccuracy::BestAvailable,
            _ => panic!("Unknown PresenceMeasurementAccuracy value: {}", value),
        }
    }
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

impl PresenceDiscoveryRequest {
    pub fn new(priority: i32, conditions: Vec<PresenceDiscoveryCondition>) -> Self {
        Self {
            priority,
            conditions,
        }
    }
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

impl  PresenceMedium {
    pub fn from_i32(value: i32) -> PresenceMedium {
        match value {
            0 => PresenceMedium::Unknown,
            1 => PresenceMedium::BLE,
            2 => PresenceMedium::WiFiRTT,
            3 => PresenceMedium::UWB,
            4 => PresenceMedium::MDNS,
            _ => panic!("Unknown PresenceMedium value: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub actions: Vec<i32>,
}

impl Device {
    pub fn new(actions: Vec<i32>) -> Self {
        Self { actions }
    }
}
#[derive(Debug)]
pub struct DiscoveryResult {
    pub medium: PresenceMedium,
    pub device: Device,
}

impl DiscoveryResult {
    pub fn new(medium: PresenceMedium, device: Device) -> Self {
        Self { medium, device }
    }
}
