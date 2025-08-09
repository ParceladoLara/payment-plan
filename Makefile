install-deps:
	@if [ -f /etc/debian_version ]; then \
		echo "Detectado sistema Debian/Ubuntu - executando setup/debian.sh"; \
		sudo bash setup/debian.sh; \
	elif [ -f /etc/arch-release ]; then \
		echo "Detectado sistema Arch Linux - executando setup/arch.sh"; \
		sudo bash setup/arch.sh; \
	else \
		echo "Sistema não suportado automaticamente. Execute manualmente:"; \
		echo "  - Para Debian/Ubuntu: sudo bash setup/debian.sh"; \
		echo "  - Para Arch Linux: sudo bash setup/arch.sh"; \
		exit 1; \
	fi

test:
	make clean
	make build-go-sdk
	make build-python-sdk
	make build-kotlin-sdk
	make build-node-sdk
	make build-wasm-sdk
	make build-php-sdk
	cargo test
	cd cli && make test
	cd generators/wasm && npm i && npm run test
	cd cli && make test
	cd sdks/go && go test ./...
	cd sdks/kotlin && ./gradlew clean test -PrunTests
	cd sdks/node && npm test
	cd sdks/web/test && npm i && npx playwright install && npm test
	cd sdks/php && composer install && composer test
	cd sdks/python && python3 -m unittest discover -s tests -p "*.py"

clean:
	cargo clean
	rm -rf ./target
	rm -rf ./sdks/go/internal/libs/linux
	rm -rf ./sdks/go/internal/libs/windows
	rm -rf ./sdks/go/internal/libs/darwin
	rm -rf ./sdks/python/payment_plan/_internal
	rm -rf ./sdks/kotlin/src/main/kotlin/com/parceladolara/paymentplan/internal/payment_plan_uniffi.kt
	rm -rf ./sdks/kotlin/build
	rm -rf ./sdks/kotlin/.gradle
	rm -rf ./sdks/kotlin/src/main/resources/native
	rm -rf ./sdks/node/node_modules
	rm -rf ./sdks/node/native
	rm -rf ./sdks/web/pkg
	rm -rf ./sdks/web/node_modules
	rm -rf ./sdks/php/vendor
	rm -rf ./sdks/php/src/Internal/native
	go clean -testcache -cache -modcache
	mkdir -p ./sdks/php/src/Internal/native
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
	uniffi-bindgen-go --library ./target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll --out-dir ./sdks/go/internal
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
	cargo run --bin uniffi-bindgen generate --library target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll --language python --out-dir sdks/python/payment_plan/_internal
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll sdks/python/payment_plan/_internal/payment_plan_uniffi.dll

build-python-sdk-linux:
	cargo build --package payment_plan_uniffi --profile release-unstripped
	cargo run --bin uniffi-bindgen generate --library target/release-unstripped/libpayment_plan_uniffi.so --language python --out-dir sdks/python/payment_plan/_internal
	cp target/release-unstripped/libpayment_plan_uniffi.so sdks/python/payment_plan/_internal/libpayment_plan_uniffi.so

build-kotlin-sdk:
	cargo build --package payment_plan_uniffi --profile release-unstripped
	cargo build --package payment_plan_uniffi --profile release-unstripped --target x86_64-pc-windows-gnu
	cargo run --bin uniffi-bindgen generate --library target/release-unstripped/libpayment_plan_uniffi.so --language kotlin  --config ./uniffi.toml --out-dir sdks/kotlin/src/main/kotlin
	sed -i 's/\bpublic\b/internal/g; s/\bdata class\b/internal data class/g; s/\bfun `/internal fun `/g' ./sdks/kotlin/src/main/kotlin/com/parceladolara/paymentplan/internal/payment_plan_uniffi.kt
	mkdir -p sdks/kotlin/src/main/resources/native/linux
	mkdir -p sdks/kotlin/src/main/resources/native/windows
	cp target/release-unstripped/libpayment_plan_uniffi.so sdks/kotlin/src/main/resources/native/linux/libpayment_plan_uniffi.so
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll sdks/kotlin/src/main/resources/native/windows/payment_plan_uniffi.dll

