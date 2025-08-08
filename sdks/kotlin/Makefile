.PHONY: help build test clean example publish

help: ## Mostrar esta mensagem de ajuda
	@echo "Comandos disponíveis:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Compilar o projeto completo (com testes)
	./gradlew build

build-fast: ## Compilar apenas o código (sem testes)
	./gradlew compileKotlin jar

test: ## Executar testes (requer biblioteca nativa)
	./gradlew clean test -PrunTests

clean: ## Limpar arquivos de build
	./gradlew clean
	rm -rf _internal/

example: ## Executar exemplo
	./gradlew runExample

publish: ## Publicar no repositório Maven local
	./gradlew publishToMavenLocal

check: ## Verificar se os bindings foram gerados
	@if [ ! -f "_internal/uniffi/payment_plan_uniffi/payment_plan_uniffi.kt" ]; then \
		echo "❌ Bindings UniFFI não encontrados. Execute 'make build-kotlin-sdk' do diretório raiz do projeto."; \
		exit 1; \
	else \
		echo "✅ Bindings UniFFI encontrados."; \
	fi

generate-bindings: ## Gerar apenas os bindings UniFFI
	@echo "Gerando bindings UniFFI..."
	@cd ../.. && cargo run --bin uniffi-bindgen generate \
		--library target/release-unstripped/libpayment_plan_uniffi.so \
		--language kotlin \
		--out-dir sdks/kotlin/_internal

all: build test ## Compilar e testar tudo
