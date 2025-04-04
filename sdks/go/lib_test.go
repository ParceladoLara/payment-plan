package payment_plan_test

import (
	"testing"
	"time"

	payment_plan "github.com/ParceladoLara/payment-plan-go-sdk"
)

func TestCalculatePaymentPlan(t *testing.T) {

	disbursementDate := time.Date(2025, 04, 7, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60))

	expected := []payment_plan.Response{
		{
			Installment:                              1,
			DueDate:                                  time.Date(2025, 05, 5, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			DisbursementDate:                         disbursementDate,
			AccumulatedDays:                          28,
			DaysIndex:                                0.981371965896169,
			AccumulatedDaysIndex:                     0.981371965896169,
			InterestRate:                             0.0235,
			InstallmentAmount:                        7996.8,
			InstallmentAmountWithoutTac:              0.0,
			TotalAmount:                              7996.8,
			DebitService:                             148.96000000000018,
			CustomerDebitServiceAmount:               148.96000000000018,
			CustomerAmount:                           7996.8,
			CalculationBasisForEffectiveInterestRate: 7948.96,
			MerchantDebitServiceAmount:               0.0,
			MerchantTotalAmount:                      390.0,
			SettledToMerchant:                        7410.0,
			MdrAmount:                                390.0,
			EffectiveInterestRate:                    0.0206,
			TotalEffectiveCost:                       0.0274,
			EirYearly:                                0.277782,
			TecYearly:                                0.383782,
			EirMonthly:                               0.0206,
			TecMonthly:                               0.0274,
			TotalIof:                                 47.84,
			ContractAmount:                           7847.84,
			ContractAmountWithoutTac:                 0.0,
			TacAmount:                                0.0,
			IofPercentage:                            8.2e-5,
			OverallIof:                               0.0038,
			PreDisbursementAmount:                    7800.0,
			PaidTotalIof:                             47.84,
			PaidContractAmount:                       7847.84,
		},
		{

			Installment:                              2,
			DueDate:                                  time.Date(2025, 06, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			DisbursementDate:                         disbursementDate,
			AccumulatedDays:                          57,
			DaysIndex:                                0.958839243657051,
			AccumulatedDaysIndex:                     1.94021120955322,
			InterestRate:                             0.0235,
			InstallmentAmount:                        4049.72,
			InstallmentAmountWithoutTac:              0.0,
			TotalAmount:                              8099.44,
			DebitService:                             242.1299999999996,
			CustomerDebitServiceAmount:               242.1299999999996,
			CustomerAmount:                           4049.72,
			CalculationBasisForEffectiveInterestRate: 4021.0649999999996,
			MerchantDebitServiceAmount:               0.0,
			MerchantTotalAmount:                      390.0,
			SettledToMerchant:                        7410.0,
			MdrAmount:                                390.0,
			EffectiveInterestRate:                    0.022,
			TotalEffectiveCost:                       0.0274,
			EirYearly:                                0.298378,
			TecYearly:                                0.382981,
			EirMonthly:                               0.022,
			TecMonthly:                               0.0274,
			TotalIof:                                 57.31,
			ContractAmount:                           7857.31,
			ContractAmountWithoutTac:                 0.0,
			TacAmount:                                0.0,
			IofPercentage:                            8.2e-5,
			OverallIof:                               0.0038,
			PreDisbursementAmount:                    7800.0,
			PaidTotalIof:                             57.31,
			PaidContractAmount:                       7857.31,
		},
		{
			Installment:                              3,
			DueDate:                                  time.Date(2025, 07, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			DisbursementDate:                         disbursementDate,
			AccumulatedDays:                          87,
			DaysIndex:                                0.936823882407599,
			AccumulatedDaysIndex:                     2.8770350919608187,
			InterestRate:                             0.0235,
			InstallmentAmount:                        2734.44,
			InstallmentAmountWithoutTac:              0.0,
			TotalAmount:                              8203.32,
			DebitService:                             336.2299999999997,
			CustomerDebitServiceAmount:               336.2299999999997,
			CustomerAmount:                           2734.44,
			CalculationBasisForEffectiveInterestRate: 2712.0766666666664,
			MerchantDebitServiceAmount:               0.0,
			MerchantTotalAmount:                      390.0,
			SettledToMerchant:                        7410.0,
			MdrAmount:                                390.0,
			EffectiveInterestRate:                    0.0225,
			TotalEffectiveCost:                       0.0272,
			EirYearly:                                0.306592,
			TecYearly:                                0.380434,
			EirMonthly:                               0.0225,
			TecMonthly:                               0.0272,
			TotalIof:                                 67.09,
			ContractAmount:                           7867.09,
			ContractAmountWithoutTac:                 0.0,
			TacAmount:                                0.0,
			IofPercentage:                            8.2e-5,
			OverallIof:                               0.0038,
			PreDisbursementAmount:                    7799.99,
			PaidTotalIof:                             67.08,
			PaidContractAmount:                       7867.08,
		},
		{
			Installment:                              4,
			DueDate:                                  time.Date(2025, 8, 4, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			DisbursementDate:                         disbursementDate,
			AccumulatedDays:                          119,
			DaysIndex:                                0.914302133077605,
			AccumulatedDaysIndex:                     3.791337225038424,
			InterestRate:                             0.0235,
			InstallmentAmount:                        2077.73,
			InstallmentAmountWithoutTac:              0.0,
			TotalAmount:                              8310.92,
			DebitService:                             433.56000000000006,
			CustomerDebitServiceAmount:               433.56000000000006,
			CustomerAmount:                           2077.73,
			CalculationBasisForEffectiveInterestRate: 2058.39,
			MerchantDebitServiceAmount:               0.0,
			MerchantTotalAmount:                      390.0,
			SettledToMerchant:                        7410.0,
			MdrAmount:                                390.0,
			EffectiveInterestRate:                    0.0228,
			TotalEffectiveCost:                       0.0271,
			EirYearly:                                0.310455,
			TecYearly:                                0.377876,
			EirMonthly:                               0.0228,
			TecMonthly:                               0.0271,
			TotalIof:                                 77.36,
			ContractAmount:                           7877.36,
			ContractAmountWithoutTac:                 0.0,
			TacAmount:                                0.0,
			IofPercentage:                            8.2e-5,
			OverallIof:                               0.0038,
			PreDisbursementAmount:                    7800.02,
			PaidTotalIof:                             77.38,
			PaidContractAmount:                       7877.38,
		},
	}

	params := payment_plan.Params{
		RequestedAmount:                7800,
		FirstPaymentDate:               time.Date(2025, 05, 3, 0, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		RequestedDate:                  time.Date(2025, 04, 5, 0, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		Installments:                   4,
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

	resp, err := payment_plan.CalculatePaymentPlan(params)

	if err != nil {
		t.Fatalf("Error calculating payment plan: %v", err)
	}

	for i, r := range resp {
		e := expected[i]

		if r.Installment != e.Installment {
			t.Errorf("Installment %d: Expected Installment %d, got %d", i+1, e.Installment, r.Installment)
		}
		if !r.DueDate.Equal(e.DueDate) {
			t.Errorf("Installment %d: Expected DueDate %v, got %v", i+1, e.DueDate, r.DueDate)
		}
		if !r.DisbursementDate.Equal(e.DisbursementDate) {
			t.Errorf("Installment %d: Expected DisbursementDate %v, got %v", i+1, e.DisbursementDate, r.DisbursementDate)
		}
		if r.AccumulatedDays != e.AccumulatedDays {
			t.Errorf("Installment %d: Expected AccumulatedDays %d, got %d", i+1, e.AccumulatedDays, r.AccumulatedDays)
		}
		if r.DaysIndex != e.DaysIndex {
			t.Errorf("Installment %d: Expected DaysIndex %f, got %f", i+1, e.DaysIndex, r.DaysIndex)
		}
		if r.AccumulatedDaysIndex != e.AccumulatedDaysIndex {
			t.Errorf("Installment %d: Expected AccumulatedDaysIndex %f, got %f", i+1, e.AccumulatedDaysIndex, r.AccumulatedDaysIndex)
		}
		if r.InterestRate != e.InterestRate {
			t.Errorf("Installment %d: Expected InterestRate %f, got %f", i+1, e.InterestRate, r.InterestRate)
		}
		if r.InstallmentAmount != e.InstallmentAmount {
			t.Errorf("Installment %d: Expected InstallmentAmount %f, got %f", i+1, e.InstallmentAmount, r.InstallmentAmount)
		}
		if r.InstallmentAmountWithoutTac != e.InstallmentAmountWithoutTac {
			t.Errorf("Installment %d: Expected InstallmentAmountWithoutTac %f, got %f", i+1, e.InstallmentAmountWithoutTac, r.InstallmentAmountWithoutTac)
		}
		if r.TotalAmount != e.TotalAmount {
			t.Errorf("Installment %d: Expected TotalAmount %f, got %f", i+1, e.TotalAmount, r.TotalAmount)
		}
		if r.DebitService != e.DebitService {
			t.Errorf("Installment %d: Expected DebitService %f, got %f", i+1, e.DebitService, r.DebitService)
		}
		if r.CustomerDebitServiceAmount != e.CustomerDebitServiceAmount {
			t.Errorf("Installment %d: Expected CustomerDebitServiceAmount %f, got %f", i+1, e.CustomerDebitServiceAmount, r.CustomerDebitServiceAmount)
		}
		if r.CustomerAmount != e.CustomerAmount {
			t.Errorf("Installment %d: Expected CustomerAmount %f, got %f", i+1, e.CustomerAmount, r.CustomerAmount)
		}
		if r.CalculationBasisForEffectiveInterestRate != e.CalculationBasisForEffectiveInterestRate {
			t.Errorf("Installment %d: Expected CalculationBasisForEffectiveInterestRate %f, got %f", i+1, e.CalculationBasisForEffectiveInterestRate, r.CalculationBasisForEffectiveInterestRate)
		}
		if r.MerchantDebitServiceAmount != e.MerchantDebitServiceAmount {
			t.Errorf("Installment %d: Expected MerchantDebitServiceAmount %f, got %f", i+1, e.MerchantDebitServiceAmount, r.MerchantDebitServiceAmount)
		}
		if r.MerchantTotalAmount != e.MerchantTotalAmount {
			t.Errorf("Installment %d: Expected MerchantTotalAmount %f, got %f", i+1, e.MerchantTotalAmount, r.MerchantTotalAmount)
		}
		if r.SettledToMerchant != e.SettledToMerchant {
			t.Errorf("Installment %d: Expected SettledToMerchant %f, got %f", i+1, e.SettledToMerchant, r.SettledToMerchant)
		}
		if r.MdrAmount != e.MdrAmount {
			t.Errorf("Installment %d: Expected MdrAmount %f, got %f", i+1, e.MdrAmount, r.MdrAmount)
		}
		if r.EffectiveInterestRate != e.EffectiveInterestRate {
			t.Errorf("Installment %d: Expected EffectiveInterestRate %f, got %f", i+1, e.EffectiveInterestRate, r.EffectiveInterestRate)
		}
		if r.TotalEffectiveCost != e.TotalEffectiveCost {
			t.Errorf("Installment %d: Expected TotalEffectiveCost %f, got %f", i+1, e.TotalEffectiveCost, r.TotalEffectiveCost)
		}
		if r.EirYearly != e.EirYearly {
			t.Errorf("Installment %d: Expected EirYearly %f, got %f", i+1, e.EirYearly, r.EirYearly)
		}
		if r.TecYearly != e.TecYearly {
			t.Errorf("Installment %d: Expected TecYearly %f, got %f", i+1, e.TecYearly, r.TecYearly)
		}
		if r.EirMonthly != e.EirMonthly {
			t.Errorf("Installment %d: Expected EirMonthly %f, got %f", i+1, e.EirMonthly, r.EirMonthly)
		}
		if r.TecMonthly != e.TecMonthly {
			t.Errorf("Installment %d: Expected TecMonthly %f, got %f", i+1, e.TecMonthly, r.TecMonthly)
		}
		if r.TotalIof != e.TotalIof {
			t.Errorf("Installment %d: Expected TotalIof %f, got %f", i+1, e.TotalIof, r.TotalIof)
		}
		if r.ContractAmount != e.ContractAmount {
			t.Errorf("Installment %d: Expected ContractAmount %f, got %f", i+1, e.ContractAmount, r.ContractAmount)
		}
		if r.ContractAmountWithoutTac != e.ContractAmountWithoutTac {
			t.Errorf("Installment %d: Expected ContractAmountWithoutTac %f, got %f", i+1, e.ContractAmountWithoutTac, r.ContractAmountWithoutTac)
		}
		if r.TacAmount != e.TacAmount {
			t.Errorf("Installment %d: Expected TacAmount %f, got %f", i+1, e.TacAmount, r.TacAmount)
		}
		if r.IofPercentage != e.IofPercentage {
			t.Errorf("Installment %d: Expected IofPercentage %f, got %f", i+1, e.IofPercentage, r.IofPercentage)
		}
		if r.OverallIof != e.OverallIof {
			t.Errorf("Installment %d: Expected OverallIof %f, got %f", i+1, e.OverallIof, r.OverallIof)
		}
		if r.PreDisbursementAmount != e.PreDisbursementAmount {
			t.Errorf("Installment %d: Expected PreDisbursementAmount %f, got %f", i+1, e.PreDisbursementAmount, r.PreDisbursementAmount)
		}
		if r.PaidTotalIof != e.PaidTotalIof {
			t.Errorf("Installment %d: Expected PaidTotalIof %f, got %f", i+1, e.PaidTotalIof, r.PaidTotalIof)
		}
		if r.PaidContractAmount != e.PaidContractAmount {
			t.Errorf("Installment %d: Expected PaidContractAmount %f, got %f", i+1, e.PaidContractAmount, r.PaidContractAmount)
		}
	}

}
