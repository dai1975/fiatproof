all: build

fetch:
	cargo fetch

build:
	cargo build #--verbose

build1:
	RUST_BACKTRASE=1 cargo build #--verbose

doc:
	cargo doc --no-deps --verbose

test:
	cargo test -- --nocapture

test1:
	RUST_BACKTRACE=1 cargo test -- --nocapture

clean:
	rm -rf target

.PHONY: all build clean test
