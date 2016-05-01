# $PROFILE can be set to 'release' on the command line to link against that.
PROFILE=debug

check: target/$(PROFILE)/ctests
	$<

SRCS = src/ctests.c src/version_test.c

target/$(PROFILE)/ctests: ../target/$(PROFILE)/liblibrdiff_capi.a $(SRCS)
	mkdir -p target/$(PROFILE)
	$(CC) -o $@ $(SRCS) \
		-L ../target/$(PROFILE)/ -l librdiff_capi \
		-I ../include

clean:
	ls target/*/ctests >/dev/null && rm target/*/ctests
