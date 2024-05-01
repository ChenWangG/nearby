// Defines structures data exchanged between the Presence Engine and its client.
// This header is included by the autogenerated presence.h to define the Presence API.
#ifndef presence_data_h
#define presence_data_h

/// <div rustbindgen opaque></div>
struct PresenceDiscoveryResult {
    int priority;
};

typedef struct PresenceBleScanRequestTag {
   int priority;
} PresenceBleScanRequest;

/// <div rustbindgen opaque></div>
typedef struct PresencePlatformTag {
    void (*start_ble_scan)(PresenceBleScanRequest, void (*PlatformBleCallback)(int));
} PresencePlatform;

#endif // presence_data_h