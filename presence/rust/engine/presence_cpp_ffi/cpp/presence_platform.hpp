// FFI the platform C APIs to Rust.
#include "../presence_enums.h"

// Forward declaration of structs already defined in Rust while opaque to C.
/// <div rustbindgen hide></div>
struct PresenceEngine;

// Forward declaration of structs opaque to Rust.
struct PresencePlatform;
struct PresenceBleScanRequest;

extern "C" {
// Both the platform (for system API) and the engine are glued in the platform
// C implementation.
void presence_platform_init(PresencePlatform* platform, struct PresenceEngine* engine);

struct PresenceBleScanRequest* presence_ble_scan_request_new(int priority);
void presence_ble_scan_request_add_action(PresenceBleScanRequest* request, int action);

void presence_start_ble_scan(struct PresenceBleScanRequest* request);

// Build discovery result for the client.
struct PresenceDiscoveryResult* presence_discovery_result_new(enum PresenceMedium medium);
void presence_discovery_result_add_action(PresenceDiscoveryResult* result,  int action);
}