.PHONY: build format lint test

build:
	@echo "Building..."
	@cargo build

format:
	@echo "Formatting..."
	@cargo fmt --all

lint:
	@echo "Linting..."
	@cargo clippy --all-targets --all-features -- -D warnings

test:
	@echo "Testing..."
	@cargo test
