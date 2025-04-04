package main

import (
	"fmt"
	"payment_plan/payment_plan_uniffi"
	"time"
)

func main() {

	params := payment_plan_uniffi.Params{
		RequestedAmount:                800,
		FirstPaymentDate:               time.Date(2024, 05, 3, 0, 0, 0, 0, time.UTC),
		RequestedDate:                  time.Date(2024, 04, 3, 0, 0, 0, 0, time.UTC),
		Installments:                   4,
		DebitServicePercentage:         0,
		Mdr:                            0.05,
		TacPercentage:                  0,
		IofOverall:                     0.0038,
		IofPercentage:                  0.03,
		InterestRate:                   0.0235,
		MinInstallmentAmount:           100,
		MaxTotalAmount:                 1000000,
		DisbursementOnlyOnBusinessDays: true,
	}

	resp, err := payment_plan_uniffi.CalculatePaymentPlan(params)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Response: %+v\n", resp)
}
