#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct PresenceRust PresenceRust;

typedef struct ScanResult ScanResult;

typedef void (*SwiftStartBleScan)(void*, void*);

struct PresenceRust *presence_new(void *ios_presence, SwiftStartBleScan swift_start_ble_scan);

struct ScanResult *scan_result_new(const uint8_t *data, uint32_t len);
