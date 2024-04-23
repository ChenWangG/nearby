#include <stdio.h>
#include "presence_platform.h"

PresencePlatform* platform_ptr;
// Object opaque to C codes and used within the Rust callback.
struct PresenceBleProviderCpp* provider_ptr;

void presence_platform_init(void* platform, struct PresenceBleProviderCpp* provider) {
    platform_ptr = (PresencePlatform*) platform;
    provider_ptr = provider;
}

void presence_start_ble_scan(PresenceBleScanRequest request, BleScanCallback cb) {
    printf("C presence_provider: start_ble_scan with priority %d\n", request.priority);
    platform_ptr->start_ble_scan(request);
    cb(provider_ptr);
}

void presence_register_rust_callback(int (*cb)(int)) {
    cb(101);
}