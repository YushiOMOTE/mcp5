default:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/gf.wasm public

setup:
	rustup target add wasm32-unknown-unknown

