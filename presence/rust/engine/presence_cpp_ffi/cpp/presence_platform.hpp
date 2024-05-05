// FFI the platform C APIs to Rust.

// Forward declaration of structs already defined in Rust while opaque to C.
/// <div rustbindgen hide></div>
struct PresenceEngine;

// Forward declaration of structs opaque to Rust.
struct PresenceBleScanRequest;

typedef void (*BleScanCallback)(struct PresenceEngine*, int);

extern "C" {
// Both the platform (for system API) and the engine are glued in the platform
// C implementation.
void presence_platform_init(void* platform, struct PresenceEngine* engine);

struct PresenceBleScanRequest* presence_ble_scan_request_new(int priority);
void presence_start_ble_scan(struct PresenceBleScanRequest* request,
                             BleScanCallback cb);

// Build discovery result for the client.
struct PresenceDiscoveryResult* presence_discovery_result_new();
}