build-kotlin-sdk-linux:
	cargo build --package payment_plan_uniffi --profile release-unstripped
	cargo run --bin uniffi-bindgen generate --library target/release-unstripped/libpayment_plan_uniffi.so --language kotlin  --config ./uniffi.toml --out-dir sdks/kotlin/src/main/kotlin
	sed -i 's/\bpublic\b/internal/g; s/\bdata class\b/internal data class/g; s/\bfun `/internal fun `/g' ./sdks/kotlin/src/main/kotlin/com/parceladolara/paymentplan/internal/payment_plan_uniffi.kt
	mkdir -p sdks/kotlin/src/main/resources/native/linux
	cp target/release-unstripped/libpayment_plan_uniffi.so sdks/kotlin/src/main/resources/native/linux/libpayment_plan_uniffi.so

build-kotlin-sdk-windows:
	cargo build --package payment_plan_uniffi --profile release-unstripped --target x86_64-pc-windows-gnu
	cargo run --bin uniffi-bindgen generate --library target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll --language kotlin --out-dir sdks/kotlin/src/main/kotlin
	mkdir -p sdks/kotlin/src/main/resources/native/windows
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_uniffi.dll sdks/kotlin/src/main/resources/native/windows/payment_plan_uniffi.dll

build-node-sdk:
	cd generators/node && npm i
	cd generators/node && npm run build:iterative
	mkdir -p sdks/node/native
	cp ./generators/node/native/index.node sdks/node/native/index.node
	cd sdks/node && npm i
	cd sdks/node && npm run build

build-wasm-sdk:
	cd generators/wasm && npm i
	cd generators/wasm && npm run build:web
	cp -r ./generators/wasm/pkg/. ./sdks/web/pkg
	rm -rf ./sdks/web/pkg/.gitignore

build-php-sdk-linux:
	cargo run --package payment_plan_c_bind --bin generate-headers --features headers --release
	cargo build --package payment_plan_c_bind --profile release-unstripped
	cp target/release-unstripped/libpayment_plan_c_bind.so sdks/php/src/Internal/native/libpayment_plan.so
	mv ./payment_plan.h ./sdks/php/src/Internal/native/payment_plan.h
	sed -i '/^#ifdef __cplusplus$$/,/^#endif$$/d; /^#ifdef __cplusplus$$/,/^} \/\* extern \\"C\\" \*\/$$/d' ./sdks/php/src/Internal/native/payment_plan.h

build-php-sdk-windows:
	cargo run --package payment_plan_c_bind --bin generate-headers --features headers --release
	cargo build --package payment_plan_c_bind --profile release-unstripped --target x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_c_bind.dll sdks/php/src/Internal/native/libpayment_plan.dll
	mv ./payment_plan.h ./sdks/php/src/Internal/native/payment_plan.h
	sed -i '/^#ifdef __cplusplus$$/,/^#endif$$/d; /^#ifdef __cplusplus$$/,/^} \/\* extern \\"C\\" \*\/$$/d' ./sdks/php/src/Internal/native/payment_plan.h

build-php-sdk:
	cargo run --package payment_plan_c_bind --bin generate-headers --features headers --release
	cargo build --package payment_plan_c_bind --profile release-unstripped
	cargo build --package payment_plan_c_bind --profile release-unstripped --target x86_64-pc-windows-gnu
	cp target/release-unstripped/libpayment_plan_c_bind.so sdks/php/src/Internal/native/libpayment_plan.so
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_c_bind.dll sdks/php/src/Internal/native/libpayment_plan.dll
	mv ./payment_plan.h ./sdks/php/src/Internal/native/payment_plan.h
	sed -i '/^#ifdef __cplusplus$$/,/^#endif$$/d; /^#ifdef __cplusplus$$/,/^} \/\* extern \\"C\\" \*\/$$/d' ./sdks/php/src/Internal/native/payment_plan.h

build-c-bind:
	mkdir -p lara-c-bind
	cargo run --package payment_plan_c_bind --bin generate-headers --features headers --release
	cargo build --package payment_plan_c_bind --profile release-unstripped
	cargo build --package payment_plan_c_bind --profile release-unstripped --target x86_64-pc-windows-gnu
	mv ./payment_plan.h lara-c-bind/payment_plan.h
	cp target/release-unstripped/libpayment_plan_c_bind.so lara-c-bind/libpayment_plan.so
	cp target/x86_64-pc-windows-gnu/release-unstripped/payment_plan_c_bind.dll lara-c-bind/libpayment_plan.dll