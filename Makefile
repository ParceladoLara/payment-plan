test:
	cargo test
	cd bin && make test
	cd node && npm i && npm run test
	cd wasm && npm i && npm run test
	cd bin && make test