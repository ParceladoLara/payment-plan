test:
	cargo test
	cd node && npm i && npm run test
	cd wasm && npm i && npm run test