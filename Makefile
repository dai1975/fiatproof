all: build

fetch:
	cargo fetch

build:
	cargo build #--verbose

build1:
	RUST_BACKTRASE=1 cargo build #--verbose

doc: target/doc
	cargo doc --no-deps --verbose

test:
	cargo test -- --nocapture

test1:
	RUST_BACKTRACE=1 cargo test -- --nocapture

release: target/doc
	rm -rf docs
	cp -r target/doc docs/
	cp docs.html docs/index.html

clean:

allclean:
	rm -rf target
	find src -name "*~" | xargs rm

.PHONY: all build clean test
