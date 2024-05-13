mod ble_scanner_cpp;
mod c_to_rust;
mod discovery_callback_cpp;
mod rust_to_c;

use c_to_rust::{PresenceBleScanResultBuilder, PresenceDiscoveryRequestBuilder};
use log::{debug, info};

use presence_core::ble_scan_provider::{
    BleScanProvider, BleScanner, PresenceScanResult, ScanRequest,
};
use presence_core::client_provider::{
    ClientProvider, DiscoveryCallback, DiscoveryResult, PresenceDiscoveryCondition,
    PresenceDiscoveryRequest, PresenceIdentityType, PresenceMeasurementAccuracy,
};

use crate::ble_scanner_cpp::{BleScannerCpp, PresenceStartBleScan};
use crate::discovery_callback_cpp::{DiscoveryCallbackCpp, PresenceDiscoveryCallback};
pub use presence_core::client_provider::PresenceMedium;
use presence_core::PresenceEngine;

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
