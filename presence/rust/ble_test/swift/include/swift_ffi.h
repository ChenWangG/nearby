#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct BleScanner BleScanner;

typedef struct BleScanner BleScanner;

typedef struct ScanResult ScanResult;

typedef void (*SwiftStartBleScan)(void*);

struct BleScanner *ble_scanner_new(SwiftStartBleScan swift_start_ble_scan);

void ble_scanner_on_result(struct BleScanner *ble_scanner, struct ScanResult *scan_result);

void ble_scanner_start(struct BleScanner *ble_scanner);

void rust_object_test(void);

struct ScanResult *scan_result_new(const uint8_t *data, uint32_t len);
