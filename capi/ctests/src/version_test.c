#include <string.h>
#include <stdio.h>

#include <librdiff.h>

#include "rdiff_capi_ctests.h"

bool version_test(void) {
    const char* v = rs_version();
    const char* e = "0.0.0";
    if (strcmp(v, e)) {
        printf("Unexpected version \"%s\", wanted \"%s\"\n", v, e);
        return false;
    }
    return true;
}
