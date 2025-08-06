package com.parceladolara.paymentplan

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import java.time.Instant
import java.time.temporal.ChronoUnit

class PaymentPlanTest {

    @Test
    fun `test calculatePaymentPlan`() {
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

        val result = PaymentPlan.calculatePaymentPlan(params)
        assertNotNull(result)
        assertTrue(result.isNotEmpty())
    }

    @Test
    fun `test calculateDownPaymentPlan`() {
        val baseParams = Params(
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

        val downPaymentParams = DownPaymentParams(
            params = baseParams,
            requestedAmount = 200.0,
            minInstallmentAmount = 25.0,
            firstPaymentDate = Instant.now().plus(15, ChronoUnit.DAYS),
            installments = 6u
        )

        val result = PaymentPlan.calculateDownPaymentPlan(downPaymentParams)
        assertNotNull(result)
        assertTrue(result.isNotEmpty())
    }

    @Test
    fun `test nextDisbursementDate`() {
        val baseDate = Instant.now()
        val nextDate = PaymentPlan.nextDisbursementDate(baseDate)
        
        assertNotNull(nextDate)
        assertTrue(nextDate.isAfter(baseDate))
    }

    @Test
    fun `test disbursementDateRange`() {
        val baseDate = Instant.now()
        val days = 5u
        
        val (startDate, endDate) = PaymentPlan.disbursementDateRange(baseDate, days)
        
        assertNotNull(startDate)
        assertNotNull(endDate)
        assertTrue(endDate.isAfter(startDate))
    }

    @Test
    fun `test getNonBusinessDaysBetween`() {
        val startDate = Instant.now()
        val endDate = startDate.plus(30, ChronoUnit.DAYS)
        
        val nonBusinessDays = PaymentPlan.getNonBusinessDaysBetween(startDate, endDate)
        
        assertNotNull(nonBusinessDays)
        // Should have some non-business days in a 30-day period
        assertTrue(nonBusinessDays.isNotEmpty())
    }
}
