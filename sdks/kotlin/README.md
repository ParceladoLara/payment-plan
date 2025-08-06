# Payment Plan Kotlin SDK

Este SDK oferece uma interface amigável em Kotlin para o sistema de cálculo de planos de pagamento.

## 🚀 Setup Rápido (Recomendado)

**Para novos usuários - comando completo:**

```bash
# Do diretório sdks/kotlin/
./setup.sh
```

**OU, se você está no diretório raiz:**

```bash
# Do diretório raiz do projeto (payment-plan/)
make build-kotlin-sdk && cd sdks/kotlin && make build
```

**Depois, para testar:**

```bash
cd sdks/kotlin  # (se não estiver já)
make example  # Executa exemplo
make test     # Executa testes
```

💡 **Diferença importante:**

- `./setup.sh` → Verifica dependências (Java/Rust) + compila tudo
- `make build-kotlin-sdk` → Apenas gera bindings (não verifica dependências)

## 📋 Pré-requisitos

O script `setup.sh` verifica e ajuda a instalar automaticamente:

- **Rust** (1.70.0+) - para compilar a biblioteca nativa
- **Java JDK** (11+) - para compilar o código Kotlin
- **Gradle** - incluído via Gradle Wrapper

**� Dica:** Se você não tem essas dependências, o script irá orientar como instalar ou pode tentar instalar automaticamente (Ubuntu/Debian e Arch Linux).

## 🔧 Fluxo de Dependências

1. Rust compila a biblioteca nativa (`libpayment_plan_uniffi.so`)
2. UniFFI gera os bindings Kotlin automaticamente
3. O SDK Kotlin usa os bindings para chamar a biblioteca nativa

**⚠️ IMPORTANTE:** Quando você baixar o código, os bindings Kotlin e bibliotecas nativas não estarão incluídos (são ignorados pelo Git). O script `setup.sh` ou `make build-kotlin-sdk` irão gerar tudo automaticamente.

## 🛠️ Comandos Úteis

```bash
make help              # Mostrar ajuda
make setup             # Configurar ambiente e gerar bindings (mesmo que ./setup.sh)
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
