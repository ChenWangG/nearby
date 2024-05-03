// Defines structures data exchanged between the Presence Engine and its client.
// This header is included by the autogenerated presence.h to define the Presence API.
#ifndef presence_data_h
#define presence_data_h

/// <div rustbindgen opaque></div>
typedef enum PresenceMedium {
    RRESENCE_MEDIUM_UNKNOWN = 0,
    PRESENCE_MEDIUM_BLE,
    PRESENCE_MEDIUM_WIFI_RTT,
    PRESENCE_MEDIUM_UWB,
    PRESENCE_MEDIUM_MDNS,
} PresenceMedium;

/// <div rustbindgen opaque></div>
typedef struct {
    int* actions;
    int  actions_size;
} PresenceDevice;

/// <div rustbindgen opaque></div>
typedef struct {
   PresenceMedium medium;
   PresenceDevice device;
} PresenceDiscoveryResult;

typedef struct PresenceBleScanRequestTag {
   int priority;
} PresenceBleScanRequest;

/// <div rustbindgen opaque></div>
typedef struct PresencePlatformTag {
    void (*start_ble_scan)(PresenceBleScanRequest, void (*PlatformBleCallback)(int));
} PresencePlatform;

#endif // presence_data_h