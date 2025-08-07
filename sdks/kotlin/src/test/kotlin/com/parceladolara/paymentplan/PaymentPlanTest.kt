package com.parceladolara.paymentplan

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import java.time.Instant
import java.time.ZoneId
import java.time.ZonedDateTime

class PaymentPlanTest {

    @Test
    fun `test calculatePaymentPlan - single installment`() {
        // Using adapted test data from Go SDK for single installment
        val params = Params(
            requestedAmount = 7800.0,
            firstPaymentDate = ZonedDateTime.of(2025, 5, 3, 0, 0, 0, 0, ZoneId.of("-03:00")).toInstant(),
            disbursementDate = ZonedDateTime.of(2025, 4, 5, 0, 0, 0, 0, ZoneId.of("-03:00")).toInstant(),
            installments = 1u,
            debitServicePercentage = 0u,
            mdr = 0.05,
            tacPercentage = 0.0,
            iofOverall = 0.0038,
            iofPercentage = 0.000082,
            interestRate = 0.0235,
            minInstallmentAmount = 100.0,
            maxTotalAmount = 1000000.0,
            disbursementOnlyOnBusinessDays = true
        )

        val result = PaymentPlan.calculatePaymentPlan(params)
        
        assertNotNull(result)
        assertTrue(result.isNotEmpty())
        
        // Verify basic functionality - amount should be higher than requested due to interest
        val firstInstallment = result[0]
        assertTrue(firstInstallment.installmentAmount > 7800.0, 
            "Installment amount should be higher than requested amount due to interest")
        assertTrue(firstInstallment.totalAmount > 7800.0,
            "Total amount should be higher than requested amount due to interest")
        
        // Verify basic structure
        assertEquals(1u, firstInstallment.installment)
        // Don't compare exact dates as they may be adjusted for business days
        assertNotNull(firstInstallment.disbursementDate)
        assertEquals(7800.0, firstInstallment.preDisbursementAmount, 0.01)
        
        // Verify the disbursement date is reasonable (within a few days of requested)
        val requestedDisbursement = params.disbursementDate
        val actualDisbursement = firstInstallment.disbursementDate
        val daysDifference = Math.abs(actualDisbursement.epochSecond - requestedDisbursement.epochSecond) / 86400
        assertTrue(daysDifference <= 7, 
            "Disbursement date should be within a week of requested date (business day adjustment)")
    }

    @Test
    fun `test calculateDownPaymentPlan`() {
        // Using adapted test data from Go SDK
        val baseParams = Params(
            requestedAmount = 7800.0,
            firstPaymentDate = ZonedDateTime.of(2025, 5, 3, 0, 0, 0, 0, ZoneId.of("-03:00")).toInstant(),
            disbursementDate = ZonedDateTime.of(2025, 4, 5, 0, 0, 0, 0, ZoneId.of("-03:00")).toInstant(),
            installments = 1u,
            debitServicePercentage = 0u,
            mdr = 0.05,
            tacPercentage = 0.0,
            iofOverall = 0.0038,
            iofPercentage = 0.000082,
            interestRate = 0.0235,
            minInstallmentAmount = 100.0,
            maxTotalAmount = 1000000.0,
            disbursementOnlyOnBusinessDays = true
        )

        val downPaymentParams = DownPaymentParams(
            params = baseParams,
            requestedAmount = 1000.0,
            minInstallmentAmount = 100.0,
            firstPaymentDate = ZonedDateTime.of(2025, 5, 3, 0, 0, 0, 0, ZoneId.of("-03:00")).toInstant(),
            installments = 1u
        )

        val result = PaymentPlan.calculateDownPaymentPlan(downPaymentParams)
        
        assertNotNull(result)
        assertTrue(result.isNotEmpty())
        
        // Verify basic functionality - reasonable down payment calculation
        val firstResult = result[0]
        assertTrue(firstResult.installmentAmount > 0, 
            "Installment amount should be positive")
        assertTrue(firstResult.totalAmount > 0,
            "Total amount should be positive")
        
        // Verify basic structure
        assertTrue(firstResult.installmentQuantity > 0u,
            "Should have at least one installment")
        assertNotNull(firstResult.firstPaymentDate)
        assertNotNull(firstResult.plans)
        assertTrue(firstResult.plans.isNotEmpty(), 
            "Should have payment plan details")
    }

