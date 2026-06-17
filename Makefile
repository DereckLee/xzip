.PHONY: help build test lint format check install

help:
	@echo "Available targets:"
	@echo "  make build   - Build debug binary"
	@echo "  make test    - Run all tests"
	@echo "  make lint    - Run clippy with warnings denied"
	@echo "  make format  - Format code with rustfmt"
	@echo "  make check   - format + lint + test"
	@echo "  make install - Install from local path with lockfile"

build:
	cargo build

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings

format:
	cargo fmt

check: format lint test

install:
	cargo install --path . --locked
