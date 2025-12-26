.PHONY: all release debug clean install

CARGO = cargo
UPX = upx
PREFIX ?= /usr/local

all: release

release:
	$(CARGO) build --release
	$(UPX) --best target/release/ddate

debug:
	$(CARGO) build

clean:
	$(CARGO) clean

install: release
	install -Dm755 target/release/ddate $(PREFIX)/bin/ddate

# Build without UPX compression
release-uncompressed:
	$(CARGO) build --release
