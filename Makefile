release:
	cargo build --release

build:
	cargo build

peek:
	cargo run --bin peek $(PID)

# NOT working now.
poke:
	cargo run --bin poke $(PID) TKEY VAL
