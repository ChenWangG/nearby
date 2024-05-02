include!(concat!(env!("OUT_DIR"), "/presence_platform.rs"));
// include!(concat!(env!("OUT_DIR"), "/presence_client.rs"));

use log::info;
use tokio::sync::mpsc;

use presence_core::client_provider::{PresenceClientProvider, DiscoveryCallback};
use presence_core::ble_scan_provider::{BleScanner, BleScanProvider, BleScanResult};

pub use presence_core::{
    PresenceDiscoveryCondition, PresenceDiscoveryRequest,
    PresenceIdentityType, PresenceMeasurementAccuracy,
};
use presence_core::{DiscoveryResult, PresenceEngine, ProviderEvent};

pub struct PresenceBleProviderCpp {}

impl PresenceBleProviderCpp {
    fn new() -> Self {
        Self {
        }
    }

    fn ble_scan_callback(&self, priority: i32) {
        println!(
            "PresenceBleProviderCpp: ble_scan_callback with priority: {}",
            priority
        );
        // self.discovery_callback.unwrap()(priority);
    }
}

impl PresenceBleProviderCpp {
    fn start_ble_scan(
        &mut self,
        request: &PresenceDiscoveryRequest,
    ) {
        println!("Rust Provider: start ble scan.");
        /*
        self.discovery_callback = Some(discovery_callback);
        unsafe {
            presence_start_ble_scan(
                PresenceBleScanRequest {
                    priority: request.priority,
                },
                Some(ble_scan_callback),
            );
        }
         */
    }
}

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

    pub fn build(&self) -> PresenceDiscoveryRequest {
        PresenceDiscoveryRequest {
            priority: self.priority,
            conditions: self.conditions.to_vec(),
        }
    }
}

pub type PresenceDiscoveryCallback = fn(PresenceDiscoveryResult);
struct DiscoveryCallbackCpp {
     presence_discovery_callback: PresenceDiscoveryCallback,
}

impl DiscoveryCallback for DiscoveryCallbackCpp {
    fn on_device_updated(&self, result: DiscoveryResult) {
        (self.presence_discovery_callback)(PresenceDiscoveryResult{});
    }
}

struct BleScannerCpp {}

impl BleScanner for BleScannerCpp {
    fn start_ble_scan(&self, request: PresenceDiscoveryRequest) {
        info!("BleScanner start ble scan with request {:?}.", request);
        unsafe {
            presence_start_ble_scan(
                PresenceBleScanRequest {
                    priority: request.priority,
                },
                Some(ble_scan_callback),
            );
        }
    }
}

unsafe extern "C" fn ble_scan_callback(engine: *mut PresenceEngine, priority: i32) {
    info!("ble_scan_callback");
    (*engine).get_ble_scan_provider().on_scan_result(BleScanResult {priority});
    // let _provider = (*engine).get_ble_scan_provider();
}

#[no_mangle]
pub unsafe extern "C" fn presence_engine_new(
    platform: *mut ::std::os::raw::c_void,
    presence_discovery_callback: PresenceDiscoveryCallback,
) -> *mut PresenceEngine {
    env_logger::init();
    info!("presence_engine_new.");
    // Channel for Providers to send events to Engine.
    let (provider_event_tx, provider_event_rx) = mpsc::channel::<ProviderEvent>(100);
    let mut ble_provider_boxed = Box::new(PresenceBleProviderCpp::new());
    let engine_ptr = Box::into_raw(Box::new(PresenceEngine::new(
        provider_event_tx,
        provider_event_rx,
        Box::new(DiscoveryCallbackCpp { presence_discovery_callback }),
        Box::new(BleScannerCpp {}),
    )));
    presence_platform_init(platform, engine_ptr);
    engine_ptr
}

#[no_mangle]
pub unsafe extern "C" fn presence_engine_run(engine: *mut PresenceEngine) {
    println!("start engine.");
    (*engine).run();
}
#[no_mangle]
pub unsafe extern "C" fn presence_engine_set_request(
    engine: *mut PresenceEngine,
    request: *mut PresenceDiscoveryRequest,
) {
    (*engine)
        .get_client_provider()
        .set_request(*Box::from_raw(request));
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
    action: u32,
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
pub unsafe extern "C" fn presence_request_debug_print(request: *const PresenceDiscoveryRequest) {
    println!("Rust FFI Lib: {:?}", *request);
}

#[cfg(test)]
mod tests {
    use crate::{
        hello, presence_request_builder_add_condition, presence_request_builder_build,
        presence_request_builder_new,
    };
    use presence_core::{PresenceIdentityType, PresenceMeasurementAccuracy};

    #[test]
    fn test_hello() {
        unsafe {
            hello();
        }
    }

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
