.PHONY: dev

dev:
	cargo watch -qcx run

release:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features
