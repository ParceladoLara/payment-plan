package com.parceladolara.paymentplan

import java.time.Instant

/** Parameters for calculating down payment plans. */
data class DownPaymentParams(
        val params: Params,
        val requestedAmount: Double,
        val minInstallmentAmount: Double,
        val firstPaymentDate: Instant,
        val installments: UInt
) {
  /**
   * Builder class for Java interoperability. Provides a fluent API for constructing
   * DownPaymentParams objects.
   */
  class Builder {
    private var params: Params? = null
    private var requestedAmount: Double = 0.0
    private var minInstallmentAmount: Double = 0.0
    private var firstPaymentDate: Instant = Instant.now()
    private var installments: UInt = 1u

    fun params(params: Params) = apply { this.params = params }
    fun requestedAmount(requestedAmount: Double) = apply { this.requestedAmount = requestedAmount }
    fun minInstallmentAmount(minInstallmentAmount: Double) = apply {
      this.minInstallmentAmount = minInstallmentAmount
    }
    fun firstPaymentDate(firstPaymentDate: Instant) = apply {
      this.firstPaymentDate = firstPaymentDate
    }
    fun installments(installments: UInt) = apply { this.installments = installments }
    fun installments(installments: Int) = apply { this.installments = installments.toUInt() }

    fun build(): DownPaymentParams {
      requireNotNull(params) { "params must be set" }
      return DownPaymentParams(
              params = params!!,
              requestedAmount = requestedAmount,
              minInstallmentAmount = minInstallmentAmount,
              firstPaymentDate = firstPaymentDate,
              installments = installments
      )
    }
  }

  companion object {
    /**
     * Creates a new Builder instance for constructing DownPaymentParams objects. Useful for Java
     * interoperability.
     *
     * @return A new Builder instance
     */
    @JvmStatic fun builder(): Builder = Builder()

    internal fun fromInternal(
            internal: com.parceladolara.paymentplan.internal.InternalDownPaymentParams
    ): DownPaymentParams {
      return DownPaymentParams(
              params = Params.fromInternal(internal.params),
              requestedAmount = internal.requestedAmount,
              minInstallmentAmount = internal.minInstallmentAmount,
              firstPaymentDate = internal.firstPaymentDate,
              installments = internal.installments
      )
    }
  }

  internal fun toInternal(): com.parceladolara.paymentplan.internal.InternalDownPaymentParams {
    return com.parceladolara.paymentplan.internal.InternalDownPaymentParams(
            params = this.params.toInternal(),
            requestedAmount = this.requestedAmount,
            minInstallmentAmount = this.minInstallmentAmount,
            firstPaymentDate = this.firstPaymentDate,
            installments = this.installments
    )
  }
}
