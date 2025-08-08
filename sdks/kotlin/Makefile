.PHONY: help build test clean example publish check

help: ## Mostrar esta mensagem de ajuda
	@echo "Comandos disponíveis:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Compilar o projeto completo (com testes)
	./gradlew build

build-fast: ## Compilar apenas o código (sem testes)
	./gradlew compileKotlin jar

test: ## Executar testes
	./gradlew clean test -PrunTests

clean: ## Limpar arquivos de build
	./gradlew clean

example: ## Executar exemplo
	./gradlew runExample

publish: ## Publicar no repositório Maven local
	./gradlew publishToMavenLocal

check: ## Verificar se os bindings foram gerados
	@if [ ! -f "_internal/uniffi/payment_plan_uniffi/payment_plan_uniffi.kt" ]; then \
		echo "❌ Bindings UniFFI não encontrados."; \
		echo "Este repositório é independente. Para gerar bindings, veja:"; \
		echo "https://github.com/ParceladoLara/payment-plan"; \
		exit 1; \
	else \
		echo "✅ Bindings UniFFI encontrados."; \
	fi

status: ## Verificar status do SDK
	@echo "📦 Payment Plan Kotlin SDK - Status"
	@echo "-------------------------------------"
	@./gradlew --version | head -3
	@echo ""
	@if [ -f "_internal/uniffi/payment_plan_uniffi/payment_plan_uniffi.kt" ]; then \
		echo "✅ UniFFI bindings: OK"; \
	else \
		echo "❌ UniFFI bindings: Missing"; \
	fi
	@if [ -d "src/main/resources/native" ] && [ ! -z "$$(ls -A src/main/resources/native 2>/dev/null)" ]; then \
		echo "✅ Native libraries: OK"; \
		echo "   Libraries found: $$(ls src/main/resources/native/ 2>/dev/null | tr '\n' ' ')"; \
	else \
		echo "❌ Native libraries: Missing"; \
	fi
	@echo ""
	@echo "📖 This is a standalone repository. For building from source:"
	@echo "   https://github.com/ParceladoLara/payment-plan"

all: build test ## Compilar e testar tudo
