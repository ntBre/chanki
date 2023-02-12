clippy:
	cargo clippy --tests

test:
	cargo test

.PHONY: build install

build:
	cargo build -p chanki-cli --release

install: build
	sudo ln -sf $(realpath target/release/chanki-cli) /usr/bin/chanki-bin
	sudo ln -sf $(realpath scripts/chanki-cli) /usr/bin/chanki
