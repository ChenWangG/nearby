// Defines structures data exchanged between the Presence Engine and its client.
#ifndef presence_data_h
#define presence_data_h

typedef enum PresenceMedium {
    RRESENCE_MEDIUM_UNKNOWN = 0,
    PRESENCE_MEDIUM_BLE,
    PRESENCE_MEDIUM_WIFI_RTT,
    PRESENCE_MEDIUM_UWB,
    PRESENCE_MEDIUM_MDNS,
} PresenceMedium;

typedef struct {
    int* actions;
    int  actions_size;
} PresenceDevice;

struct PresenceDiscoveryResult {
   PresenceMedium medium;
   PresenceDevice device;
};

struct PresenceBleScanRequest {
   int priority;
};

struct PresencePlatform {
    void (*start_ble_scan)(struct PresenceBleScanRequest*, void (*PlatformBleCallback)(int));
};

#endif // presence_data_h