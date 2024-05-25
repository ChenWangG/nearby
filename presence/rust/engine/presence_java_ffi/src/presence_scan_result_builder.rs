use jni::objects::JClass;
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use presence_core::ble_scan_provider::PresenceScanResult;
use presence_core::client_provider::PresenceMedium;

#[derive(Debug)]
struct PresenceScanResultBuilder {
    medium: i32,
    actions: Vec<i32>,
}

impl PresenceScanResultBuilder {
    pub fn into_result(&self) -> PresenceScanResult {
        PresenceScanResult::new(PresenceMedium::from_i32(self.medium), self.actions.to_vec())
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceScanResultBuilder_create(
    _env: JNIEnv,
    _class: JClass,
    medium: jint,
) -> jlong {
    println!("Presence scan result builder with medium: {}", medium);
    Box::into_raw(Box::new(PresenceScanResultBuilder {
        medium,
        actions: Vec::new(),
    })) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceScanResultBuilder_addAction(
    _env: JNIEnv,
    _class: JClass,
    builderInRust: jlong,
    action: jint,
) {
    println!("Presence scan result builder add action: {}", action);
    let builder = &mut *(builderInRust as *mut PresenceScanResultBuilder);
    builder.actions.push(action);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceScanResultBuilder_build(
    _env: JNIEnv,
    _class: JClass,
    builderInRust: jlong,
) -> jlong {
    println!("PresenceScanResult Builder build");
    // Consume the builder.
    let builder = Box::from_raw(builderInRust as *mut PresenceScanResultBuilder);
    Box::into_raw(Box::new(builder.into_result())) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceScanResultBuilder_debug(
    _env: JNIEnv,
    _class: JClass,
    builderInRust: jlong,
) {
    println!(
        "Presence scan result builder debug: {:?}",
        *(builderInRust as *const PresenceScanResultBuilder)
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_presence_engine_PresenceScanResultBuilder_debugResult(
    _env: JNIEnv,
    _class: JClass,
    resultInRust: jlong,
) {
    println!(
        "Presence scan result debug: {:?}",
        *(resultInRust as *const PresenceScanResult)
    );
}

#[cfg(test)]
mod tests {
    use crate::presence_scan_result_builder::PresenceScanResultBuilder;
    use presence_core::client_provider::{
        PresenceIdentityType, PresenceMeasurementAccuracy, PresenceMedium,
    };

    #[test]
    fn test_build_discovery_request() {
        assert_eq!(1, 1);
        let builder = PresenceScanResultBuilder {
            medium: 1,
            actions: Vec::from([101]),
        };
        let result = builder.into_result();
        assert_eq!(result.medium, PresenceMedium::BLE);
        assert_eq!(result.actions.len(), 1);
        assert_eq!(result.actions[0], 101);
    }
}
