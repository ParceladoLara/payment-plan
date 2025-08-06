#!/bin/bash

# Script para configurar e construir o SDK Kotlin do Payment Plan
set -e

echo "=== Payment Plan Kotlin SDK Setup ==="

# Navegar para o diretório raiz do projeto
cd "$(dirname "$0")/../.."

echo "1. Verificando se a biblioteca Rust está compilada..."
if [ ! -f "target/release-unstripped/libpayment_plan_uniffi.so" ]; then
    echo "Biblioteca não encontrada. Compilando..."
    cargo build --release --package payment_plan_uniffi
else
    echo "Biblioteca encontrada!"
fi

echo "2. Gerando bindings UniFFI para Kotlin..."
cargo run --bin uniffi-bindgen generate \
    --library target/release-unstripped/libpayment_plan_uniffi.so \
    --language kotlin \
    --out-dir sdks/kotlin/_internal

echo "3. Verificando se os arquivos foram gerados..."
if [ ! -f "sdks/kotlin/_internal/uniffi/payment_plan_uniffi/payment_plan_uniffi.kt" ]; then
    echo "Erro: Arquivos Kotlin não foram gerados corretamente"
    echo "Certifique-se de que a biblioteca foi compilada primeiro:"
    echo "  cargo build --release --package payment_plan_uniffi"
    exit 1
fi

echo "4. Construindo o SDK Kotlin..."
cd sdks/kotlin

# Usar o Gradle Wrapper que já está configurado
if [ -f "./gradlew" ]; then
    GRADLE_CMD="./gradlew"
else
    echo "Erro: gradlew não encontrado. Execute o comando do diretório kotlin/."
    exit 1
fi

echo "4. Executando build..."
$GRADLE_CMD clean compileKotlin jar

echo "5. Build de compilação concluído!"
echo "Nota: Testes foram pulados pois requerem a biblioteca nativa."
echo "Para executar testes: $GRADLE_CMD test -PrunTests"

echo ""
echo "=== Build concluído com sucesso! ==="
echo ""
echo "Para usar o SDK:"
echo "1. Inclua a biblioteca nativa no classpath"
echo "2. Adicione a dependência JNA"
echo "3. Importe: import com.parceladolara.paymentplan.PaymentPlan"
echo ""
echo "Para executar o exemplo:"
echo "  $GRADLE_CMD run"
echo ""
echo "Para publicar:"
echo "  $GRADLE_CMD publish"
