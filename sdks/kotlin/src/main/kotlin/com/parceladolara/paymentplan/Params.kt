package com.parceladolara.paymentplan

import java.time.Instant

/** Parameters for calculating payment plans. */
data class Params(
        val requestedAmount: Double,
        val firstPaymentDate: Instant,
        val disbursementDate: Instant,
        val installments: UInt,
        val debitServicePercentage: UShort,
        val mdr: Double,
        val tacPercentage: Double,
        val iofOverall: Double,
        val iofPercentage: Double,
        val interestRate: Double,
        val minInstallmentAmount: Double,
        val maxTotalAmount: Double,
        val disbursementOnlyOnBusinessDays: Boolean
) {
  /**
   * Builder class for Java interoperability. Provides a fluent API for constructing Params objects.
   */
  class Builder {
    private var requestedAmount: Double = 0.0
    private var firstPaymentDate: Instant = Instant.now()
    private var disbursementDate: Instant = Instant.now()
    private var installments: UInt = 1u
    private var debitServicePercentage: UShort = 0u
    private var mdr: Double = 0.0
    private var tacPercentage: Double = 0.0
    private var iofOverall: Double = 0.0
    private var iofPercentage: Double = 0.0
    private var interestRate: Double = 0.0
    private var minInstallmentAmount: Double = 0.0
    private var maxTotalAmount: Double = 0.0
    private var disbursementOnlyOnBusinessDays: Boolean = true

    fun requestedAmount(requestedAmount: Double) = apply { this.requestedAmount = requestedAmount }
    fun firstPaymentDate(firstPaymentDate: Instant) = apply {
      this.firstPaymentDate = firstPaymentDate
    }
    fun disbursementDate(disbursementDate: Instant) = apply {
      this.disbursementDate = disbursementDate
    }
    fun installments(installments: UInt) = apply { this.installments = installments }
    fun installments(installments: Int) = apply { this.installments = installments.toUInt() }
    fun debitServicePercentage(debitServicePercentage: UShort) = apply {
      this.debitServicePercentage = debitServicePercentage
    }
    fun debitServicePercentage(debitServicePercentage: Int) = apply {
      this.debitServicePercentage = debitServicePercentage.toUShort()
    }
    fun mdr(mdr: Double) = apply { this.mdr = mdr }
    fun tacPercentage(tacPercentage: Double) = apply { this.tacPercentage = tacPercentage }
    fun iofOverall(iofOverall: Double) = apply { this.iofOverall = iofOverall }
    fun iofPercentage(iofPercentage: Double) = apply { this.iofPercentage = iofPercentage }
    fun interestRate(interestRate: Double) = apply { this.interestRate = interestRate }
    fun minInstallmentAmount(minInstallmentAmount: Double) = apply {
      this.minInstallmentAmount = minInstallmentAmount
    }
    fun maxTotalAmount(maxTotalAmount: Double) = apply { this.maxTotalAmount = maxTotalAmount }
    fun disbursementOnlyOnBusinessDays(disbursementOnlyOnBusinessDays: Boolean) = apply {
      this.disbursementOnlyOnBusinessDays = disbursementOnlyOnBusinessDays
    }

    fun build(): Params {
      return Params(
              requestedAmount = requestedAmount,
              firstPaymentDate = firstPaymentDate,
              disbursementDate = disbursementDate,
              installments = installments,
              debitServicePercentage = debitServicePercentage,
              mdr = mdr,
              tacPercentage = tacPercentage,
              iofOverall = iofOverall,
              iofPercentage = iofPercentage,
              interestRate = interestRate,
              minInstallmentAmount = minInstallmentAmount,
              maxTotalAmount = maxTotalAmount,
              disbursementOnlyOnBusinessDays = disbursementOnlyOnBusinessDays
      )
    }
  }

  companion object {
    /**
     * Creates a new Builder instance for constructing Params objects. Useful for Java
     * interoperability.
     *
     * @return A new Builder instance
     */
    @JvmStatic fun builder(): Builder = Builder()

    internal fun fromInternal(
            internal: com.parceladolara.paymentplan.internal.InternalParams
    ): Params {
      return Params(
              requestedAmount = internal.requestedAmount,
              firstPaymentDate = internal.firstPaymentDate,
              disbursementDate = internal.disbursementDate,
              installments = internal.installments,
              debitServicePercentage = internal.debitServicePercentage,
              mdr = internal.mdr,
              tacPercentage = internal.tacPercentage,
              iofOverall = internal.iofOverall,
              iofPercentage = internal.iofPercentage,
              interestRate = internal.interestRate,
              minInstallmentAmount = internal.minInstallmentAmount,
              maxTotalAmount = internal.maxTotalAmount,
              disbursementOnlyOnBusinessDays = internal.disbursementOnlyOnBusinessDays
      )
    }
  }

  internal fun toInternal(): com.parceladolara.paymentplan.internal.InternalParams {
    return com.parceladolara.paymentplan.internal.InternalParams(
            requestedAmount = this.requestedAmount,
            firstPaymentDate = this.firstPaymentDate,
            disbursementDate = this.disbursementDate,
            installments = this.installments,
            debitServicePercentage = this.debitServicePercentage,
            mdr = this.mdr,
            tacPercentage = this.tacPercentage,
            iofOverall = this.iofOverall,
            iofPercentage = this.iofPercentage,
            interestRate = this.interestRate,
            minInstallmentAmount = this.minInstallmentAmount,
            maxTotalAmount = this.maxTotalAmount,
            disbursementOnlyOnBusinessDays = this.disbursementOnlyOnBusinessDays
    )
  }
}
