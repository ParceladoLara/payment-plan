package payment_plan

import (
	"time"

	"github.com/ParceladoLara/payment-plan-go-sdk/internal/payment_plan_uniffi"
)

type Params = payment_plan_uniffi.Params
type Response = payment_plan_uniffi.Response
type DownPaymentParams = payment_plan_uniffi.DownPaymentParams
type DownPaymentResponse = payment_plan_uniffi.DownPaymentResponse

func CalculatePaymentPlan(params Params) ([]Response, error) {
	response, err := payment_plan_uniffi.CalculatePaymentPlan(params)
	if err != nil {
		return nil, err
	}
	convertedResponse := make([]Response, len(response))
	copy(convertedResponse, response)
	return convertedResponse, nil
}

func CalculateDownPaymentPlan(params DownPaymentParams) ([]DownPaymentResponse, error) {
	response, err := payment_plan_uniffi.CalculateDownPaymentPlan(params)
	if err != nil {
		return nil, err
	}
	convertedResponse := make([]DownPaymentResponse, len(response))
	copy(convertedResponse, response)
	return convertedResponse, nil
}

// NextDisbursementDate calculates the next disbursement date based on the given base date.
//
// This function assumes disbursement dates on business days only.
// This function also assumes that the disbursement day can't occur on the same day as the system date, so in this case +1 day is added no matter what.
// baseDates in the past are allowed, for debugging purposes. but keep the rule of not being the same day in mind.
func NextDisbursementDate(baseDate time.Time) time.Time {
	return payment_plan_uniffi.NextDisbursementDate(baseDate)
}

// DisbursementDateRange calculates and returns (start, end) disbursement dates based on the given base date and number of days.
// The start date is the next disbursement date and the end date is the end of a range that fits the number of business days.
// for example:
//
//	baseDate = "2025-04-03"
//	days = 5
//	start = "2025-04-03"(assuming that the call was not made on "2025-04-03")
//	end = "2025-04-09"
//
// This range fits 5 business days, 2025-04-03, 2025-04-04, 2025-04-07, 2025-04-08 and 2025-04-09.
//
// This function assumes disbursement dates on business days only.
// This function also assumes that the disbursement day can't occur on the same day as the system date, so in this case +1 day is added no matter what.
// baseDates in the past are allowed, for debugging purposes. but keep the rule of not being the same day in mind.
func DisbursementDateRange(baseDate time.Time, days uint32) (time.Time, time.Time) {
	result := payment_plan_uniffi.DisbursementDateRange(baseDate, days)
	return result[0], result[1]
}

// GetNonBusinessDaysBetween returns a slice of non-business days between the given start and end dates.
// Both start and end dates are inclusive.
//
// This function assumes disbursement dates on business days only.
func GetNonBusinessDaysBetween(startDate time.Time, endDate time.Time) []time.Time {
	return payment_plan_uniffi.GetNonBusinessDaysBetween(startDate, endDate)
}
