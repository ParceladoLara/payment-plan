# Payment Plan Kotlin SDK

Este SDK oferece uma interface amigável em Kotlin para o sistema de cálculo de planos de pagamento.

## 🚀 Setup Rápido (Recomendado)

**Para novos usuários - usando setup global:**

```bash
# Do diretório raiz do projeto (payment-plan/)
# 1. Instalar todas as dependências (incluindo Java)
./setup/debian.sh    # Para Debian/Ubuntu
# OU
./setup/arch.sh      # Para Arch Linux

# 2. Compilar o SDK Kotlin
make build-kotlin-sdk
```

**Para usuários experientes (dependências já instaladas):**

```bash
# Do diretório raiz do projeto (payment-plan/)
make build-kotlin-sdk && cd sdks/kotlin && make build

# Depois testar
cd sdks/kotlin
make example  # Executa exemplo
make test     # Executa testes
```

## 📋 Pré-requisitos

Os scripts de setup global (`./setup/debian.sh` ou `./setup/arch.sh`) instalam automaticamente:

- **Rust** (1.81.0+) - para compilar a biblioteca nativa
- **Java JDK** (11+) - para compilar o código Kotlin
- **Gradle** - incluído via Gradle Wrapper no projeto

**💡 Dica:** Para outras distribuições Linux ou sistemas operacionais, consulte a seção "Instalação Manual" no README principal do projeto.

## 🔧 Fluxo de Dependências

1. Rust compila a biblioteca nativa (`libpayment_plan_uniffi.so`)
2. UniFFI gera os bindings Kotlin automaticamente
3. O SDK Kotlin usa os bindings para chamar a biblioteca nativa

**⚠️ IMPORTANTE:** Os bindings Kotlin e bibliotecas nativas agora estão incluídos no repositório Git. O comando `make build-kotlin-sdk` irá verificar se já existem antes de regenerá-los.

## 🛠️ Comandos Úteis

**Do diretório `sdks/kotlin/`:**

```bash
make help              # Mostrar ajuda
make build             # Compilar o projeto
make test              # Executar testes
make example           # Executar exemplo
make clean             # Limpar arquivos de build
make publish           # Publicar o pacote
make all               # Configurar, compilar e testar tudo
```

## 📖 Setup Manual (Avançado)

Se você preferir fazer tudo manualmente:

### 1. Compilar dependências Rust

```bash
# Do diretório raiz (payment-plan/)
cargo build --release --package payment_plan_uniffi
```

### 2. Gerar bindings UniFFI

```bash
cargo run --bin uniffi-bindgen generate \
  --library target/release-unstripped/libpayment_plan_uniffi.so \
  --language kotlin \
  --out-dir sdks/kotlin/_internal
```

### 3. Compilar o SDK Kotlin

```bash
cd sdks/kotlin
./gradlew build
```

### 4. Executar testes

```bash
./gradlew test -PrunTests
```

## Instalação

### Usando o JAR local

Após compilar, você pode usar o JAR gerado em `build/libs/`:

```kotlin
dependencies {
    implementation(files("path/to/payment-plan-kotlin-sdk-1.0.0.jar"))
    implementation("net.java.dev.jna:jna:5.13.0")
}
```

### Publicação Maven

Para publicar no repositório Maven local:

```bash
./gradlew publishToMavenLocal
```

Então adicione a dependência:

```kotlin
dependencies {
    implementation("com.parceladolara:payment-plan-kotlin-sdk:1.0.0")
}
```

## Uso

### Importação

```kotlin
import com.parceladolara.paymentplan.PaymentPlan
import com.parceladolara.paymentplan.Params
import com.parceladolara.paymentplan.DownPaymentParams
```

### Calculando um Plano de Pagamento

```kotlin
import java.time.Instant
import java.time.temporal.ChronoUnit

val params = Params(
    requestedAmount = 1000.0,
    firstPaymentDate = Instant.now().plus(30, ChronoUnit.DAYS),
    disbursementDate = Instant.now().plus(1, ChronoUnit.DAYS),
    installments = 12u,
    debitServicePercentage = 350u,
    mdr = 0.035,
    tacPercentage = 0.01,
    iofOverall = 0.0038,
    iofPercentage = 0.0,
    interestRate = 0.02,
    minInstallmentAmount = 50.0,
    maxTotalAmount = 2000.0,
    disbursementOnlyOnBusinessDays = true
)

val paymentPlan = PaymentPlan.calculatePaymentPlan(params)
for (response in paymentPlan) {
    println("Valor da parcela: ${response.installmentAmount}")
    println("Total: ${response.totalAmount}")
    // ... outros campos
}
```