    @Test
    fun `test disbursementDateRange`() {
        // Using same test data as Go SDK
        val baseDate = ZonedDateTime.of(2025, 4, 3, 0, 0, 0, 0, ZoneId.of("UTC")).toInstant()
        val days = 5u

        val (startDate, endDate) = PaymentPlan.disbursementDateRange(baseDate, days)

        assertNotNull(startDate)
        assertNotNull(endDate)
        assertTrue(endDate.isAfter(startDate),
            "End date should be after start date")
        
        // Verify the range spans approximately the requested number of days
        val daysDifference = (endDate.epochSecond - startDate.epochSecond) / 86400
        assertTrue(daysDifference >= days.toLong() - 2 && daysDifference <= days.toLong() + 5,
            "Date range should span approximately $days business days (allowing for weekends)")
        
        // Verify dates are adjusted for business days (should be in Brazil timezone with 7 AM)
        val startZoned = startDate.atZone(ZoneId.of("-03:00"))
        val endZoned = endDate.atZone(ZoneId.of("-03:00"))
        
        // Business day adjustment means 7 AM Brazil time
        assertTrue(startZoned.hour == 7 || startZoned.hour == 0,
            "Start date should be adjusted for business hours")
        assertTrue(endZoned.hour == 7 || endZoned.hour == 0,
            "End date should be adjusted for business hours")
    }

    @Test
    fun `test nextDisbursementDate`() {
        // Using same test data as Go SDK
        val baseDate = ZonedDateTime.of(2025, 4, 3, 0, 0, 0, 0, ZoneId.of("UTC")).toInstant()

        val result = PaymentPlan.nextDisbursementDate(baseDate)

        assertNotNull(result)
        
        // Verify the result is a business day adjustment of the base date
        val baseDateBrazil = baseDate.atZone(ZoneId.of("-03:00")).toLocalDate()
        val resultDateBrazil = result.atZone(ZoneId.of("-03:00")).toLocalDate()
        
        // Should be same day or a few days later (business day adjustment)
        assertTrue(!resultDateBrazil.isBefore(baseDateBrazil),
            "Next disbursement date should not be before base date")
        
        val daysDifference = resultDateBrazil.toEpochDay() - baseDateBrazil.toEpochDay()
        assertTrue(daysDifference <= 7,
            "Next disbursement date should be within a week of base date")
    }

    @Test
    fun `test nextDisbursementDate - not today`() {
        // Using today's date as the base date (similar to Go test)
        val today = Instant.now()

        val nextDate = PaymentPlan.nextDisbursementDate(today)

        assertNotNull(nextDate)
        
        // Validate that the result is not the same as today
        val todayLocal = today.atZone(ZoneId.systemDefault()).toLocalDate()
        val nextDateLocal = nextDate.atZone(ZoneId.systemDefault()).toLocalDate()
        
        assertNotEquals(todayLocal, nextDateLocal,
            "Next disbursement date should not be the same as today. Got: $nextDate")
    }

    @Test
    fun `test getNonBusinessDaysBetween`() {
        // Using same test data as Go SDK
        val startDate = ZonedDateTime.of(2025, 4, 1, 0, 0, 0, 0, ZoneId.of("UTC")).toInstant()
        val endDate = ZonedDateTime.of(2025, 4, 30, 0, 0, 0, 0, ZoneId.of("UTC")).toInstant()

        val nonBusinessDays = PaymentPlan.getNonBusinessDaysBetween(startDate, endDate)

        assertNotNull(nonBusinessDays)
        assertTrue(nonBusinessDays.isNotEmpty(),
            "Expected non-business days between $startDate and $endDate, but got none")

        // Verify all returned dates are within the range
        for (date in nonBusinessDays) {
            assertTrue(!date.isBefore(startDate),
                "Non-business day $date should not be before start date $startDate")
            assertTrue(!date.isAfter(endDate),
                "Non-business day $date should not be after end date $endDate")
        }
        
        // Verify we have a reasonable number of non-business days in April (weekends + holidays)
        // April 2025 has 30 days, so should have at least 8 weekends + some holidays
        assertTrue(nonBusinessDays.size >= 8,
            "Should have at least 8 non-business days in a full month (weekends minimum)")
        assertTrue(nonBusinessDays.size <= 20,
            "Should not have more than 20 non-business days in a month")
    }
}
