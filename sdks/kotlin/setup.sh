#!/bin/bash

# Script para configurar e construir o SDK Kotlin do Payment Plan
set -e

echo "=== Payment Plan Kotlin SDK Setup ==="

# Função para comparar versões
version_compare() {
    local version1=$1
    local version2=$2
    
    # Converter versões para formato comparável
    local v1=$(echo "$version1" | sed 's/[^0-9.].*//')
    local v2=$(echo "$version2" | sed 's/[^0-9.].*//')
    
    if [ "$(printf '%s\n' "$v1" "$v2" | sort -V | head -n1)" = "$v2" ]; then
        return 0  # version1 >= version2
    else
        return 1  # version1 < version2
    fi
}

# Verificar dependências essenciais
echo "0. Verificando dependências do sistema..."

# Verificar se o Rust está instalado
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust não encontrado!"
    echo "Por favor, instale o Rust primeiro:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "  source ~/.cargo/env"
    exit 1
fi

# Verificar versão do Rust
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
if ! version_compare "$RUST_VERSION" "1.70.0"; then
    echo "❌ Rust versão $RUST_VERSION é muito antiga (mínimo: 1.70.0)"
    echo "Execute: rustup update stable"
    exit 1
fi
echo "✅ Rust $RUST_VERSION - OK"

# Verificar se o Java está instalado
if ! command -v java &> /dev/null; then
    echo "❌ Java não encontrado!"
    
    # Detectar sistema operacional e sugerir instalação
    if [ -f /etc/debian_version ]; then
        echo "Para instalar o Java no Debian/Ubuntu:"
        echo "  sudo apt update && sudo apt install -y openjdk-11-jdk"
    elif [ -f /etc/arch-release ]; then
        echo "Para instalar o Java no Arch Linux:"
        echo "  sudo pacman -S jdk11-openjdk"
    else
        echo "Por favor, instale o Java JDK 11 ou superior"
    fi
    
    echo ""
    echo "💡 Você gostaria que eu tente instalar automaticamente? (y/n)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        if [ -f /etc/debian_version ]; then
            echo "Instalando OpenJDK 11..."
            sudo apt update && sudo apt install -y openjdk-11-jdk
        elif [ -f /etc/arch-release ]; then
            echo "Instalando OpenJDK 11..."
            sudo pacman -S --noconfirm jdk11-openjdk
        else
            echo "❌ Instalação automática não suportada para este sistema"
            exit 1
        fi
    else
        exit 1
    fi
fi

# Verificar versão do Java
JAVA_VERSION=$(java -version 2>&1 | head -n1 | cut -d'"' -f2 | cut -d'.' -f1-2)
if [[ "$JAVA_VERSION" == "1.8" ]]; then
    JAVA_MAJOR=8
else
    JAVA_MAJOR=$(echo "$JAVA_VERSION" | cut -d'.' -f1)
fi

if [ "$JAVA_MAJOR" -lt 11 ]; then
    echo "❌ Java versão $JAVA_VERSION é muito antiga (mínimo: Java 11)"
    echo "Por favor, instale Java 11 ou superior"
    exit 1
fi
echo "✅ Java $JAVA_VERSION - OK"

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
