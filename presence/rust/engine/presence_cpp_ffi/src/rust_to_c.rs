// Data structures passed from Rust to C.
include!(concat!(env!("OUT_DIR"), "/cpp_ffi.rs"));

use presence_core::ble_scan_provider::ScanRequest;
use presence_core::client_provider::{DiscoveryResult, PresenceMedium};

impl PresenceBleScanRequest {
    pub fn from_scan_request(scan_request: ScanRequest) -> *mut Self {
        unsafe {
            let ble_scan_request = presence_ble_scan_request_new(scan_request.priority);
            for action in scan_request.actions {
                presence_ble_scan_request_add_action(ble_scan_request, action);
            }
            ble_scan_request
        }
    }
}

impl PresenceDiscoveryResult {
    pub fn from_discovery_result(result: DiscoveryResult) -> *mut Self {
        unsafe {
            let presence_result = presence_discovery_result_new(result.medium);
            for action in result.device.actions {
                presence_discovery_result_add_action(presence_result, action);
            }
            presence_result
        }
    }
}
