#include <stdio.h>
#include "presence_platform.h"

PresencePlatform* platform_ptr;

void presence_platform_init(void* platform, void* provider) {

}

void presence_start_ble_scan(PresenceBleScanRequest request) {
    printf("C presence_provider: start_ble_scan with priority %d\n", request.priority);
    platform_ptr->start_ble_scan(request);
}

void presence_register_provider(void* provider) {
    platform_ptr = (PresencePlatform*) provider;
}

void presence_register_rust_callback(int (*cb)(int)) {
    cb(101);
}