### Calculando um Plano de Pagamento com Entrada

```kotlin
val baseParams = Params(/* ... parâmetros base ... */)

val downPaymentParams = DownPaymentParams(
    params = baseParams,
    requestedAmount = 200.0,
    minInstallmentAmount = 25.0,
    firstPaymentDate = Instant.now().plus(15, ChronoUnit.DAYS),
    installments = 6u
)

val downPaymentPlan = PaymentPlan.calculateDownPaymentPlan(downPaymentParams)
```

### Calculando a Próxima Data de Desembolso

```kotlin
val nextDate = PaymentPlan.nextDisbursementDate(Instant.now())
println("Próxima data de desembolso: $nextDate")
```

### Calculando um Intervalo de Datas de Desembolso

```kotlin
val (startDate, endDate) = PaymentPlan.disbursementDateRange(Instant.now(), 5u)
println("Período: $startDate até $endDate")
```

### Obtendo Dias Não Úteis em um Período

```kotlin
val startDate = Instant.now()
val endDate = startDate.plus(30, ChronoUnit.DAYS)
val nonBusinessDays = PaymentPlan.getNonBusinessDaysBetween(startDate, endDate)

println("Dias não úteis: $nonBusinessDays")
```

## Métodos Principais

### `calculatePaymentPlan(params: Params): List<Response>`

Calcula um plano de pagamento baseado nos parâmetros fornecidos.

### `calculateDownPaymentPlan(params: DownPaymentParams): List<DownPaymentResponse>`

Calcula um plano de pagamento com entrada baseado nos parâmetros fornecidos.

### `nextDisbursementDate(baseDate: Instant): Instant`

Calcula a próxima data de desembolso baseada na data fornecida. Assume que desembolsos só ocorrem em dias úteis.

### `disbursementDateRange(baseDate: Instant, days: UInt): Pair<Instant, Instant>`

Calcula um intervalo de datas de desembolso baseado na data base e número de dias úteis.

### `getNonBusinessDaysBetween(startDate: Instant, endDate: Instant): List<Instant>`

Retorna uma lista de dias não úteis entre as datas fornecidas (ambas inclusivas).

## Dependências

Este SDK requer:

- Kotlin JVM 1.9.0+
- JNA 5.13.0+ (para comunicação com a biblioteca nativa)

## 🔧 Integração com o Projeto Principal

O SDK Kotlin está **totalmente integrado** ao sistema de build principal do projeto Payment Plan.

### Comandos Disponíveis no Makefile Principal:

```bash
make build-kotlin-sdk         # Build completo (Linux + Windows)
make build-kotlin-sdk-linux   # Build apenas Linux
make build-kotlin-sdk-windows # Build apenas Windows
make test                     # Inclui testes do Kotlin
make clean                    # Limpa arquivos do Kotlin
```

### Características da Integração:

- **Multiplataforma**: Gera bibliotecas para Linux (`.so`) e Windows (`.dll`)
- **Build inteligente**: Detecta se bindings já existem antes de regenerar
- **Bindings automáticos**: Gera automaticamente os bindings UniFFI para Kotlin
- **Testes integrados**: Incluído no comando `make test` principal
- **Limpeza automática**: Incluído no comando `make clean` principal

## ✨ Características do SDK

- **Interface limpa**: Apenas 5 métodos principais bem documentados
- **Type aliases**: Para melhor exposição da API
- **Exemplos completos**: Demonstrando todos os casos de uso
- **Testes unitários**: Para validação da funcionalidade
- **Build automatizado**: Makefile e scripts para facilitar uso
- **Documentação completa**: README com exemplos e instruções

## 📦 Distribuição

O JAR compilado está em `build/libs/payment-plan-kotlin-sdk-1.0.0.jar` e pode ser:

1. **Usado localmente**: Adicionando o JAR como dependência
2. **Publicado no Maven**: Via `./gradlew publish`
3. **Distribuído**: Como biblioteca standalone

## ✅ Status do Projeto

- ✅ SDK criado e compilando
- ✅ Wrapper com 5 métodos principais funcionando
- ✅ Exemplo completo executando (`make example`)
- ✅ Testes unitários passando (`make test`)
- ✅ Documentação completa
- ✅ Build automatizado e integrado
- ✅ JAR gerado e funcional
- ✅ Suporte multiplataforma (Linux/Windows)
- ✅ Bindings UniFFI gerados automaticamente
- ✅ Integração completa com projeto principal

O SDK Kotlin está **pronto para uso em produção**!
