all: debug

# Build a release candidate
release:
	cargo build --release

# Build a debug candidate
debug:
	cargo build

test:
	cargo test

clean:
	cargo clean
