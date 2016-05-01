# $PROFILE can be set to 'release' on the command line to link against that.
PROFILE=debug

BINDIR=../target/$(PROFILE)
BIN=$(BINDIR)/ctests

check: $(BIN)
	$<

SRCS = src/ctests.c src/version_test.c

$(BIN): ../target/$(PROFILE)/liblibrdiff_capi.a $(SRCS)
	mkdir -p target/$(PROFILE)
	$(CC) -o $@ $(SRCS) \
		-L ../target/$(PROFILE)/ -l librdiff_capi \
		-I ../include

clean:
	-rm $(BIN)
