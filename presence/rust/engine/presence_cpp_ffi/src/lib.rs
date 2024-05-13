// Define Presence Engine APIs to be FFIed to C.
mod ble_scanner_cpp;
mod c_to_rust;
mod discovery_callback_cpp;
mod rust_to_c;

use presence_core::PresenceEngine;
use presence_core::ble_scan_provider::PresenceScanResult;
use presence_core::client_provider::PresenceDiscoveryRequest;

use crate::ble_scanner_cpp::{BleScannerCpp, PresenceStartBleScan};
use crate::discovery_callback_cpp::{DiscoveryCallbackCpp, PresenceDiscoveryCallback};

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
pub unsafe extern "C" fn presence_engine_stop(engine: *mut PresenceEngine) {
    (*engine).stop();
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
