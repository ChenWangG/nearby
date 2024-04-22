#include <stdio.h>
#include "presence_provider.h"

PresenceProvider* provider_ptr;

void presence_start_ble_scan(PresenceBleScanRequest request) {
    printf("C presence_provider: start_ble_scan with priority %d\n", request.priority);
    provider_ptr->start_ble_scan(request);
}

void presence_register_provider(void* provider) {
    provider_ptr = (PresenceProvider*) provider;
}

void presence_register_rust_callback(int (*cb)(int)) {
    cb(101);
}