# Payment Plan Kotlin SDK

> **⚠️ Important Notice**: This repository and the code in it is a carbon copy of the code from [Parcelado Lara Payment Plan](https://github.com/ParceladoLara/payment-plan). For building from source, configuration setup, and contributing, please refer to the main repository.

Este SDK oferece uma interface amigável em Kotlin para o sistema de cálculo de planos de pagamento.

## � Instalação

### Maven

```xml
<dependency>
    <groupId>com.parceladolara</groupId>
    <artifactId>payment-plan-kotlin-sdk</artifactId>
    <version>1.0.0</version>
</dependency>
```

### Gradle (Kotlin DSL)

```kotlin
dependencies {
    implementation("com.parceladolara:payment-plan-kotlin-sdk:1.0.0")
}
```

### Gradle (Groovy)

```groovy
dependencies {
    implementation 'com.parceladolara:payment-plan-kotlin-sdk:1.0.0'
}
```

## 🚀 Uso Rápido

```kotlin
import com.parceladolara.paymentplan.PaymentPlan
import com.parceladolara.paymentplan.Params
import java.time.ZonedDateTime
import java.time.ZoneId

fun main() {
    val params = Params(
        requestedAmount = 1000.0,
        firstPaymentDate = ZonedDateTime.of(2025, 6, 3, 10, 0, 0, 0, ZoneId.of("UTC")).toInstant(),
        disbursementDate = ZonedDateTime.of(2025, 5, 3, 10, 0, 0, 0, ZoneId.of("UTC")).toInstant(),
        installments = 3u,
        debitServicePercentage = 350u,
        mdr = 0.035,
        tacPercentage = 0.01,
        iofOverall = 0.0038,
        iofPercentage = 0.0,
        interestRate = 0.02,
        minInstallmentAmount = 50.0,
        maxTotalAmount = 2000.0,
        disbursementOnlyOnBusinessDays = false
    )

    val result = PaymentPlan.calculatePaymentPlan(params)

    result.forEach { installment ->
        println("Parcela ${installment.installment}: R$ ${installment.installmentAmount}")
        println("  Vencimento: ${installment.dueDate}")
        println("  Total: R$ ${installment.totalAmount}")
    }
}
```

## 🧪 Desenvolvimento Local

Para desenvolver e testar o SDK localmente:

```bash
# 1. Clone o repositório
git clone https://github.com/ParceladoLara/payment-plan-kotlin-sdk.git
cd payment-plan-kotlin-sdk

# 2. Verificar status
make status

# 3. Compilar
make build

# 4. Executar exemplo
make example

# 5. Executar testes
make test
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

## 🛠️ Comandos Disponíveis

```bash
make help              # Mostrar ajuda
make status            # Verificar status do SDK
make build             # Compilar o projeto
make test              # Executar testes
make example           # Executar exemplo
make clean             # Limpar arquivos de build
make publish           # Publicar no repositório Maven local
make all               # Compilar e testar tudo
```

## 📚 API

### Métodos Principais

#### `calculatePaymentPlan(params: Params): List<Response>`

Calcula um plano de pagamento baseado nos parâmetros fornecidos.

#### `calculateDownPaymentPlan(params: DownPaymentParams): List<DownPaymentResponse>`

Calcula um plano de pagamento com entrada.

#### `nextDisbursementDate(baseDate: Instant): Instant`

Calcula a próxima data de desembolso (apenas dias úteis).

#### `disbursementDateRange(baseDate: Instant, days: UInt): Pair<Instant, Instant>`

Calcula um intervalo de datas de desembolso.

#### `getNonBusinessDaysBetween(startDate: Instant, endDate: Instant): List<Instant>`

Retorna dias não úteis entre as datas fornecidas.

## 🔧 Requisitos

- **Java JDK 17+**
- **Kotlin JVM 1.9.0+**
- **JNA 5.13.0+** (incluído automaticamente)

## � Para Desenvolvimento

> **Para contribuições, build from source, e configuração completa, consulte o repositório principal:**  
> **[https://github.com/ParceladoLara/payment-plan](https://github.com/ParceladoLara/payment-plan)**

Este repositório contém apenas o SDK Kotlin pré-compilado e pronto para uso.

## � Licença

Este projeto está licenciado sob os mesmos termos do projeto principal Payment Plan.

## 🤝 Suporte

Para suporte técnico, issues, e contribuições, utilize o repositório principal:
[https://github.com/ParceladoLara/payment-plan](https://github.com/ParceladoLara/payment-plan)
