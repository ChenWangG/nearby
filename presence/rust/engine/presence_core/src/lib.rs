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

pub trait PresenceBleProvider {
    fn start_ble_scan(&self, request: &PresenceDiscoveryRequest);
}

pub struct PresenceEngine {
    ble_provider: Box<dyn PresenceBleProvider>,
}

impl PresenceEngine {
    pub fn new(ble_provider: Box<dyn PresenceBleProvider>) -> Self {
        Self { ble_provider }
    }

    pub fn start_discovery(&self, request: &PresenceDiscoveryRequest) {
        println!("Rust Engine: start discovery with request: {:?}.", request);
        self.ble_provider.start_ble_scan(request);
    }
}