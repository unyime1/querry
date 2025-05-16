test:
	cargo test --verbose -- --test-threads=1

format:
	cargo fmt --all -- --check

lint:
	cargo clippy --verbose -- -D warnings
