package com.parceladolara.paymentplan

// Import the generated uniffi bindings
// Note: You need to ensure the generated Kotlin files are in the classpath
// The uniffi-bindgen generates the files in sdks/kotlin/_internal/uniffi/payment_plan_uniffi/
import com.parceladolara.paymentplan.internal.NativeLibraryLoader
import com.parceladolara.paymentplan.internal.calculateDownPaymentPlan as internalCalculateDownPaymentPlan
import com.parceladolara.paymentplan.internal.calculatePaymentPlan as internalCalculatePaymentPlan
import com.parceladolara.paymentplan.internal.disbursementDateRange as internalDisbursementDateRange
import com.parceladolara.paymentplan.internal.getNonBusinessDaysBetween as internalGetNonBusinessDaysBetween
import com.parceladolara.paymentplan.internal.nextDisbursementDate as internalNextDisbursementDate
import java.time.Instant

/**
 * PaymentPlan SDK for Kotlin
 *
 * This SDK provides a wrapper around the payment plan calculation engine. It exposes five main
 * methods for calculating payment plans and working with disbursement dates.
 */
object PaymentPlan {

    init {
        // Load the native library when the object is first accessed
        try {
            NativeLibraryLoader.loadLibrary()
        } catch (e: Exception) {
            throw RuntimeException("Failed to initialize PaymentPlan SDK: ${e.message}", e)
        }
    }

    /**
     * Calculates a payment plan based on the provided parameters.
     *
     * @param params The parameters for calculating the payment plan
     * @return A list of Response objects representing the payment plan
     * @throws PaymentPlanException if the calculation fails
     */
    @JvmStatic
    @Throws(PaymentPlanException::class)
    fun calculatePaymentPlan(params: Params): List<Response> {
        return try {
            val internalResult = internalCalculatePaymentPlan(params.toInternal())
            internalResult.map { Response.fromInternal(it) }
        } catch (e: com.parceladolara.paymentplan.internal.Exception) {
            throw PaymentPlanException.fromInternal(e)
        }
    }

    /**
     * Calculates a down payment plan based on the provided parameters.
     *
     * @param params The parameters for calculating the down payment plan
     * @return A list of DownPaymentResponse objects representing the down payment plan
     * @throws PaymentPlanException if the calculation fails
     */
    @JvmStatic
    @Throws(PaymentPlanException::class)
    fun calculateDownPaymentPlan(params: DownPaymentParams): List<DownPaymentResponse> {
        return try {
            val internalResult = internalCalculateDownPaymentPlan(params.toInternal())
            internalResult.map { DownPaymentResponse.fromInternal(it) }
        } catch (e: com.parceladolara.paymentplan.internal.Exception) {
            throw PaymentPlanException.fromInternal(e)
        }
    }

    /**
     * Calculates the next disbursement date based on the given base date.
     *
     * This function assumes disbursement dates on business days only. This function also assumes
     * that the disbursement day can't occur on the same day as the system date, so in this case +1
     * day is added no matter what. Base dates in the past are allowed, for debugging purposes, but
     * keep the rule of not being the same day in mind.
     *
     * @param baseDate The base date to calculate the next disbursement date
     * @return The next disbursement date
     */
    @JvmStatic
    fun nextDisbursementDate(baseDate: Instant): Instant {
        return internalNextDisbursementDate(baseDate)
    }

    /**
     * Calculates and returns (start, end) disbursement dates based on the given base date and
     * number of days.
     *
     * The start date is the next disbursement date and the end date is the end of a range that fits
     * the number of business days. For example:
     * ```
     *     baseDate = "2025-04-03"
     *     days = 5
     *     start = "2025-04-03" (assuming that the call was not made on "2025-04-03")
     *     end = "2025-04-09"
     * ```
     * This range fits 5 business days: 2025-04-03, 2025-04-04, 2025-04-07, 2025-04-08, and
     * 2025-04-09.
     *
     * This function assumes disbursement dates on business days only. This function also assumes
     * that the disbursement day can't occur on the same day as the system date, so in this case +1
     * day is added no matter what. Base dates in the past are allowed, for debugging purposes, but
     * keep the rule of not being the same day in mind.
     *
     * @param baseDate The base date to calculate the disbursement date range
     * @param days The number of business days in the range
     * @return A Pair containing the start and end disbursement dates
     */
    @JvmStatic
    fun disbursementDateRange(baseDate: Instant, days: UInt): Pair<Instant, Instant> {
        val result = internalDisbursementDateRange(baseDate, days)
        return Pair(result[0], result[1])
    }

    /**
     * Returns a list of non-business days between the given start and end dates.
     *
     * Both start and end dates are inclusive.
     *
     * This function assumes disbursement dates on business days only.
     *
     * @param startDate The start date of the range
     * @param endDate The end date of the range
     * @return A list of non-business days within the range
     */
    @JvmStatic
    fun getNonBusinessDaysBetween(startDate: Instant, endDate: Instant): List<Instant> {
        return internalGetNonBusinessDaysBetween(startDate, endDate)
    }
}
