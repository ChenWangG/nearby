#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct BleScanner BleScanner;

typedef struct BleScanner BleScanner;

typedef struct PresenceRust PresenceRust;

typedef struct ScanResult ScanResult;

void ble_scanner_on_result(struct BleScanner *ble_scanner, struct ScanResult *scan_result);

struct PresenceRust *presence_new(void);

struct ScanResult *scan_result_new(const uint8_t *data, uint32_t len);
