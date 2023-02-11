clippy:
	cargo clippy --tests

test:
	cargo test

.PHONY: build install

build:
	cargo build --release

install: build
	sudo ln -sf $(realpath target/release/chanki) /usr/bin/chanki-bin
	sudo ln -sf $(realpath scripts/chanki) /usr/bin/chanki
