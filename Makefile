.PHONY: build format lint test
.SILENT:

build:
	echo "Building..."
	cargo build

release:
	echo "Building for release..."
	cargo build --release

format:
	echo "Formatting..."
	cargo fmt --all

lint:
	echo "Linting..."
	cargo clippy --all-targets --all-features -- -D warnings

fix:
	echo "Fixing..."
	cargo fix

test:
	echo "Testing..."
	cargo test

example:
	echo "Running example..."
	cargo run -- generate local -t assets/example_project -d target/new_project
