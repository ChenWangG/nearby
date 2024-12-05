#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct BleScanner BleScanner;

typedef struct BleScanner BleScanner;

typedef struct PresenceRust PresenceRust;

typedef struct ScanResult ScanResult;

typedef void (*SwiftStartBleScan)(void*);

void ble_scanner_on_result(struct BleScanner *ble_scanner, struct ScanResult *scan_result);

struct PresenceRust *presence_new(void *ios_presence, SwiftStartBleScan swift_start_ble_scan);

struct ScanResult *scan_result_new(const uint8_t *data, uint32_t len);
