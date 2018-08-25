// Tests in C for the Rust rdiff C API.
//
// These are invoked by run_ctests.rs.
//
// All tests return true for success.

#include <string.h>
#include <stdio.h>

#include <rdiff.h>

#include "rdiff_capi_ctests.h"

// Version string is plausible.
bool version_test(void) {
    const char* v = rs_version();
    const char* e = "0.0.0";
    if (strcmp(v, e)) {
        printf("Unexpected version \"%s\", wanted \"%s\"\n", v, e);
        return false;
    }
    return true;
}
