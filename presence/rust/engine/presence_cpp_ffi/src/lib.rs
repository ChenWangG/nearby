include!(concat!(env!("OUT_DIR"), "/cpp_ffi.rs"));

use log::{debug, info};
use tokio::sync::mpsc;

use presence_core::ble_scan_provider::{
    BleScanProvider, BleScanner, PresenceScanResult, ScanRequest,
};
use presence_core::client_provider::{
    ClientProvider, DiscoveryCallback, DiscoveryResult, PresenceDiscoveryCondition,
    PresenceDiscoveryRequest, PresenceIdentityType, PresenceMeasurementAccuracy,
};

pub use presence_core::client_provider::PresenceMedium;
use presence_core::PresenceEngine;

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

struct PresenceBleScanResultBuilder {
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

pub type PresenceDiscoveryCallback = fn(*mut PresenceDiscoveryResult);
struct DiscoveryCallbackCpp {
    presence_discovery_callback: PresenceDiscoveryCallback,
}

impl DiscoveryCallback for DiscoveryCallbackCpp {
    fn on_device_update(&self, result: DiscoveryResult) {
        unsafe {
            let presence_result = presence_discovery_result_new(result.medium);
            for action in result.device.actions {
                presence_discovery_result_add_action(presence_result, action);
            }
            (self.presence_discovery_callback)(presence_result);
        }
    }
}

pub type PresenceStartBleScan = fn(*mut PresenceBleScanRequest);
struct BleScannerCpp {
    presence_start_ble_scan: PresenceStartBleScan,
}

impl BleScanner for BleScannerCpp {
    fn start_ble_scan(&self, request: ScanRequest) {
        info!("BleScanner start ble scan with request {:?}.", request);
        unsafe {
            let ble_scan_request = presence_ble_scan_request_new(request.priority);
            for action in request.actions {
                presence_ble_scan_request_add_action(ble_scan_request, action);
            }
            (self.presence_start_ble_scan)(ble_scan_request);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn presence_engine_new(
    presence_discovery_callback: PresenceDiscoveryCallback,
    presence_start_ble_scan: PresenceStartBleScan,
) -> *mut PresenceEngine {
    env_logger::init();
    let engine_ptr = Box::into_raw(Box::new(PresenceEngine::new(
        Box::new(DiscoveryCallbackCpp {
            presence_discovery_callback,
        }),
        Box::new(BleScannerCpp {
            presence_start_ble_scan,
        }),
    )));
    engine_ptr
}

#[no_mangle]
pub unsafe extern "C" fn presence_engine_run(engine: *mut PresenceEngine) {
    (*engine).run();
}
#[no_mangle]
pub unsafe extern "C" fn presence_engine_set_discovery_request(
    engine: *mut PresenceEngine,
    request: *mut PresenceDiscoveryRequest,
) {
    (*engine).set_discovery_request(*Box::from_raw(request));
}

#[no_mangle]
pub unsafe extern "C" fn presence_on_scan_result(
    engine: *mut PresenceEngine,
    scan_result: *mut PresenceScanResult,
) {
    (*engine).on_scan_result(*(Box::from_raw(scan_result)));
}

#[no_mangle]
pub extern "C" fn presence_request_builder_new(
    priority: i32,
) -> *mut PresenceDiscoveryRequestBuilder {
    Box::into_raw(Box::new(PresenceDiscoveryRequestBuilder::new(priority)))
}

#[no_mangle]
pub unsafe extern "C" fn presence_request_builder_add_condition(
    builder: *mut PresenceDiscoveryRequestBuilder,
    action: i32,
    identity_type: PresenceIdentityType,
    measurement_accuracy: PresenceMeasurementAccuracy,
) {
    (*builder).add_condition(PresenceDiscoveryCondition {
        action,
        identity_type,
        measurement_accuracy,
    });
}

#[no_mangle]
pub unsafe extern "C" fn presence_request_builder_build(
    builder: *mut PresenceDiscoveryRequestBuilder,
) -> *mut PresenceDiscoveryRequest {
    Box::into_raw(Box::new(Box::from_raw(builder).build()))
}

#[no_mangle]
pub extern "C" fn presence_ble_scan_result_builder_new(
    medium: PresenceMedium,
) -> *mut PresenceBleScanResultBuilder {
    Box::into_raw(Box::new(PresenceBleScanResultBuilder::new(medium)))
}

#[no_mangle]
pub unsafe extern "C" fn presence_ble_scan_result_builder_add_action(
    builder: *mut PresenceBleScanResultBuilder,
    action: i32,
) {
    (*builder).add_action(action);
}

#[no_mangle]
pub unsafe extern "C" fn presence_ble_scan_result_builder_build(
    builder: *mut PresenceBleScanResultBuilder,
) -> *mut PresenceScanResult {
    Box::into_raw(Box::new(Box::from_raw(builder).build()))
}

#[no_mangle]
pub unsafe extern "C" fn presence_request_debug_print(request: *const PresenceDiscoveryRequest) {
    println!("Rust FFI Lib: {:?}", *request);
}

#[no_mangle]
pub extern "C" fn presence_enum_medium_debug_print(presence_medium: PresenceMedium) {
    debug!("Medium type: {:?}", presence_medium)
}

#[cfg(test)]
mod tests {
    use crate::{
        presence_request_builder_add_condition, presence_request_builder_build,
        presence_request_builder_new,
    };
    use presence_core::client_provider::PresenceIdentityType;
    use presence_core::client_provider::PresenceMeasurementAccuracy;

    #[test]
    fn test_request_builder() {
        unsafe {
            let builder = presence_request_builder_new(1);
            presence_request_builder_add_condition(
                builder,
                10,
                PresenceIdentityType::Private,
                PresenceMeasurementAccuracy::BestAvailable,
            );
            let request = presence_request_builder_build(builder);
            assert_eq!((*request).priority, 1);
            assert_eq!((*request).conditions.len(), 1);
            assert_eq!((*request).conditions[0].action, 10);
        }
    }
}
