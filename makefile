run: build-frontend run-server

build-all: build-frontend build-server

check:
	cargo check

build-frontend:
	cargo build -p triox-frontend --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/triox_frontend.wasm --target web --out-dir data/static/WASM --out-name frontend --no-typescript

build-server:
	cargo build -p triox-server

run-server:
	cargo run -p triox-server

clean:
	cargo clean
	rm data/static/WASM/*
