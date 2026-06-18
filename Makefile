.PHONY: help build test lint format check install man install-man clean

PREFIX ?= /usr/local
DESTDIR ?=
MAN_DIR = $(DESTDIR)$(PREFIX)/share/man/man1

help:
	@echo "Available targets:"
	@echo "  make build       - Build debug binary"
	@echo "  make test        - Run all tests"
	@echo "  make lint        - Run clippy with warnings denied"
	@echo "  make format      - Format code with rustfmt"
	@echo "  make check       - format + lint + test"
	@echo "  make man         - Generate man pages into ./man"
	@echo "  make install     - Install xzip binary with lockfile"
	@echo "  make install-man - Install man pages (run make man first)"
	@echo "  make clean       - Remove build artifacts and generated files"

build:
	cargo build

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings

format:
	cargo fmt

check: format lint test

man:
	cargo run --features gen-man --bin gen-man

install:
	cargo install --path . --locked

install-man: man
	install -d $(MAN_DIR)
	install -m 644 man/xzip*.1 $(MAN_DIR)/

clean:
	cargo clean
	rm -rf man/
	rm -f *.zip
