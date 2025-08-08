import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.9.0"
    `java-library`
    `maven-publish`
}

group = "com.parceladolara"
version = "1.0.0"

repositories {
    mavenCentral()
}

// Include the generated uniffi bindings in the source set
kotlin {
    sourceSets {
        main {
            kotlin.srcDirs("src/main/kotlin", "_internal")
        }
    }
}

dependencies {
    implementation("net.java.dev.jna:jna:5.13.0")
    
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.9.0")
}

tasks.test {
    useJUnitPlatform()
    // Skip tests by default since they require the native library
    onlyIf { project.hasProperty("runTests") }
    
    // Set the library path to find the native library
    systemProperty("jna.library.path", "../../target/release-unstripped")
    
    doFirst {
        val nativeLibPath = file("../../target/release-unstripped/libpayment_plan_uniffi.so")
        if (!nativeLibPath.exists()) {
            throw GradleException(
                "Native library not found at: ${nativeLibPath.absolutePath}\n" +
                "Please compile the Rust library first:\n" +
                "  cargo build --release --package payment_plan_uniffi"
            )
        }
        println("Running tests. Using native library at: ${nativeLibPath.absolutePath}")
    }
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "17"
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

kotlin {
    jvmToolchain(17)
}

// Task to ensure the uniffi bindings are generated before compilation
tasks.register("generateUniffiBindings") {
    description = "Generate uniffi bindings for Kotlin"
    
    doLast {
        println("Make sure to run the uniffi-bindgen command before building:")
        println("cargo run --bin uniffi-bindgen generate --library target/release-unstripped/libpayment_plan_uniffi.so --language kotlin --out-dir sdks/kotlin/_internal")
    }
}

tasks.named("compileKotlin") {
    dependsOn("generateUniffiBindings")
}

// Task to run the example
tasks.register<JavaExec>("runExample") {
    group = "application"
    description = "Run the PaymentPlan example"
    classpath = sourceSets.main.get().runtimeClasspath
    mainClass.set("com.parceladolara.paymentplan.examples.PaymentPlanExample")
    
    // Set the library path to find the native library
    systemProperty("jna.library.path", "../../target/release-unstripped")
    
    doFirst {
        val nativeLibPath = file("../../target/release-unstripped/libpayment_plan_uniffi.so")
        if (!nativeLibPath.exists()) {
            throw GradleException(
                "Native library not found at: ${nativeLibPath.absolutePath}\n" +
                "Please compile the Rust library first:\n" +
                "  cargo build --release --package payment_plan_uniffi"
            )
        }
        println("Using native library at: ${nativeLibPath.absolutePath}")
    }
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
            
            artifactId = "payment-plan-kotlin-sdk"
            
            pom {
                name.set("Payment Plan Kotlin SDK")
                description.set("Kotlin SDK for payment plan calculations")
                url.set("https://github.com/ParceladoLara/payment-plan")
                
                developers {
                    developer {
                        id.set("parceladolara")
                        name.set("Parcelado Lara")
                        email.set("it-group@lara.app.br")
                    }
                }
            }
        }
    }
}
