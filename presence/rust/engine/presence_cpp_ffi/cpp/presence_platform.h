typedef struct PresenceBleScanRequestTag {
   int priority;
} PresenceBleScanRequest;

typedef struct PresencePlatformTag {
    void (*start_ble_scan)(PresenceBleScanRequest request);
} PresencePlatform;

// Forward declaration the object which is defined in Rust.
// The object is opaque to C and used by Rust callback.
/// <div rustbindgen hide></div>
struct PresenceBleProviderCpp;

void presence_platform_init(void* platform, struct PresenceBleProviderCpp* provider);
void presence_start_ble_scan(PresenceBleScanRequest request);
void presence_register_rust_callback(int (*cb)(int));