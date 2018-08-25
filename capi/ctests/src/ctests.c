#include <stdio.h>

#include "rdiff_capi_ctests.h"

int main(void) {
    bool passed = version_test();
    printf("%s\n", passed ? "ok" : "failed!");
    return !passed;
}
