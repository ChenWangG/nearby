typedef struct PresenceBleScanRequestTag {
   int priority;
} PresenceBleScanRequest;

typedef struct PresenceProviderTag {
    void (*start_ble_scan)(PresenceBleScanRequest request);
} PresenceProvider;


void presence_register_provider(void* provider);
void presence_start_ble_scan(PresenceBleScanRequest request);