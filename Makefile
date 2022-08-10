default:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./public/ --target web ./target/wasm32-unknown-unknown/release/gf.wasm

setup:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli
