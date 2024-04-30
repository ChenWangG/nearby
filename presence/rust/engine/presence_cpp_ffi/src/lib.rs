include!(concat!(env!("OUT_DIR"), "/presence_platform.rs"));
include!(concat!(env!("OUT_DIR"), "/presence_client.rs"));

use tokio::sync::mpsc;

use presence_core::client_provider::PresenceClient;
pub use presence_core::{
    PresenceBleProvider, PresenceDiscoveryCondition, PresenceDiscoveryRequest,
    PresenceIdentityType, PresenceMeasurementAccuracy,
};
use presence_core::{PresenceDiscoveryCallback, PresenceEngine, ProviderEvent};

pub struct PresenceBleProviderCpp {
    discovery_callback: Option<PresenceDiscoveryCallback>,
}

impl PresenceBleProviderCpp {
    fn new() -> Self {
        Self {
            discovery_callback: None,
        }
    }

    fn ble_scan_callback(&self, priority: i32) {
        println!(
            "PresenceBleProviderCpp: ble_scan_callback with priority: {}",
            priority
        );
        self.discovery_callback.unwrap()(priority);
    }
}

unsafe extern "C" fn ble_scan_callback(ble_provider: *mut PresenceBleProviderCpp, priority: i32) {
    println!("ble_scan_callback");
    (*ble_provider).ble_scan_callback(priority);
}

impl PresenceBleProvider for PresenceBleProviderCpp {
    fn start_ble_scan(
        &mut self,
        request: &PresenceDiscoveryRequest,
        discovery_callback: PresenceDiscoveryCallback,
    ) {
        println!("Rust Provider: start ble scan.");
        self.discovery_callback = Some(discovery_callback);
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

#[no_mangle]
pub unsafe extern "C" fn presence_engine_new(
    platform: *mut ::std::os::raw::c_void,
    discovery_callback: PresenceDiscoveryCallback,
) -> *mut PresenceEngine {
    // Channel for Providers to send events to Engine.
    let (provider_event_tx, provider_event_rx) = mpsc::channel::<ProviderEvent>(100);
    let mut ble_provider_boxed = Box::new(PresenceBleProviderCpp::new());
    presence_platform_init(platform, &mut *ble_provider_boxed);
    Box::into_raw(Box::new(PresenceEngine::new(
        provider_event_tx,
        provider_event_rx,
        discovery_callback,
        ble_provider_boxed,
    )))
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
pub unsafe extern "C" fn presence_engine_start_discovery(
    engine_ptr: *mut PresenceEngine,
    request_ptr: *const PresenceDiscoveryRequest,
    discovery_callback: PresenceDiscoveryCallback,
) {
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
