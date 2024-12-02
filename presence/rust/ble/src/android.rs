use crate::scan_result::ScanResult;
use crate::{BleScanRequest, BleScanResult, BleScanner, ScanCallback, ScanCallbackBox, Scanner};
use jni::objects::{GlobalRef, JClass};
use jni::sys::jlong;
use jni::{JNIEnv, JavaVM};
use log::{debug, info};

static BLE_CLASS: &str = "com/google/nearby/ble/BleScanner";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/ble/BleScanner;";
static START_SIGNATURE: &str = "(J)V";

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        info!("(PresenceRust) BleScanner start.");
        let scan_callback_box = ScanCallbackBox {
            scan_callback: Box::new(callback),
        };
        let callback_ptr = Box::into_raw(Box::new(scan_callback_box)) as jlong;
        info!("BleScanner Lib start with callback addr: {}.", callback_ptr);
        let mut env = self.jvm.get_env().unwrap();
        env.call_method(
            self.java_ble_scanner.as_obj(),
            "start",
            START_SIGNATURE,
            &[callback_ptr.into()],
        )
        .unwrap();
    }
}

impl BleScanner {
    pub fn new(jvm: JavaVM) -> Self {
        let mut env = jvm.get_env().unwrap();
        let ble_scanner = env
            .call_static_method(BLE_CLASS, "build", BUILD_SIGNATURE, &[])
            .unwrap()
            .l()
            .unwrap();
        let java_ble_scanner = env.new_global_ref(ble_scanner).unwrap();
        Self {
            jvm,
            java_ble_scanner,
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_com_google_nearby_ble_BleScanner_onScanResult(
    mut env: JNIEnv,
    _class: JClass,
    callback_ptr: jlong,
    result: jlong,
) {
    info!(
        "(PresenceRust) BleScanner_onScanResult with callback_ptr: {}.",
        callback_ptr
    );
    let scan_callback_box_ptr = callback_ptr as *mut ScanCallbackBox;
    let result_ptr = result as *mut ScanResult;
    let service_data = (&*result_ptr).service_data().clone();
    debug!("BleScanner onScanResult:");
    for data in &service_data {
        debug!("{}", data)
    }
    &(*scan_callback_box_ptr)
        .scan_callback
        .on_update(BleScanResult::new(1, service_data));
}
