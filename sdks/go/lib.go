package main

import (
	"fmt"
	"payment_plan/payment_plan_uniffi"
	"time"
)

func main() {

	params := payment_plan_uniffi.Params{
		RequestedAmount:                7800,
		FirstPaymentDate:               time.Date(2025, 05, 3, 0, 0, 0, 0, time.UTC),
		RequestedDate:                  time.Date(2025, 04, 5, 0, 0, 0, 0, time.UTC),
		Installments:                   24,
		DebitServicePercentage:         0,
		Mdr:                            0.05,
		TacPercentage:                  0,
		IofOverall:                     0.0038,
		IofPercentage:                  0.000082,
		InterestRate:                   0.0235,
		MinInstallmentAmount:           100,
		MaxTotalAmount:                 1000000,
		DisbursementOnlyOnBusinessDays: true,
	}

	resp, err := payment_plan_uniffi.CalculatePaymentPlan(params)
	if err != nil {
		panic(err)
	}
	for _, installment := range resp {
		fmt.Printf("Installment: %d\n", installment.Installment)
		fmt.Printf("Due Date: %s\n", installment.DueDate.Format("2006-01-02"))
		fmt.Printf("Disbursement Date: %s\n", installment.DisbursementDate.Format("2006-01-02"))
		fmt.Printf("Accumulated Days: %d\n", installment.AccumulatedDays)
		fmt.Printf("Days Index: %f\n", installment.DaysIndex)
		fmt.Printf("Accumulated Days Index: %f\n", installment.AccumulatedDaysIndex)
		fmt.Printf("Interest Rate: %f\n", installment.InterestRate)
		fmt.Printf("Installment Amount: %f\n", installment.InstallmentAmount)
		fmt.Printf("Installment Amount Without TAC: %f\n", installment.InstallmentAmountWithoutTac)
		fmt.Printf("Total Amount: %f\n", installment.TotalAmount)
		fmt.Printf("Debit Service: %f\n", installment.DebitService)
		fmt.Printf("Customer Debit Service Amount: %f\n", installment.CustomerDebitServiceAmount)
		fmt.Printf("Customer Amount: %f\n", installment.CustomerAmount)
		fmt.Printf("Calculation Basis for Effective Interest Rate: %f\n", installment.CalculationBasisForEffectiveInterestRate)
		fmt.Printf("Merchant Debit Service Amount: %f\n", installment.MerchantDebitServiceAmount)
		fmt.Printf("Merchant Total Amount: %f\n", installment.MerchantTotalAmount)
		fmt.Printf("Settled to Merchant: %f\n", installment.SettledToMerchant)
		fmt.Printf("MDR Amount: %f\n", installment.MdrAmount)
		fmt.Printf("Effective Interest Rate: %f\n", installment.EffectiveInterestRate)
		fmt.Printf("Total Effective Cost: %f\n", installment.TotalEffectiveCost)
		fmt.Printf("EIR Yearly: %f\n", installment.EirYearly)
		fmt.Printf("TEC Yearly: %f\n", installment.TecYearly)
		fmt.Printf("EIR Monthly: %f\n", installment.EirMonthly)
		fmt.Printf("TEC Monthly: %f\n", installment.TecMonthly)
		fmt.Printf("Total IOF: %f\n", installment.TotalIof)
		fmt.Printf("Contract Amount: %f\n", installment.ContractAmount)
		fmt.Printf("Contract Amount Without TAC: %f\n", installment.ContractAmountWithoutTac)
		fmt.Printf("TAC Amount: %f\n", installment.TacAmount)
		fmt.Printf("IOF Percentage: %f\n", installment.IofPercentage)
		fmt.Printf("Overall IOF: %f\n", installment.OverallIof)
		fmt.Printf("Pre Disbursement Amount: %f\n", installment.PreDisbursementAmount)
		fmt.Printf("Paid Total IOF: %f\n", installment.PaidTotalIof)
		fmt.Printf("Paid Contract Amount: %f\n", installment.PaidContractAmount)
		fmt.Println("--------------------------------------------------")
	}
}
