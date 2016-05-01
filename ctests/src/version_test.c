#include <string.h>
#include <stdio.h>

#include <librdiff.h>

#include "rdiff_capi_ctests.h"

int version_test(void) {
    const char* v = rs_version();
    if (strcmp(v, "3.0.0")) {
        printf("Unexpected version \"%s\"\n", v);
        return 1;
    }
    return 0;
}
