#include <stdio.h>
#include <stdlib.h>
#include "../presence_data.h"
#include "presence_platform.hpp"

PresencePlatform* platform_ptr;
// Object opaque to C codes and used within the Rust callback.
struct PresenceEngine* engine_ptr;

BleScanCallback ble_scan_callback;

void presence_platform_init(void* platform, struct PresenceEngine* engine) {
    platform_ptr = (PresencePlatform*) platform;
    engine_ptr = engine;
}

void platform_ble_scan_callback(int priority) {
    ble_scan_callback(engine_ptr, priority);
}

struct PresenceBleScanRequest* presence_ble_scan_request_new(int priority) {
  PresenceBleScanRequest* request = new PresenceBleScanRequest();
  request->priority = priority;
  return request;
}

void presence_ble_scan_request_free(struct PresenceBleScanRequest* request) {
  delete request;
}

void presence_start_ble_scan(struct PresenceBleScanRequest *request, BleScanCallback cb) {
    printf("C presence_provider: start_ble_scan with priority %d\n", request->priority);
    ble_scan_callback = cb;
    platform_ptr->start_ble_scan(request, platform_ble_scan_callback);
}

struct PresenceDiscoveryResult* presence_discovery_result_new() {
    // struct PresenceDevice presence_device = { .actions = NULL, .actions_size = 20 };
    PresenceDiscoveryResult *result = (PresenceDiscoveryResult*)malloc(sizeof(PresenceDiscoveryResult));
    result->medium = PRESENCE_MEDIUM_BLE;
    return result;
}