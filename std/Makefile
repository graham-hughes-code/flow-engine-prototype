build:
	cargo build --release --target wasm32-unknown-unknown
	rm -rf ../flow_engine/std
	mkdir ../flow_engine/std
	find ./target/wasm32-unknown-unknown -maxdepth 2 -name "*.wasm" -type f | xargs cp -vt ../flow_engine/std
