use jni::objects::JObject;
use jni::sys::jint;
use jni::JavaVM;
use presence_core::ble_scan_provider::ScanRequest;

static CLASS_ENGINE: &str = "com/google/nearby/presence/engine/PresenceBleScanRequest";
static TO_BUILDER_SIGNATURE: &str =
    "(I)Lcom/google/nearby/presence/engine/PresenceBleScanRequest$Builder;";
static BUILD_SIGNATURE: &str = "()Lcom/google/nearby/presence/engine/PresenceBleScanRequest;";

pub struct PresenceScanRequestBuilder<'a> {
    jvm: &'a JavaVM,
    builder: JObject<'a>,
}

impl<'a> PresenceScanRequestBuilder<'a> {
    pub fn new(jvm: &'a JavaVM, prority: i32) -> PresenceScanRequestBuilder<'a> {
        let mut env = jvm.get_env().unwrap();
        let builder = env
            .call_static_method(
                CLASS_ENGINE,
                "toBuilder",
                TO_BUILDER_SIGNATURE,
                &[(prority as jint).into()],
            )
            .unwrap()
            .l()
            .unwrap();
        Self { jvm, builder }
    }

    pub fn add_action(&mut self, action: i32) {
        let mut env = self.jvm.get_env().unwrap();
        env.call_method(
            &self.builder,
            "addAction",
            "(I)V",
            &[(action as jint).into()],
        )
        .unwrap();
    }

    pub fn build(&mut self) -> JObject<'a> {
        self.jvm
            .get_env()
            .unwrap()
            .call_method(&self.builder, "build", BUILD_SIGNATURE, &[])
            .unwrap()
            .l()
            .unwrap()
    }

    pub fn from_scan_request(jvm: &'a JavaVM, scan_request: ScanRequest) -> Self {
        let mut builder = PresenceScanRequestBuilder::new(jvm, scan_request.priority);
        for action in scan_request.actions {
            builder.add_action(action);
        }
        builder
    }
}
