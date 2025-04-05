setup:
	cargo build

run:
	cargo run

debug:
	RUST_BACKTRACE=1 cargo run

clean:
	cargo clean

rebuild:
	cargo clean && cargo build
