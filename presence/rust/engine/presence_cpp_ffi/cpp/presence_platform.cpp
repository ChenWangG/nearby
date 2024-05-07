#include <stdio.h>
#include <stdlib.h>
#include "../presence_data.h"
#include "presence_platform.hpp"

PresencePlatform* platform_ptr;
// Object opaque to C codes and used within the Rust callback.
struct PresenceEngine* engine_ptr;

void presence_platform_init(PresencePlatform* platform, struct PresenceEngine* engine) {
    platform_ptr = (PresencePlatform*) platform;
    engine_ptr = engine;
}

struct PresenceBleScanRequest* presence_ble_scan_request_new(int priority) {
  PresenceBleScanRequest* request = new PresenceBleScanRequest();
  request->priority = priority;
  return request;
}

void presence_ble_scan_request_add_action(PresenceBleScanRequest* request, int action) {
   request->actions.push_back(action);
}

void presence_ble_scan_request_free(struct PresenceBleScanRequest* request) {
  delete request;
}

void presence_start_ble_scan(struct PresenceBleScanRequest *request) {
    printf("C presence_provider: start_ble_scan with priority %d\n", request->priority);
    platform_ptr->start_ble_scan(request);
}

struct PresenceDiscoveryResult* presence_discovery_result_new(enum PresenceMedium medium) {
    // struct PresenceDevice presence_device = { .actions = NULL, .actions_size = 20 };
    // PresenceDiscoveryResult *result = (PresenceDiscoveryResult*)malloc(sizeof(PresenceDiscoveryResult));
    PresenceDiscoveryResult *result = new PresenceDiscoveryResult();
    result->medium = medium;
    return result;
}

void presence_discovery_result_add_action(PresenceDiscoveryResult* result,  int action) {
   result->device.actions.push_back(action);
}