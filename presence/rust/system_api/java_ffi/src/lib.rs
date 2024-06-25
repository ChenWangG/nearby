use system_api_core::{Ble, BleScanRequest};
use jni::JNIEnv;

pub struct JavaBle;

impl Ble for JavaBle {
    type LanguageEnv<'a> = JNIEnv<'a>;
    fn start_scan<'a>(&self, language_env: Self::LanguageEnv<'a>, request: BleScanRequest) {
        todo!()
    }
}
