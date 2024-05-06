// Defines structures data exchanged between the Presence Engine and its client.
#ifndef presence_data_h
#define presence_data_h

#include "presence_enums.h"

typedef struct {
    int* actions;
    int  actions_size;
} PresenceDevice;

struct PresenceDiscoveryResult {
   enum PresenceMedium medium;
   PresenceDevice device;
};

struct PresenceBleScanRequest {
   int priority;
};

struct PresencePlatform {
    void (*start_ble_scan)(struct PresenceBleScanRequest*, void (*PlatformBleCallback)(int));
};

#endif // presence_data_h