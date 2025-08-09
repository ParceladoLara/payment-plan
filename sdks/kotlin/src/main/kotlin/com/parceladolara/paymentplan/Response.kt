package com.parceladolara.paymentplan

import java.time.Instant

/** Represents a payment plan response with detailed calculation results. */
data class Response(
        val installment: UInt,
        val dueDate: Instant,
        val disbursementDate: Instant,
        val accumulatedDays: Long,
        val daysIndex: Double,
        val accumulatedDaysIndex: Double,
        val interestRate: Double,
        val installmentAmount: Double,
        val installmentAmountWithoutTac: Double,
        val totalAmount: Double,
        val debitService: Double,
        val customerDebitServiceAmount: Double,
        val customerAmount: Double,
        val calculationBasisForEffectiveInterestRate: Double,
        val merchantDebitServiceAmount: Double,
        val merchantTotalAmount: Double,
        val settledToMerchant: Double,
        val mdrAmount: Double,
        val effectiveInterestRate: Double,
        val totalEffectiveCost: Double,
        val eirYearly: Double,
        val tecYearly: Double,
        val eirMonthly: Double,
        val tecMonthly: Double,
        val totalIof: Double,
        val contractAmount: Double,
        val contractAmountWithoutTac: Double,
        val tacAmount: Double,
        val iofPercentage: Double,
        val overallIof: Double,
        val preDisbursementAmount: Double,
        val paidTotalIof: Double,
        val paidContractAmount: Double,
        val invoices: List<Invoice>
) {
        companion object {
                internal fun fromInternal(
                        internal: com.parceladolara.paymentplan.internal.InternalResponse
                ): Response {
                        return Response(
                                installment = internal.installment,
                                dueDate = internal.dueDate,
                                disbursementDate = internal.disbursementDate,
                                accumulatedDays = internal.accumulatedDays,
                                daysIndex = internal.daysIndex,
                                accumulatedDaysIndex = internal.accumulatedDaysIndex,
                                interestRate = internal.interestRate,
                                installmentAmount = internal.installmentAmount,
                                installmentAmountWithoutTac = internal.installmentAmountWithoutTac,
                                totalAmount = internal.totalAmount,
                                debitService = internal.debitService,
                                customerDebitServiceAmount = internal.customerDebitServiceAmount,
                                customerAmount = internal.customerAmount,
                                calculationBasisForEffectiveInterestRate =
                                        internal.calculationBasisForEffectiveInterestRate,
                                merchantDebitServiceAmount = internal.merchantDebitServiceAmount,
                                merchantTotalAmount = internal.merchantTotalAmount,
                                settledToMerchant = internal.settledToMerchant,
                                mdrAmount = internal.mdrAmount,
                                effectiveInterestRate = internal.effectiveInterestRate,
                                totalEffectiveCost = internal.totalEffectiveCost,
                                eirYearly = internal.eirYearly,
                                tecYearly = internal.tecYearly,
                                eirMonthly = internal.eirMonthly,
                                tecMonthly = internal.tecMonthly,
                                totalIof = internal.totalIof,
                                contractAmount = internal.contractAmount,
                                contractAmountWithoutTac = internal.contractAmountWithoutTac,
                                tacAmount = internal.tacAmount,
                                iofPercentage = internal.iofPercentage,
                                overallIof = internal.overallIof,
                                preDisbursementAmount = internal.preDisbursementAmount,
                                paidTotalIof = internal.paidTotalIof,
                                paidContractAmount = internal.paidContractAmount,
                                invoices = internal.invoices.map { Invoice.fromInternal(it) }
                        )
                }
        }

        internal fun toInternal(): com.parceladolara.paymentplan.internal.InternalResponse {
                return com.parceladolara.paymentplan.internal.InternalResponse(
                        installment = this.installment,
                        dueDate = this.dueDate,
                        disbursementDate = this.disbursementDate,
                        accumulatedDays = this.accumulatedDays,
                        daysIndex = this.daysIndex,
                        accumulatedDaysIndex = this.accumulatedDaysIndex,
                        interestRate = this.interestRate,
                        installmentAmount = this.installmentAmount,
                        installmentAmountWithoutTac = this.installmentAmountWithoutTac,
                        totalAmount = this.totalAmount,
                        debitService = this.debitService,
                        customerDebitServiceAmount = this.customerDebitServiceAmount,
                        customerAmount = this.customerAmount,
                        calculationBasisForEffectiveInterestRate =
                                this.calculationBasisForEffectiveInterestRate,
                        merchantDebitServiceAmount = this.merchantDebitServiceAmount,
                        merchantTotalAmount = this.merchantTotalAmount,
                        settledToMerchant = this.settledToMerchant,
                        mdrAmount = this.mdrAmount,
                        effectiveInterestRate = this.effectiveInterestRate,
                        totalEffectiveCost = this.totalEffectiveCost,
                        eirYearly = this.eirYearly,
                        tecYearly = this.tecYearly,
                        eirMonthly = this.eirMonthly,
                        tecMonthly = this.tecMonthly,
                        totalIof = this.totalIof,
                        contractAmount = this.contractAmount,
                        contractAmountWithoutTac = this.contractAmountWithoutTac,
                        tacAmount = this.tacAmount,
                        iofPercentage = this.iofPercentage,
                        overallIof = this.overallIof,
                        preDisbursementAmount = this.preDisbursementAmount,
                        paidTotalIof = this.paidTotalIof,
                        paidContractAmount = this.paidContractAmount,
                        invoices = this.invoices.map { it.toInternal() }
                )
        }
}
