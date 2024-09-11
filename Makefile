run:
	cargo run --bin trader

run-release:
	cargo run --bin trader --release

run-release-watch:
	cargo watch --bin trader --release

build:
	cargo build --bin trader

build-release:
	cargo build --bin trader --release

clean:
	rm -rf logs

.PHONY: run run-release run-release-watch build build-release clean
