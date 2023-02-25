clippy:
	cargo clippy --all

test:
	cargo test

.PHONY: build install

gui:
	cargo run -p chanki-gui

build:
	cargo build -p chanki-cli --release


install: build
	sudo ln -sf $(realpath target/release/chanki-cli) /usr/bin/chanki-bin
	sudo ln -sf $(realpath scripts/chanki-cli) /usr/bin/chanki
