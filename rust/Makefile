BINARY_NAME=flatted

.PHONY: build test lint check clean

build:
	cargo build --release --features cli
	cp target/release/$(BINARY_NAME) ./$(BINARY_NAME)

test:
	cargo test

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings

check: test lint

clean:
	cargo clean
	rm -f $(BINARY_NAME)
