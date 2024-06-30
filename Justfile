set quiet

build: ## build the code
    echo "Building..."
    cargo build

release: ## build for release
    cargo build --release

run: ## run the code
    echo "Running..."
    cargo run --

format: ## format the code
    echo "Formatting..."
    cargo fmt --all

lint: ## lint the code
    echo "Linting..."
    cargo clippy --all-targets --all-features -- -D warnings

fix: ## fix the code
    echo "Fixing..."
    cargo fix

test: ## run the tests
    echo "Testing..."
    cargo test

example: ## run the example
    echo "Running example..."
    rm -rf target/new_project
    cargo run -- template generate local -t assets/example_project -d target/new_project
