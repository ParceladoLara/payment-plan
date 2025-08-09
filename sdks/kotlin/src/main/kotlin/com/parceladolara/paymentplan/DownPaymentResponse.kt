package com.parceladolara.paymentplan

import java.time.Instant

/** Response for down payment plan calculations. */
data class DownPaymentResponse(
        val installmentAmount: Double,
        val totalAmount: Double,
        val installmentQuantity: UInt,
        val firstPaymentDate: Instant,
        val plans: List<Response>
) {
        companion object {
                internal fun fromInternal(
                        internal: com.parceladolara.paymentplan.internal.InternalDownPaymentResponse
                ): DownPaymentResponse {
                        return DownPaymentResponse(
                                installmentAmount = internal.installmentAmount,
                                totalAmount = internal.totalAmount,
                                installmentQuantity = internal.installmentQuantity,
                                firstPaymentDate = internal.firstPaymentDate,
                                plans = internal.plans.map { Response.fromInternal(it) }
                        )
                }
        }

        internal fun toInternal():
                com.parceladolara.paymentplan.internal.InternalDownPaymentResponse {
                return com.parceladolara.paymentplan.internal.InternalDownPaymentResponse(
                        installmentAmount = this.installmentAmount,
                        totalAmount = this.totalAmount,
                        installmentQuantity = this.installmentQuantity,
                        firstPaymentDate = this.firstPaymentDate,
                        plans = this.plans.map { it.toInternal() }
                )
        }
}
