# $PROFILE can be set to 'release' on the command line to link against that.
# build.rs and run_ctests.rs should arrange for the right value to be
# passed in.
PROFILE=debug

CFLAGS=-Wall -Werror
BINDIR=../target/$(PROFILE)
BIN=$(BINDIR)/ctests

check: $(BIN)
	$<

SRCS = src/ctests.c src/version_test.c

$(BIN): ../target/$(PROFILE)/liblibrdiff_capi.a $(SRCS)
	$(CC) -o $@ $(SRCS) $(CFLAGS) \
		-L ../target/$(PROFILE)/ -l librdiff_capi \
		-I ../include

clean:
	-rm $(BIN)
