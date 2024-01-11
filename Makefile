.DEFAULT_GOAL := help
.SILENT:

.PHONY: help
help:  ## these help instructions
	sed -rn 's/^([a-zA-Z_-]+):.*?## (.*)$$/"\1" "\2"/p' < $(MAKEFILE_LIST)|xargs printf "make %-20s# %s\n"

.PHONY: build
build: ## build the code
	echo "Building..."
	cargo build

.PHONY: release
release: ## build for release
	echo "Building for release..."
	cargo build --release

.PHONY: run
run: ## run the code
	echo "Running..."
	cargo run --

.PHONY: format
format: ## format the code
	echo "Formatting..."
	cargo fmt --all

.PHONY: lint
lint: ## lint the code
	echo "Linting..."
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: fix
fix: ## fix the code
	echo "Fixing..."
	cargo fix

.PHONY: test
test: ## run the tests
	echo "Testing..."
	cargo test

.PHONY: example
example: ## run the example
	echo "Running example..."
	cargo run -- generate local -t assets/example_project -d target/new_project
