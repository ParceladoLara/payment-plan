test:
	cargo test
	cd bin && make test
	cd node && npm i && npm run test
	cd wasm && npm i && npm run test
	cd bin && make test

build-go-sdk:
	cargo clean
	rm -rf ./target
	rm -rf ./sdks/go/libs/linux
	rm -rf ./sdks/go/libs/windows
	rm -rf ./sdks/go/libs/darwin
	mkdir -p ./sdks/go/libs/linux
	mkdir -p ./sdks/go/libs/windows
	mkdir -p ./sdks/go/libs/darwin
	cargo build --package payment_plan_uniffi --release
	cargo build --package payment_plan_uniffi --release --target x86_64-pc-windows-gnu
	cp target/release/libpayment_plan_uniffi.a sdks/go/libs/linux/libpayment_plan_uniffi.a
	cp target/x86_64-pc-windows-gnu/release/libpayment_plan_uniffi.a sdks/go/libs/windows/libpayment_plan_uniffi.a
	uniffi-bindgen-go --library ./target/release/libpayment_plan_uniffi.so --out-dir ./sdks/go
	sed -i 's|// #include <payment_plan_uniffi.h>|/*\n#cgo windows LDFLAGS: -L./../libs/windows -lpayment_plan_uniffi -lws2_32 -luserenv -lkernel32 -lntdll\n#cgo linux LDFLAGS: -L./../libs/linux -lpayment_plan_uniffi  -lm -ldl\n#cgo darwin LDFLAGS: -L./../libs/darwin -lpayment_plan_uniffi  -lm -ldl\n#include <payment_plan_uniffi.h>\n*/|' sdks/go/payment_plan_uniffi/payment_plan_uniffi.go