typedef struct PresenceBleScanRequestTag {
   int priority;
} PresenceBleScanRequest;

typedef struct PresenceProviderTag {
    void (*start_ble_scan)(PresenceBleScanRequest request);
} PresenceProvider;


void presence_register_provider(void* provider);
void presence_start_ble_scan(PresenceBleScanRequest request);
void presence_register_rust_callback(int (*cb)(int));