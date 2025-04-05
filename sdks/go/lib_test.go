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
		helperAssert(r, e, i, t)

	}

}

func TestCalculateDownPaymentPlan(t *testing.T) {
	expected := []payment_plan.DownPaymentResponse{
		{
			InstallmentAmount:   1000,
			TotalAmount:         1000,
			InstallmentQuantity: 1,
			FirstPaymentDate:    time.Date(2025, 5, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			Plans: []payment_plan.Response{
				{
					Installment:                              1,
					DueDate:                                  time.Date(2025, 6, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 5, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          25,
					DaysIndex:                                0.981371965896169,
					AccumulatedDaysIndex:                     0.981371965896169,
					InterestRate:                             0.0235,
					InstallmentAmount:                        7994.82,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              7994.82,
					DebitService:                             148.92999999999972,
					CustomerDebitServiceAmount:               148.92999999999972,
					CustomerAmount:                           7994.82,
					CalculationBasisForEffectiveInterestRate: 7948.929999999999,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0231,
					TotalEffectiveCost:                       0.0305,
					EirYearly:                                0.315926,
					TecYearly:                                0.433592,
					EirMonthly:                               0.0231,
					TecMonthly:                               0.0305,
					TotalIof:                                 45.89,
					ContractAmount:                           7845.89,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             45.89,
					PaidContractAmount:                       7845.89,
				},
				{
					Installment:                              2,
					DueDate:                                  time.Date(2025, 7, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 5, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          55,
					DaysIndex:                                0.958839243657051,
					AccumulatedDaysIndex:                     1.94021120955322,
					InterestRate:                             0.0235,
					InstallmentAmount:                        4048.88,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8097.76,
					DebitService:                             242.07000000000022,
					CustomerDebitServiceAmount:               242.07000000000022,
					CustomerAmount:                           4048.88,
					CalculationBasisForEffectiveInterestRate: 4021.0350000000003,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0234,
					TotalEffectiveCost:                       0.029,
					EirYearly:                                0.319877,
					TecYearly:                                0.408833,
					EirMonthly:                               0.0234,
					TecMonthly:                               0.029,
					TotalIof:                                 55.69,
					ContractAmount:                           7855.69,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7799.99,
					PaidTotalIof:                             55.68,
					PaidContractAmount:                       7855.68,
				},
				{
					Installment:                              3,
					DueDate:                                  time.Date(2025, 8, 4, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 5, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          87,
					DaysIndex:                                0.935788233217493,
					AccumulatedDaysIndex:                     2.875999442770713,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2735.05,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8205.15,
					DebitService:                             339.13999999999965,
					CustomerDebitServiceAmount:               339.13999999999965,
					CustomerAmount:                           2735.05,
					CalculationBasisForEffectiveInterestRate: 2713.0466666666666,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0234,
					TotalEffectiveCost:                       0.0282,
					EirYearly:                                0.320481,
					TecYearly:                                0.396244,
					EirMonthly:                               0.0234,
					TecMonthly:                               0.0282,
					TotalIof:                                 66.01,
					ContractAmount:                           7866.01,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7799.99,
					PaidTotalIof:                             66,
					PaidContractAmount:                       7866,
				},
				{
					Installment:                              4,
					DueDate:                                  time.Date(2025, 9, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 5, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          117,
					DaysIndex:                                0.913291381450307,
					AccumulatedDaysIndex:                     3.78929082422102,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2078.54,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8314.16,
					DebitService:                             437.9499999999999,
					CustomerDebitServiceAmount:               437.9499999999999,
					CustomerAmount:                           2078.54,
					CalculationBasisForEffectiveInterestRate: 2059.4875,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0236,
					TotalEffectiveCost:                       0.0279,
					EirYearly:                                0.323112,
					TecYearly:                                0.391907,
					EirMonthly:                               0.0236,
					TecMonthly:                               0.0279,
					TotalIof:                                 76.21,
					ContractAmount:                           7876.21,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7799.98,
					PaidTotalIof:                             76.19,
					PaidContractAmount:                       7876.19,
				},
			},
		},
		{
			InstallmentAmount:   500,
			TotalAmount:         1000,
			InstallmentQuantity: 2,
			FirstPaymentDate:    time.Date(2025, 5, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			Plans: []payment_plan.Response{
				{
					Installment:                              1,
					DueDate:                                  time.Date(2025, 7, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 6, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          24,
					DaysIndex:                                0.981371965896169,
					AccumulatedDaysIndex:                     0.981371965896169,
					InterestRate:                             0.0235,
					InstallmentAmount:                        7994.17,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              7994.17,
					DebitService:                             148.92000000000007,
					CustomerDebitServiceAmount:               148.92000000000007,
					CustomerAmount:                           7994.17,
					CalculationBasisForEffectiveInterestRate: 7948.92,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0241,
					TotalEffectiveCost:                       0.0317,
					EirYearly:                                0.331065,
					TecYearly:                                0.453471,
					EirMonthly:                               0.0241,
					TecMonthly:                               0.0317,
					TotalIof:                                 45.25,
					ContractAmount:                           7845.25,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             45.25,
					PaidContractAmount:                       7845.25,
				},
				{
					Installment:                              2,
					DueDate:                                  time.Date(2025, 8, 4, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 6, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          56,
					DaysIndex:                                0.957779256710963,
					AccumulatedDaysIndex:                     1.9391512226071321,
					InterestRate:                             0.0235,
					InstallmentAmount:                        4051.09,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8102.18,
					DebitService:                             246.50000000000028,
					CustomerDebitServiceAmount:               246.50000000000028,
					CustomerAmount:                           4051.09,
					CalculationBasisForEffectiveInterestRate: 4023.25,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0238,
					TotalEffectiveCost:                       0.0294,
					EirYearly:                                0.326624,
					TecYearly:                                0.416087,
					EirMonthly:                               0.0238,
					TecMonthly:                               0.0294,
					TotalIof:                                 55.68,
					ContractAmount:                           7855.68,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             55.68,
					PaidContractAmount:                       7855.68,
				},
				{
					Installment:                              3,
					DueDate:                                  time.Date(2025, 9, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 6, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          86,
					DaysIndex:                                0.93475372892694,
					AccumulatedDaysIndex:                     2.873904951534072,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2736.97,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8210.91,
					DebitService:                             345.11999999999983,
					CustomerDebitServiceAmount:               345.11999999999983,
					CustomerAmount:                           2736.97,
					CalculationBasisForEffectiveInterestRate: 2715.04,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.024,
					TotalEffectiveCost:                       0.0288,
					EirYearly:                                0.329156,
					TecYearly:                                0.405648,
					EirMonthly:                               0.024,
					TecMonthly:                               0.0288,
					TotalIof:                                 65.79,
					ContractAmount:                           7865.79,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             65.79,
					PaidContractAmount:                       7865.79,
				},
				{
					Installment:                              4,
					DueDate:                                  time.Date(2025, 10, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 6, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          116,
					DaysIndex:                                0.912281747198563,
					AccumulatedDaysIndex:                     3.786186698732635,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2080.16,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8320.64,
					DebitService:                             444.74999999999943,
					CustomerDebitServiceAmount:               444.74999999999943,
					CustomerAmount:                           2080.16,
					CalculationBasisForEffectiveInterestRate: 2061.1875,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0241,
					TotalEffectiveCost:                       0.0285,
					EirYearly:                                0.331464,
					TecYearly:                                0.400907,
					EirMonthly:                               0.0241,
					TecMonthly:                               0.0285,
					TotalIof:                                 75.89,
					ContractAmount:                           7875.89,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7799.98,
					PaidTotalIof:                             75.87,
					PaidContractAmount:                       7875.87,
				},
			},
		},
		{
			InstallmentAmount:   333.3333333333333,
			TotalAmount:         1000,
			InstallmentQuantity: 3,
			FirstPaymentDate:    time.Date(2025, 5, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			Plans: []payment_plan.Response{
				{
					Installment:                              1,
					DueDate:                                  time.Date(2025, 8, 4, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 7, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          26,
					DaysIndex:                                0.980287069256833,
					AccumulatedDaysIndex:                     0.980287069256833,
					InterestRate:                             0.0235,
					InstallmentAmount:                        8004.34,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8004.34,
					DebitService:                             157.79000000000013,
					CustomerDebitServiceAmount:               157.79000000000013,
					CustomerAmount:                           8004.34,
					CalculationBasisForEffectiveInterestRate: 7957.79,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0236,
					TotalEffectiveCost:                       0.0307,
					EirYearly:                                0.322466,
					TecYearly:                                0.437689,
					EirMonthly:                               0.0236,
					TecMonthly:                               0.0307,
					TotalIof:                                 46.55,
					ContractAmount:                           7846.55,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             46.55,
					PaidContractAmount:                       7846.55,
				},
				{
					Installment:                              2,
					DueDate:                                  time.Date(2025, 9, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 7, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          56,
					DaysIndex:                                0.956720441569568,
					AccumulatedDaysIndex:                     1.937007510826401,
					InterestRate:                             0.0235,
					InstallmentAmount:                        4055.92,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8111.84,
					DebitService:                             255.50000000000014,
					CustomerDebitServiceAmount:               255.50000000000014,
					CustomerAmount:                           4055.92,
					CalculationBasisForEffectiveInterestRate: 4027.75,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0241,
					TotalEffectiveCost:                       0.0296,
					EirYearly:                                0.330449,
					TecYearly:                                0.418932,
					EirMonthly:                               0.0241,
					TecMonthly:                               0.0296,
					TotalIof:                                 56.34,
					ContractAmount:                           7856.34,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800.01,
					PaidTotalIof:                             56.35,
					PaidContractAmount:                       7856.35,
				},
				{
					Installment:                              3,
					DueDate:                                  time.Date(2025, 10, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 7, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          86,
					DaysIndex:                                0.933720368270266,
					AccumulatedDaysIndex:                     2.870727879096667,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2740.16,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8220.48,
					DebitService:                             354.23999999999955,
					CustomerDebitServiceAmount:               354.23999999999955,
					CustomerAmount:                           2740.16,
					CalculationBasisForEffectiveInterestRate: 2718.08,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0243,
					TotalEffectiveCost:                       0.0291,
					EirYearly:                                0.334168,
					TecYearly:                                0.410516,
					EirMonthly:                               0.0243,
					TecMonthly:                               0.0291,
					TotalIof:                                 66.24,
					ContractAmount:                           7866.24,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800.01,
					PaidTotalIof:                             66.25,
					PaidContractAmount:                       7866.25,
				},
				{
					Installment:                              4,
					DueDate:                                  time.Date(2025, 11, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 7, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          117,
					DaysIndex:                                0.912281747198563,
					AccumulatedDaysIndex:                     3.78300962629523,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2082.05,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8328.2,
					DebitService:                             451.7800000000007,
					CustomerDebitServiceAmount:               451.7800000000007,
					CustomerAmount:                           2082.05,
					CalculationBasisForEffectiveInterestRate: 2062.945,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0243,
					TotalEffectiveCost:                       0.0286,
					EirYearly:                                0.33315,
					TecYearly:                                0.402404,
					EirMonthly:                               0.0243,
					TecMonthly:                               0.0286,
					TotalIof:                                 76.42,
					ContractAmount:                           7876.42,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             76.42,
					PaidContractAmount:                       7876.42,
				},
			},
		},
		{
			InstallmentAmount:   250,
			TotalAmount:         1000,
			InstallmentQuantity: 4,
			FirstPaymentDate:    time.Date(2025, 5, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
			Plans: []payment_plan.Response{
				{
					Installment:                              1,
					DueDate:                                  time.Date(2025, 9, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 8, 11, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          23,
					DaysIndex:                                0.981371965896169,
					AccumulatedDaysIndex:                     0.981371965896169,
					InterestRate:                             0.0235,
					InstallmentAmount:                        7993.5,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              7993.5,
					DebitService:                             148.9,
					CustomerDebitServiceAmount:               148.9,
					CustomerAmount:                           7993.5,
					CalculationBasisForEffectiveInterestRate: 7948.9,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0252,
					TotalEffectiveCost:                       0.0329,
					EirYearly:                                0.347719,
					TecYearly:                                0.475332,
					EirMonthly:                               0.0252,
					TecMonthly:                               0.0329,
					TotalIof:                                 44.6,
					ContractAmount:                           7844.6,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             44.6,
					PaidContractAmount:                       7844.6,
				},
				{
					Installment:                              2,
					DueDate:                                  time.Date(2025, 10, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 8, 11, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          53,
					DaysIndex:                                0.957779256710963,
					AccumulatedDaysIndex:                     1.9391512226071321,
					InterestRate:                             0.0235,
					InstallmentAmount:                        4050.43,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8100.86,
					DebitService:                             246.4699999999997,
					CustomerDebitServiceAmount:               246.4699999999997,
					CustomerAmount:                           4050.43,
					CalculationBasisForEffectiveInterestRate: 4023.2349999999997,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0251,
					TotalEffectiveCost:                       0.0308,
					EirYearly:                                0.34648,
					TecYearly:                                0.439943,
					EirMonthly:                               0.0251,
					TecMonthly:                               0.0308,
					TotalIof:                                 54.39,
					ContractAmount:                           7854.39,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800.01,
					PaidTotalIof:                             54.4,
					PaidContractAmount:                       7854.4,
				},
				{
					Installment:                              3,
					DueDate:                                  time.Date(2025, 11, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 8, 11, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          84,
					DaysIndex:                                0.935788233217493,
					AccumulatedDaysIndex:                     2.874939455824625,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2735.54,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8206.62,
					DebitService:                             342.1200000000008,
					CustomerDebitServiceAmount:               342.1200000000008,
					CustomerAmount:                           2735.54,
					CalculationBasisForEffectiveInterestRate: 2714.0400000000004,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0247,
					TotalEffectiveCost:                       0.0296,
					EirYearly:                                0.340141,
					TecYearly:                                0.418684,
					EirMonthly:                               0.0247,
					TecMonthly:                               0.0296,
					TotalIof:                                 64.5,
					ContractAmount:                           7864.5,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800.01,
					PaidTotalIof:                             64.51,
					PaidContractAmount:                       7864.51,
				},
				{
					Installment:                              4,
					DueDate:                                  time.Date(2025, 12, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					DisbursementDate:                         time.Date(2025, 8, 11, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
					AccumulatedDays:                          114,
					DaysIndex:                                0.914302133077605,
					AccumulatedDaysIndex:                     3.78924158890223,
					InterestRate:                             0.0235,
					InstallmentAmount:                        2078.15,
					InstallmentAmountWithoutTac:              0,
					TotalAmount:                              8312.6,
					DebitService:                             437.99000000000035,
					CustomerDebitServiceAmount:               437.99000000000035,
					CustomerAmount:                           2078.15,
					CalculationBasisForEffectiveInterestRate: 2059.4975,
					MerchantDebitServiceAmount:               0,
					MerchantTotalAmount:                      390,
					SettledToMerchant:                        7410,
					MdrAmount:                                390,
					EffectiveInterestRate:                    0.0245,
					TotalEffectiveCost:                       0.0289,
					EirYearly:                                0.336923,
					TecYearly:                                0.407548,
					EirMonthly:                               0.0245,
					TecMonthly:                               0.0289,
					TotalIof:                                 74.61,
					ContractAmount:                           7874.61,
					ContractAmountWithoutTac:                 0,
					TacAmount:                                0,
					IofPercentage:                            0.000082,
					OverallIof:                               0.0038,
					PreDisbursementAmount:                    7800,
					PaidTotalIof:                             74.61,
					PaidContractAmount:                       7874.61,
				},
			},
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

	downPaymentParams := payment_plan.DownPaymentParams{
		Params:               params,
		RequestedAmount:      1000,
		MinInstallmentAmount: 100,
		FirstPaymentDate:     time.Date(2025, 05, 3, 0, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		Installments:         4,
	}

	resp, err := payment_plan.CalculateDownPaymentPlan(downPaymentParams)
	if err != nil {
		t.Fatalf("Error calculating payment plan: %v", err)
	}

	for i, dPlan := range resp {
		expectedPlan := expected[i]
		if !dPlan.FirstPaymentDate.Equal(expectedPlan.FirstPaymentDate) {
			t.Errorf("Expected FirstPaymentDate %v, got %v", expectedPlan.FirstPaymentDate, dPlan.FirstPaymentDate)
		}

		if dPlan.InstallmentAmount != expectedPlan.InstallmentAmount {
			t.Errorf("Expected InstallmentAmount %v, got %v", expectedPlan.InstallmentAmount, dPlan.InstallmentAmount)
		}

		if dPlan.TotalAmount != expectedPlan.TotalAmount {
			t.Errorf("Expected TotalAmount %v, got %v", expectedPlan.TotalAmount, dPlan.TotalAmount)
		}

		if dPlan.InstallmentQuantity != expectedPlan.InstallmentQuantity {
			t.Errorf("Expected InstallmentQuantity %v, got %v", expectedPlan.InstallmentQuantity, dPlan.InstallmentQuantity)
		}

		for j, plan := range dPlan.Plans {
			helperAssert(plan, expectedPlan.Plans[j], j, t)
		}
	}

}

func helperAssert(r payment_plan.Response, e payment_plan.Response, i int, t *testing.T) {
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

func TestDisbursementDateRange(t *testing.T) {
	// Mock base date and number of days
	baseDate := time.Date(2025, 4, 3, 0, 0, 0, 0, time.UTC)
	days := uint32(5)

	// Call the function
	start, end := payment_plan.DisbursementDateRange(baseDate, days)

	// Expected results
	expectedStart := time.Date(2025, 4, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60))
	expectedEnd := time.Date(2025, 4, 9, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60))

	// Validate the results
	if !start.Equal(expectedStart) {
		t.Errorf("Expected start date %v, got %v", expectedStart, start)
	}

	if !end.Equal(expectedEnd) {
		t.Errorf("Expected end date %v, got %v", expectedEnd, end)
	}
}

func TestNextDisbursementDate(t *testing.T) {
	// Mock base date
	baseDate := time.Date(2025, 4, 3, 0, 0, 0, 0, time.UTC)

	// Call the function
	nextDate := payment_plan.NextDisbursementDate(baseDate)

	// Expected result (assuming 2025-04-03 is a business day and not the same as the system date)
	expectedDate := time.Date(2025, 4, 3, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60))

	// Validate the result
	if !nextDate.Equal(expectedDate) {
		t.Errorf("Expected next disbursement date %v, got %v", expectedDate, nextDate)
	}
}

func TestNextDisbursementDate_NotToday(t *testing.T) {
	// Use today's date as the base date
	today := time.Now()

	// Call the function
	nextDate := payment_plan.NextDisbursementDate(today)

	// Validate that the result is not the same as today
	if nextDate.Year() == today.Year() && nextDate.Month() == today.Month() && nextDate.Day() == today.Day() {
		t.Errorf("Next disbursement date should not be the same as today. Got: %v", nextDate)
	}
}

func TestGetNonBusinessDaysBetween(t *testing.T) {
	// Define start and end dates
	startDate := time.Date(2025, 4, 1, 0, 0, 0, 0, time.UTC)
	endDate := time.Date(2025, 4, 30, 0, 0, 0, 0, time.UTC)

	expectedNonBusinessDays := []time.Time{
		time.Date(2025, 4, 5, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 6, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 12, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 13, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 18, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 19, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 20, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 21, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 26, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
		time.Date(2025, 4, 27, 7, 0, 0, 0, time.FixedZone("-03", -3*60*60)),
	}

	// Call the function
	nonBusinessDays := payment_plan.GetNonBusinessDaysBetween(startDate, endDate)

	// Validate the result
	if len(nonBusinessDays) == 0 {
		t.Errorf("Expected non-business days between %v and %v, but got none", startDate, endDate)
	}

	// Check if all returned dates are within the range and are non-business days
	// Check if all expected dates are in the result
	for _, expectedDate := range expectedNonBusinessDays {
		found := false
		for _, actualDate := range nonBusinessDays {
			if actualDate.Equal(expectedDate) {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("Expected non-business day %v not found in the result", expectedDate)
		}
	}
}
