use jni::objects::JClass;
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use presence_core::client_provider::{
    PresenceDiscoveryCondition, PresenceDiscoveryRequest, PresenceIdentityType,
    PresenceMeasurementAccuracy,
};

#[derive(Debug)]
struct PresenceDiscoveryRequestBuilder {
    priority: i32,
    conditions: Vec<(i32, i32, i32)>,
}

impl PresenceDiscoveryRequestBuilder {
    pub fn into_request(&self) -> PresenceDiscoveryRequest {
        let conditions = self
            .conditions
            .iter()
            .map(|condition| PresenceDiscoveryCondition {
                action: condition.0,
                identity_type: PresenceIdentityType::from_i32(condition.1),
                measurement_accuracy: PresenceMeasurementAccuracy::from_i32(condition.2),
            })
            .collect();
        PresenceDiscoveryRequest::new(self.priority, conditions)
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceDiscoveryRequestBuilder_create(
    _env: JNIEnv,
    _class: JClass,
    priority: jint,
) -> jlong {
    println!(
        "Presece Discovery request builder with priority: {}",
        priority
    );
    Box::into_raw(Box::new(PresenceDiscoveryRequestBuilder {
        priority,
        conditions: Vec::new(),
    })) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceDiscoveryRequestBuilder_addCondition(
    _env: JNIEnv,
    _class: JClass,
    builderInRust: jlong,
    action: jint,
    identityType: jint,
    measurementAccuracy: jint,
) {
    println!("Presence Discovery request builder add condition with action: {}, identityType: {}, accuracy: {}",
             action, identityType, measurementAccuracy);
    let builder = &mut *(builderInRust as *mut PresenceDiscoveryRequestBuilder);
    builder
        .conditions
        .push((action, identityType, measurementAccuracy));
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceDiscoveryRequestBuilder_build(
    _env: JNIEnv,
    _class: JClass,
    builderInRust: jlong,
) -> jlong {
    println!("PresenceDiscoveryRequest Builder build");
    // Consume the builder.
    let builder = Box::from_raw(builderInRust as *mut PresenceDiscoveryRequestBuilder);
    Box::into_raw(Box::new(builder.into_request())) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceDiscoveryRequestBuilder_debug(
    _env: JNIEnv,
    _class: JClass,
    builderInRust: jlong,
) {
    println!(
        "Presence Discovery request builder debug: {:?}",
        *(builderInRust as *const PresenceDiscoveryRequestBuilder)
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceDiscoveryRequestBuilder_debugResult(
    _env: JNIEnv,
    _class: JClass,
    resultInRust: jlong,
) {
    println!(
        "Presence Discovery request builder debug: {:?}",
        *(resultInRust as *const PresenceDiscoveryRequest)
    );
}

#[cfg(test)]
mod tests {
    use crate::presence_discovery_request_builder::PresenceDiscoveryRequestBuilder;
    use presence_core::client_provider::{PresenceIdentityType, PresenceMeasurementAccuracy};

    #[test]
    fn test_build_discovery_request() {
        assert_eq!(1, 1);
        let builder = PresenceDiscoveryRequestBuilder {
            priority: 0,
            conditions: Vec::from([(1, 1, 1)]),
        };
        let request = builder.into_request();
        assert_eq!(request.priority, 0);
        assert_eq!(request.conditions.len(), 1);
        assert_eq!(request.conditions[0].action, 1);
        assert_eq!(
            request.conditions[0].identity_type,
            PresenceIdentityType::Private
        );
        assert_eq!(
            request.conditions[0].measurement_accuracy,
            PresenceMeasurementAccuracy::CoarseAccuracy
        );
    }
}
