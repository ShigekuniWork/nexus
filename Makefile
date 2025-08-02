.PHONY: check
check:
	cargo check --all-targets --all-features

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: test
test:
	cargo test --all-targets --all-features

.PHONY: machete
machete:
	cargo machete --fix

.PHONY: all-check
all-check: check fmt lint test machete
