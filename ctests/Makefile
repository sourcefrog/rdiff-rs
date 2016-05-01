check: target/debug/ctest
	$<

TESTBIN = target/debug/ctest

SRCS = src/ctests.c src/version_test.c

target/debug/ctest: ../target/debug/liblibrdiff_capi.a $(SRCS)
	mkdir -p target/debug
	$(CC) -o $@ $(SRCS) \
		-L ../target/debug/ -l librdiff_capi \
		-I ../include

clean:
	[ -f target/debug/ctest ] && rm target/debug/ctest
