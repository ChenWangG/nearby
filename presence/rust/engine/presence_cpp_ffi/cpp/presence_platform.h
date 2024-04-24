typedef struct PresenceBleScanRequestTag {
   int priority;
} PresenceBleScanRequest;

/// <div rustbindgen hide></div>
typedef struct PresencePlatformTag {
    void (*start_ble_scan)(PresenceBleScanRequest, void (*PlatformBleCallback)(int));
} PresencePlatform;

// Forward declaration the object which is defined in Rust.
// The object is opaque to C and used by Rust callback.
/// <div rustbindgen hide></div>
struct PresenceBleProviderCpp;

typedef void (*BleScanCallback)(struct PresenceBleProviderCpp*, int);

void presence_platform_init(void* platform, struct PresenceBleProviderCpp* provider);
void presence_start_ble_scan(PresenceBleScanRequest request, BleScanCallback cb);