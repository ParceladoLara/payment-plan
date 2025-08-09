package com.parceladolara.paymentplan

import java.time.Instant

/** Represents an invoice with payment details and due date information. */
data class Invoice(
        val accumulatedDays: Long,
        val factor: Double,
        val accumulatedFactor: Double,
        val dueDate: Instant
) {
  companion object {
    internal fun fromInternal(
            internal: com.parceladolara.paymentplan.internal.InternalInvoice
    ): Invoice {
      return Invoice(
              accumulatedDays = internal.accumulatedDays,
              factor = internal.factor,
              accumulatedFactor = internal.accumulatedFactor,
              dueDate = internal.dueDate
      )
    }
  }

  internal fun toInternal(): com.parceladolara.paymentplan.internal.InternalInvoice {
    return com.parceladolara.paymentplan.internal.InternalInvoice(
            accumulatedDays = this.accumulatedDays,
            factor = this.factor,
            accumulatedFactor = this.accumulatedFactor,
            dueDate = this.dueDate
    )
  }
}
