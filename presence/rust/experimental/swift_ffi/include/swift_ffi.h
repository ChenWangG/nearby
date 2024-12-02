#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*CallSwift)(int32_t);

int32_t test_swift_ffi(void);

int32_t test_swift_callback(CallSwift call_swift);
