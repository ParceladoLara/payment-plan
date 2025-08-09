package com.parceladolara.paymentplan

/** Exceptions that can be thrown by the payment plan calculations. */
sealed class PaymentPlanException : Exception() {

  /** Thrown when invalid parameters are provided to the calculation functions. */
  class InvalidParams(message: String = "Invalid parameters provided") : PaymentPlanException() {
    override val message: String = message
  }

  /** Thrown when an error occurs during payment plan calculation. */
  class CalculationException(message: String = "Error occurred during calculation") :
          PaymentPlanException() {
    override val message: String = message
  }

  companion object {
    internal fun fromInternal(
            internal: com.parceladolara.paymentplan.internal.Exception
    ): PaymentPlanException {
      return when (internal) {
        is com.parceladolara.paymentplan.internal.Exception.InvalidParams ->
                InvalidParams(internal.message.ifEmpty { "Invalid parameters provided" })
        is com.parceladolara.paymentplan.internal.Exception.CalculationException ->
                CalculationException(
                        internal.message.ifEmpty { "Error occurred during calculation" }
                )
      }
    }
  }
}
