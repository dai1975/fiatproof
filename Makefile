all: build

build:
	cargo build #--verbose

test:
	cargo test

.PHONY: all build clean test
