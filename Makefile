test:
	make clean
	make build-go-sdk
	make build-python-sdk
	cargo test
	cd cli && make test
	cd node && npm i && npm run test
	cd wasm && npm i && npm run test
	cd cli && make test
	cd sdks/go && go test ./...
	cd sdks/python && python3 -m unittest discover -s tests -p "*.py"

clean:
	cargo clean
	rm -rf ./target
	rm -rf ./sdks/go/internal/libs/linux
	rm -rf ./sdks/go/internal/libs/windows
	rm -rf ./sdks/go/internal/libs/darwin
	rm -rf ./sdks/python/payment_plan/_internal
	mkdir -p ./sdks/go/internal/libs/linux
	mkdir -p ./sdks/go/internal/libs/windows
	mkdir -p ./sdks/go/internal/libs/darwin

build-go-sdk:
	cargo build --package payment_plan_uniffi --profile release-unstripped
	cargo build --package payment_plan_uniffi --profile release-unstripped --target x86_64-pc-windows-gnu
	cp target/release-unstripped/libpayment_plan_uniffi.a sdks/go/internal/libs/linux/libpayment_plan_uniffi.a
	cp target/x86_64-pc-windows-gnu/release-unstripped/libpayment_plan_uniffi.a sdks/go/internal/libs/windows/libpayment_plan_uniffi.a
	uniffi-bindgen-go --library ./target/release-unstripped/libpayment_plan_uniffi.so --out-dir ./sdks/go/internal
	sed -i 's|// #include <payment_plan_uniffi.h>|/*\n#cgo windows LDFLAGS: -L./../libs/windows -lpayment_plan_uniffi -lws2_32 -luserenv -lkernel32 -lntdll\n#cgo linux LDFLAGS: -L./../libs/linux -lpayment_plan_uniffi  -lm -ldl\n#cgo darwin LDFLAGS: -L./../libs/darwin -lpayment_plan_uniffi  -lm -ldl\n#include <payment_plan_uniffi.h>\n*/|' sdks/go/internal/payment_plan_uniffi/payment_plan_uniffi.go

build-go-sdk-windows:
	cargo build --package payment_plan_uniffi --profile release-unstripped --target x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release-unstripped/libpayment_plan_uniffi.a sdks/go/internal/libs/windows/libpayment_plan_uniffi.a
	uniffi-bindgen-go --library ./target/x86_64-pc-windows-gnu/release-unstripped/libpayment_plan_uniffi.so --out-dir ./sdks/go/internal
	sed -i 's|// #include <payment_plan_uniffi.h>|/*\n#cgo windows LDFLAGS: -L./../libs/windows -lpayment_plan_uniffi -lws2_32 -luserenv -lkernel32 -lntdll\n#include <payment_plan_uniffi.h>\n*/|' sdks/go/internal/payment_plan_uniffi/payment_plan_uniffi.go

build-go-sdk-linux:
	cargo build --package payment_plan_uniffi --profile release-unstripped
	cp target/release-unstripped/libpayment_plan_uniffi.a sdks/go/internal/libs/linux/libpayment_plan_uniffi.a
	uniffi-bindgen-go --library ./target/release-unstripped/libpayment_plan_uniffi.so --out-dir ./sdks/go/internal
	sed -i 's|// #include <payment_plan_uniffi.h>|/*\n#cgo linux LDFLAGS: -L./../libs/linux -lpayment_plan_uniffi  -lm -ldl\n#include <payment_plan_uniffi.h>\n*/|' sdks/go/internal/payment_plan_uniffi/payment_plan_uniffi.go

build-python-sdk:
	cargo build --package payment_plan_uniffi --profile release-unstripped
	cargo build --package payment_plan_uniffi --profile release-unstripped --target x86_64-pc-windows-gnu
	cargo run --bin uniffi-bindgen generate --library target/release-unstripped/libpayment_plan_uniffi.so --language python --out-dir sdks/python/payment_plan/_internal
	cp target/release-unstripped/libpayment_plan_uniffi.so sdks/python/payment_plan/_internal/libpayment_plan_uniffi.so
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll sdks/python/payment_plan/_internal/payment_plan_uniffi.dll

build-python-sdk-windows:
	cargo build --package payment_plan_uniffi --profile release-unstripped --target x86_64-pc-windows-gnu
	cargo run --bin uniffi-bindgen generate --library target/x86_64-pc-windows-gnu/release-unstripped/libpayment_plan_uniffi.so --language python --out-dir sdks/python/payment_plan/_internal
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll sdks/python/payment_plan/_internal/payment_plan_uniffi.dll

build-python-sdk-linux:
	cargo build --package payment_plan_uniffi --profile release-unstripped
	cargo run --bin uniffi-bindgen generate --library target/release-unstripped/libpayment_plan_uniffi.so --language python --out-dir sdks/python/payment_plan/_internal
	cp target/release-unstripped/libpayment_plan_uniffi.so sdks/python/payment_plan/_internal/libpayment_plan_uniffi.so