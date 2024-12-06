#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct PresenceRust PresenceRust;

typedef struct ScanResult ScanResult;

typedef void (*SwiftStartBleScan)(void*, void*);

typedef void (*SwiftOnDiscover)(void*, uint32_t);

void ble_scanner_on_result(void *scan_callback, struct ScanResult *scan_result);

struct PresenceRust *presence_new(void *ios_presence, SwiftStartBleScan swift_start_ble_scan);

void presence_set_request(struct PresenceRust *presence_rust_ptr);

void presence_start(struct PresenceRust *presence_rust_ptr, SwiftOnDiscover swift_on_discover);

struct ScanResult *scan_result_new(const uint8_t *data, uint32_t len);
