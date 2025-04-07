setup:
	cargo build

run:
	cargo run

debug:
	RUST_BACKTRACE=1 cargo run

clean:
	cargo clean 

test:
	cargo test

rebuild:
	cargo clean && cargo build
