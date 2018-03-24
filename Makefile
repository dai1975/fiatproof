all: build

build:
	cargo build #--verbose

test:
	cargo test -- --nocapture

test1:
	RUST_BACKTRACE=1 cargo test -- --nocapture

.PHONY: all build clean test
