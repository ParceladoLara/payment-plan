package com.parceladolara.paymentplan.examples

import com.parceladolara.paymentplan.PaymentPlan
import com.parceladolara.paymentplan.Params
import com.parceladolara.paymentplan.DownPaymentParams
import java.time.Instant
import java.time.temporal.ChronoUnit

/**
 * Exemplo de uso do Payment Plan Kotlin SDK
 */
object PaymentPlanExample {

    @JvmStatic
    fun main(args: Array<String>) {
        println("=== Payment Plan Kotlin SDK - Exemplos ===\n")

        // Exemplo 1: Cálculo de plano de pagamento simples
        simplePaymentPlanExample()

        // Exemplo 2: Cálculo de plano de pagamento com entrada
        downPaymentPlanExample()

        // Exemplo 3: Calculando datas de desembolso
        disbursementDateExample()

        // Exemplo 4: Obtendo dias não úteis
        nonBusinessDaysExample()
    }

    private fun simplePaymentPlanExample() {
        println("1. Exemplo: Plano de Pagamento Simples")
        println("Calculando plano para R$ 1.000,00 em 12x...")

        val params = Params(
            requestedAmount = 1000.0,
            firstPaymentDate = Instant.now().plus(30, ChronoUnit.DAYS),
            disbursementDate = Instant.now().plus(1, ChronoUnit.DAYS),
            installments = 12u,
            debitServicePercentage = 350u, // 3.5%
            mdr = 0.035,
            tacPercentage = 0.01,
            iofOverall = 0.0038,
            iofPercentage = 0.0,
            interestRate = 0.02,
            minInstallmentAmount = 50.0,
            maxTotalAmount = 2000.0,
            disbursementOnlyOnBusinessDays = true
        )

        try {
            val responses = PaymentPlan.calculatePaymentPlan(params)
            
            println("Planos disponíveis: ${responses.size}")
            responses.forEachIndexed { index, response ->
                println("  Opção ${index + 1}:")
                println("    Parcelas: ${response.installment}")
                println("    Valor da parcela: R$ ${"%.2f".format(response.installmentAmount)}")
                println("    Valor total: R$ ${"%.2f".format(response.totalAmount)}")
                println("    Valor líquido: R$ ${"%.2f".format(response.customerAmount)}")
                if (response.invoices.isNotEmpty()) {
                    println("    Primeira cobrança: ${response.invoices[0].dueDate}")
                }
                println()
            }
        } catch (e: Exception) {
            println("Erro ao calcular plano de pagamento: ${e.message}")
        }
        
        println("=" * 50 + "\n")
    }

    private fun downPaymentPlanExample() {
        println("2. Exemplo: Plano de Pagamento com Entrada")
        println("Calculando plano para R$ 1.000,00 com entrada de R$ 200,00...")

        val baseParams = Params(
            requestedAmount = 1000.0,
            firstPaymentDate = Instant.now().plus(45, ChronoUnit.DAYS),
            disbursementDate = Instant.now().plus(1, ChronoUnit.DAYS),
            installments = 10u,
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

        val downPaymentParams = DownPaymentParams(
            params = baseParams,
            requestedAmount = 200.0,  // Entrada de R$ 200
            minInstallmentAmount = 30.0,
            firstPaymentDate = Instant.now().plus(15, ChronoUnit.DAYS),
            installments = 4u  // Entrada em até 4x
        )

        try {
            val responses = PaymentPlan.calculateDownPaymentPlan(downPaymentParams)
            
            println("Planos com entrada disponíveis: ${responses.size}")
            responses.forEachIndexed { index, response ->
                println("  Opção ${index + 1}:")
                println("    === Entrada ===")
                println("    Parcelas da entrada: ${response.installmentQuantity}")
                println("    Valor da parcela da entrada: R$ ${"%.2f".format(response.installmentAmount)}")
                println("    Total da entrada: R$ ${"%.2f".format(response.totalAmount)}")
                
                println("    === Financiamento Principal ===")
                if (response.plans.isNotEmpty()) {
                    val mainPlan = response.plans[0]
                    println("    Parcelas principais: ${mainPlan.installment}")
                    println("    Valor da parcela principal: R$ ${"%.2f".format(mainPlan.installmentAmount)}")
                    println("    Total principal: R$ ${"%.2f".format(mainPlan.totalAmount)}")
                }
                
                println("    === Total Geral ===")
                val totalGeral = response.totalAmount + (response.plans.firstOrNull()?.totalAmount ?: 0.0)
                println("    Valor total geral: R$ ${"%.2f".format(totalGeral)}")
                println("    Valor líquido: R$ ${"%.2f".format(response.plans.firstOrNull()?.customerAmount ?: 0.0)}")
                println()
            }
        } catch (e: Exception) {
            println("Erro ao calcular plano com entrada: ${e.message}")
        }
        
        println("=" * 50 + "\n")
    }

    private fun disbursementDateExample() {
        println("3. Exemplo: Cálculo de Datas de Desembolso")

        val today = Instant.now()
        
        try {
            // Próxima data de desembolso
            val nextDate = PaymentPlan.nextDisbursementDate(today)
            println("Data base: $today")
            println("Próxima data de desembolso: $nextDate")
            
            // Intervalo de datas para 10 dias úteis
            val (startDate, endDate) = PaymentPlan.disbursementDateRange(today, 10u)
            println("Intervalo para 10 dias úteis:")
            println("  Início: $startDate")
            println("  Fim: $endDate")
            
        } catch (e: Exception) {
            println("Erro ao calcular datas: ${e.message}")
        }
        
        println("=" * 50 + "\n")
    }

    private fun nonBusinessDaysExample() {
        println("4. Exemplo: Dias Não Úteis")

        val startDate = Instant.now()
        val endDate = startDate.plus(30, ChronoUnit.DAYS)
        
        try {
            val nonBusinessDays = PaymentPlan.getNonBusinessDaysBetween(startDate, endDate)
            
            println("Período analisado: $startDate até $endDate")
            println("Dias não úteis encontrados: ${nonBusinessDays.size}")
            
            if (nonBusinessDays.isNotEmpty()) {
                println("Primeiros 5 dias não úteis:")
                nonBusinessDays.take(5).forEach { date ->
                    println("  - $date")
                }
                if (nonBusinessDays.size > 5) {
                    println("  ... e mais ${nonBusinessDays.size - 5} dias")
                }
            }
            
        } catch (e: Exception) {
            println("Erro ao buscar dias não úteis: ${e.message}")
        }
        
        println("=" * 50 + "\n")
    }

    private operator fun String.times(n: Int): String = this.repeat(n)
}
