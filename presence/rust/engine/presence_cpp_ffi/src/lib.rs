include!(concat!(env!("OUT_DIR"), "/presence_provider.rs"));

pub use presence_core::{
    PresenceBleProvider,
    PresenceDiscoveryCondition, PresenceDiscoveryRequest, PresenceIdentityType,
    PresenceMeasurementAccuracy,
};
use presence_core::PresenceEngine;

pub struct PresenceBleProviderCpp {}

impl PresenceBleProviderCpp {
     fn new() -> Self { Self {} }
}

impl PresenceBleProvider for PresenceBleProviderCpp {
    fn start_ble_scan(&self, request: &PresenceDiscoveryRequest) {
        println!("Rust Provider: start ble scan.");
        unsafe {
            presence_start_ble_scan(PresenceBleScanRequest{ priority: request.priority })
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
pub unsafe extern "C" fn presence_engine_new(provider: *mut ::std::os::raw::c_void) -> *mut PresenceEngine {
    presence_register_provider(provider);
   Box::into_raw(Box::new(PresenceEngine::new(Box::new(PresenceBleProviderCpp::new()))))
}

#[no_mangle]
pub unsafe extern "C" fn presence_engine_start_discovery(engine_ptr: *mut PresenceEngine,
                                                  request_ptr: *const PresenceDiscoveryRequest) {
    (*engine_ptr).start_discovery(&*request_ptr);
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
) -> *const PresenceDiscoveryRequest {
    Box::into_raw(Box::new(Box::from_raw(builder).build()))
}

#[no_mangle]
pub unsafe extern "C" fn presence_request_debug_print(
    request: *const PresenceDiscoveryRequest,
) {
    println!("Rust FFI Lib: {:?}", *request);
}

#[cfg(test)]
mod tests {
    use crate::{hello, presence_request_builder_add_condition, presence_request_builder_build, presence_request_builder_new};
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
