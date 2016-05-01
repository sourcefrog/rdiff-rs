#include <stdio.h>

#include "rdiff_capi_ctests.h"

int main(void) {
    int failed = version_test();
    printf("%s\n", failed ? "failed!" : "ok!");
    return failed;
}
