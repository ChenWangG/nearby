#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#if defined(DEFINE_IOS)
typedef struct BleScanner BleScanner;
#endif

typedef struct BleScanner BleScanner;

#if defined(DEFINE_IOS)
typedef void (*SwiftStartBleScan)(void);
#endif

#if defined(DEFINE_IOS)
struct BleScanner *ble_scanner_new(SwiftStartBleScan swift_start_ble_scan);
#endif
