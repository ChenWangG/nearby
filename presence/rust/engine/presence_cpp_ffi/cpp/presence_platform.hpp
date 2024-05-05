// FFI the platform C APIs to Rust.
#include "../presence_data.h"

// Forward declaration the object which is defined in Rust.
// The object is opaque to C and used by Rust callback.
/// <div rustbindgen hide></div>
struct PresenceEngine;

typedef void (*BleScanCallback)(struct PresenceEngine*, int);

extern "C" {
void presence_platform_init(void* platform, struct PresenceEngine* engine);
void presence_start_ble_scan(PresenceBleScanRequest request,
                             BleScanCallback cb);
PresenceDiscoveryResult* presence_discovery_result_new();
}