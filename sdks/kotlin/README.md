# Payment Plan Kotlin SDK

> **⚠️ Important Notice**: This repository and the code in it is a carbon copy of the code from [Parcelado Lara Payment Plan](https://github.com/ParceladoLara/payment-plan). For building from source, configuration setup, and contributing, please refer to the main repository.

This SDK provides a friendly Kotlin interface for the payment plan calculation system.

## 📦 Installation

Add the JitPack repository and dependency to your project:

### Maven

```xml
<repositories>
    <repository>
        <id>jitpack.io</id>
        <url>https://jitpack.io</url>
    </repository>
</repositories>

<dependency>
    <groupId>com.github.parceladolara</groupId>
    <artifactId>payment-plan-kotlin-sdk</artifactId>
    <version>v1.0.0</version>
</dependency>
```

### Gradle (Kotlin DSL)

```kotlin
repositories {
    maven { url = uri("https://jitpack.io") }
}

dependencies {
    implementation("com.github.parceladolara:payment-plan-kotlin-sdk:v1.0.0")
}
```

### Gradle (Groovy)

```groovy
repositories {
    maven { url 'https://jitpack.io' }
}

dependencies {
    implementation 'com.github.parceladolara:payment-plan-kotlin-sdk:v1.0.0'
}
```

> **📋 Note**: Always use the latest version available on [JitPack](https://jitpack.io/#parceladolara/payment-plan-kotlin-sdk). Versions follow the `v1.x.x` pattern.

## 🚀 Quick Start

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
        println("Installment ${installment.installment}: $${installment.installmentAmount}")
        println("  Due Date: ${installment.dueDate}")
        println("  Total: $${installment.totalAmount}")
    }
}
```

## 🧪 Local Development

To develop and test the SDK locally:

```bash
# 1. Clone the repository
git clone https://github.com/ParceladoLara/payment-plan-kotlin-sdk.git
cd payment-plan-kotlin-sdk

# 2. Check status
make status

# 3. Build
make build

# 4. Run example
make example

# 5. Run tests
make test
```

## Local Installation (Alternative)

### Using Local JAR

After building, you can use the generated JAR in `build/libs/`:

```kotlin
dependencies {
    implementation(files("path/to/payment-plan-kotlin-sdk-1.0.0.jar"))
    implementation("net.java.dev.jna:jna:5.13.0")
}
```

### Maven Local Publishing

To publish to local Maven repository:

```bash
./gradlew publishToMavenLocal
```

Then add the dependency:

```kotlin
dependencies {
    implementation("com.parceladolara:payment-plan-kotlin-sdk:1.0.0")
}
```

## Usage

### Import

```kotlin
import com.parceladolara.paymentplan.PaymentPlan
import com.parceladolara.paymentplan.Params
import com.parceladolara.paymentplan.DownPaymentParams
```

### Calculating a Payment Plan

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
    println("Installment amount: ${response.installmentAmount}")
    println("Total: ${response.totalAmount}")
    // ... other fields
}
```

### Calculating a Down Payment Plan

```kotlin
val baseParams = Params(/* ... base parameters ... */)

val downPaymentParams = DownPaymentParams(
    params = baseParams,
    requestedAmount = 200.0,
    minInstallmentAmount = 25.0,
    firstPaymentDate = Instant.now().plus(15, ChronoUnit.DAYS),
    installments = 6u
)

val downPaymentPlan = PaymentPlan.calculateDownPaymentPlan(downPaymentParams)
```

### Calculating Next Disbursement Date

```kotlin
val nextDate = PaymentPlan.nextDisbursementDate(Instant.now())
println("Next disbursement date: $nextDate")
```

### Calculating Disbursement Date Range

```kotlin
val (startDate, endDate) = PaymentPlan.disbursementDateRange(Instant.now(), 5u)
println("Period: $startDate to $endDate")
```

### Getting Non-Business Days in a Period

```kotlin
val startDate = Instant.now()
val endDate = startDate.plus(30, ChronoUnit.DAYS)
val nonBusinessDays = PaymentPlan.getNonBusinessDaysBetween(startDate, endDate)

println("Non-business days: $nonBusinessDays")
```

## 🛠️ Available Commands

```bash
make help              # Show help
make status            # Check SDK status
make build             # Build the project
make test              # Run tests
make example           # Run example
make clean             # Clean build files
make publish           # Publish to local Maven repository
make all               # Build and test everything
```

## 📚 API

### Main Methods

#### `calculatePaymentPlan(params: Params): List<Response>`

Calculates a payment plan based on provided parameters.

#### `calculateDownPaymentPlan(params: DownPaymentParams): List<DownPaymentResponse>`

Calculates a payment plan with down payment.

#### `nextDisbursementDate(baseDate: Instant): Instant`

Calculates the next disbursement date (business days only).

#### `disbursementDateRange(baseDate: Instant, days: UInt): Pair<Instant, Instant>`

Calculates a disbursement date range.

#### `getNonBusinessDaysBetween(startDate: Instant, endDate: Instant): List<Instant>`

Returns non-business days between the provided dates.

## 🔧 Requirements

- **Java JDK 17+**
- **Kotlin JVM 1.9.0+**
- **JNA 5.13.0+** (included automatically)

## ⚡ Compatibility

This SDK is compatible with:

- **JVM**: Java 17+, Kotlin 1.9.0+
- **Android**: API Level 21+ (Android 5.0)
- **Systems**: Linux, macOS, Windows

Native libraries are automatically included and loaded by UniFFI.

## 🌍 For Development

> **For contributions, building from source, and complete configuration, consult the main repository:**  
> **[https://github.com/ParceladoLara/payment-plan](https://github.com/ParceladoLara/payment-plan)**

This repository contains only the pre-compiled and ready-to-use Kotlin SDK.

## 📄 License

This project is licensed under the same terms as the main Payment Plan project.

## 🤝 Support

For technical support, issues, and contributions, use the main repository:
[https://github.com/ParceladoLara/payment-plan](https://github.com/ParceladoLara/payment-plan)
