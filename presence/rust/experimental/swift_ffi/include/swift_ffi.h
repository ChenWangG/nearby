#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RustObject RustObject;

typedef void (*CallSwift)(int32_t);

struct RustObject *rust_object_new(CallSwift call_swift);

void rust_object_call_swift(struct RustObject *rust_object);

int32_t test_swift_callback(CallSwift call_swift);
