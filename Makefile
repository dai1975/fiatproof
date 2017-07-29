all: build

build:
	cargo build #--verbose

test:
	cargo test -- --nocapture

.PHONY: all build clean test
