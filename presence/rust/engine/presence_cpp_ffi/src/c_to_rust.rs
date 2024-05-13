// Data structures passed from C to Rust.

use presence_core::ble_scan_provider::PresenceScanResult;
use presence_core::client_provider::{PresenceDiscoveryCondition, PresenceDiscoveryRequest, PresenceMedium};

pub struct PresenceDiscoveryRequestBuilder {
    priority: i32,
    conditions: Vec<PresenceDiscoveryCondition>,
}

impl PresenceDiscoveryRequestBuilder {
    pub fn new(priority: i32) -> Self {
        Self {
            priority,
            conditions: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: PresenceDiscoveryCondition) {
        self.conditions.push(condition);
    }

    // Builder itself is consumed to the result.
    pub fn build(self) -> PresenceDiscoveryRequest {
        PresenceDiscoveryRequest::new(self.priority, self.conditions)
    }
}

pub struct PresenceBleScanResultBuilder {
    pub medium: PresenceMedium,
    actions: Vec<i32>,
}

impl PresenceBleScanResultBuilder {
    pub fn new(medium: PresenceMedium) -> Self {
        Self {
            medium,
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: i32) {
        self.actions.push(action);
    }

    pub fn build(&self) -> PresenceScanResult {
        PresenceScanResult {
            medium: self.medium,
            actions: self.actions.to_vec(),
        }
    }
}

