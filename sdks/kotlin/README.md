# Payment Plan Kotlin SDK

Este SDK oferece uma interface amigável em Kotlin para o sistema de cálculo de planos de pagamento.

## Pré-requisitos

- JDK 11 ou superior
- Gradle 8.4 ou superior
- A biblioteca nativa compilada (`libpayment_plan_uniffi.so`)

## Setup Rápido

1. **Gerar os bindings e configurar:**

   ```bash
   make setup
   ```

2. **Compilar o projeto:**

   ```bash
   make build
   ```

3. **Executar testes:**

   ```bash
   make test
   ```

4. **Executar exemplo:**
   ```bash
   make example
   ```

## Setup Manual

### 1. Gerar os bindings UniFFI

Primeiro, certifique-se de que a biblioteca Rust foi compilada:

```bash
cd ../..
cargo build --release --package payment_plan_uniffi
```

Em seguida, gere os bindings Kotlin:

```bash
cargo run --bin uniffi-bindgen generate \
  --library target/release-unstripped/libpayment_plan_uniffi.so \
  --language kotlin \
  --out-dir sdks/kotlin/_internal
```

### 2. Compilar o SDK

```bash
cd sdks/kotlin
./gradlew compileKotlin jar
```

**Nota:** Os testes são pulados por padrão pois requerem a biblioteca nativa. Para executar os testes:

```bash
./gradlew test -PrunTests
```

## Testes

Os testes requerem que a biblioteca nativa esteja disponível. Para executá-los:

1. Certifique-se de que a biblioteca foi compilada:

   ```bash
   cd ../..
   cargo build --release --package payment_plan_uniffi
   ```

2. Execute os testes:
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

## Testes

Execute os testes com:

```bash
./gradlew test
```

## Build

Para compilar o projeto:

```bash
./gradlew build
```

Para publicar:

```bash
./gradlew publish
```

## Comandos Disponíveis

Este projeto inclui um Makefile com comandos úteis:

```bash
make help              # Mostrar ajuda
make setup             # Configurar ambiente e gerar bindings
make build             # Compilar o projeto
make test              # Executar testes
make clean             # Limpar arquivos de build
make example           # Executar exemplo
make publish           # Publicar o pacote
make check             # Verificar se bindings foram gerados
make generate-bindings # Gerar apenas os bindings UniFFI
make all               # Configurar, compilar e testar tudo
```
