typedef struct PresenceBleScanRequestTag {
   int priority;
} PresenceBleScanRequest;

typedef struct PresencePlatformTag {
    void (*start_ble_scan)(PresenceBleScanRequest request);
} PresencePlatform;


void presence_platform_init(void* platform, void* provider);
void presence_register_provider(void* provider);
void presence_start_ble_scan(PresenceBleScanRequest request);
void presence_register_rust_callback(int (*cb)(int));