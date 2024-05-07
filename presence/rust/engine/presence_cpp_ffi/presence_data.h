// Defines structures data exchanged between the Presence Engine and its client.
#ifndef presence_data_h
#define presence_data_h

#include "presence_enums.h"
#include <cstdint>
#include <vector>

typedef struct {
    std::vector<int> actions;
} PresenceDevice;

struct PresenceDiscoveryResult {
   enum PresenceMedium medium;
   PresenceDevice device;
};

struct PresenceBleScanRequest {
    int priority;
    std::vector<uint32_t> actions;
};

// Struct to hook system APIs from different platforms.
struct PresencePlatform {
    void (*start_ble_scan)(struct PresenceBleScanRequest*);
};

#endif // presence_data_h