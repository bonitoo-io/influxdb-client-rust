.PHONY: help
.DEFAULT_GOAL := help

help:
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make \033[36m<target>\033[0m\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-46s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

	
fmt: ## Format code
	cargo fmt
	
check-fmt: ## Check format of code
	cargo fmt -- --check

check-cargo: ## Check the current package
	cargo check

test: ## Run tests
	cargo test

test-ci: ## Run tests with XML output
	cargo install cargo-junit
	cargo junit --name junit.xml

coverage-ci: ## Report CodeCoverage to CodeCov - https://github.com/codecov/example-rust
	sudo apt-get install cmake -y
	./scripts/code-coverage.sh

check: check-fmt test ## Check all