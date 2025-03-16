.PHONY: build run test test-unit test-integration test-e2e lint clean

build:
	cargo build

run:
	cargo run

test: test-unit test-integration test-e2e

test-unit:
	cargo test --lib

test-integration:
	cp .env.test .env
	cargo test --test integration_tests
	cp .env.backup .env || true

test-e2e:
	cargo test --test e2e

lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings

clean:
	cargo clean