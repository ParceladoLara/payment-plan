import unittest
from datetime import datetime, timedelta, timezone
from payment_plan import Params, Response, calculate_payment_plan


class TestCalculatePaymentPlan(unittest.TestCase):
    def test_calculate_payment_plan(self):
        disbursement_date = datetime(2025, 4, 7, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3)))

        expected = [
          Response(
              installment=1,
              due_date=datetime(2025, 5, 5, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
              disbursement_date=disbursement_date,
              accumulated_days=28,
              days_index=0.981371965896169,
              accumulated_days_index=0.981371965896169,
              interest_rate=0.0235,
              installment_amount=7996.8,
              installment_amount_without_tac=0.0,
              total_amount=7996.8,
              debit_service=148.96000000000018,
              customer_debit_service_amount=148.96000000000018,
              customer_amount=7996.8,
              calculation_basis_for_effective_interest_rate=7948.96,
              merchant_debit_service_amount=0.0,
              merchant_total_amount=390.0,
              settled_to_merchant=7410.0,
              mdr_amount=390.0,
              effective_interest_rate=0.0206,
              total_effective_cost=0.0274,
              eir_yearly=0.277782,
              tec_yearly=0.383782,
              eir_monthly=0.0206,
              tec_monthly=0.0274,
              total_iof=47.84,
              contract_amount=7847.84,
              contract_amount_without_tac=0.0,
              tac_amount=0.0,
              iof_percentage=8.2e-5,
              overall_iof=0.0038,
              pre_disbursement_amount=7800.0,
              paid_total_iof=47.84,
              paid_contract_amount=7847.84,
          ),
          Response(
              installment=2,
              due_date=datetime(2025, 6, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
              disbursement_date=disbursement_date,
              accumulated_days=57,
              days_index=0.958839243657051,
              accumulated_days_index=1.94021120955322,
              interest_rate=0.0235,
              installment_amount=4049.72,
              installment_amount_without_tac=0.0,
              total_amount=8099.44,
              debit_service=242.1299999999996,
              customer_debit_service_amount=242.1299999999996,
              customer_amount=4049.72,
              calculation_basis_for_effective_interest_rate=4021.0649999999996,
              merchant_debit_service_amount=0.0,
              merchant_total_amount=390.0,
              settled_to_merchant=7410.0,
              mdr_amount=390.0,
              effective_interest_rate=0.022,
              total_effective_cost=0.0274,
              eir_yearly=0.298378,
              tec_yearly=0.382981,
              eir_monthly=0.022,
              tec_monthly=0.0274,
              total_iof=57.31,
              contract_amount=7857.31,
              contract_amount_without_tac=0.0,
              tac_amount=0.0,
              iof_percentage=8.2e-5,
              overall_iof=0.0038,
              pre_disbursement_amount=7800.0,
              paid_total_iof=57.31,
              paid_contract_amount=7857.31,
          ),
          Response(
              installment=3,
              due_date=datetime(2025, 7, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
              disbursement_date=disbursement_date,
              accumulated_days=87,
              days_index=0.936823882407599,
              accumulated_days_index=2.8770350919608187,
              interest_rate=0.0235,
              installment_amount=2734.44,
              installment_amount_without_tac=0.0,
              total_amount=8203.32,
              debit_service=336.2299999999997,
              customer_debit_service_amount=336.2299999999997,
              customer_amount=2734.44,
              calculation_basis_for_effective_interest_rate=2712.0766666666664,
              merchant_debit_service_amount=0.0,
              merchant_total_amount=390.0,
              settled_to_merchant=7410.0,
              mdr_amount=390.0,
              effective_interest_rate=0.0225,
              total_effective_cost=0.0272,
              eir_yearly=0.306592,
              tec_yearly=0.380434,
              eir_monthly=0.0225,
              tec_monthly=0.0272,
              total_iof=67.09,
              contract_amount=7867.09,
              contract_amount_without_tac=0.0,
              tac_amount=0.0,
              iof_percentage=8.2e-5,
              overall_iof=0.0038,
              pre_disbursement_amount=7799.99,
              paid_total_iof=67.08,
              paid_contract_amount=7867.08,
          ),
          Response(
              installment=4,
              due_date=datetime(2025, 8, 4, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
              disbursement_date=disbursement_date,
              accumulated_days=119,
              days_index=0.914302133077605,
              accumulated_days_index=3.791337225038424,
              interest_rate=0.0235,
              installment_amount=2077.73,
              installment_amount_without_tac=0.0,
              total_amount=8310.92,
              debit_service=433.56000000000006,
              customer_debit_service_amount=433.56000000000006,
              customer_amount=2077.73,
              calculation_basis_for_effective_interest_rate=2058.39,
              merchant_debit_service_amount=0.0,
              merchant_total_amount=390.0,
              settled_to_merchant=7410.0,
              mdr_amount=390.0,
              effective_interest_rate=0.0228,
              total_effective_cost=0.0271,
              eir_yearly=0.310455,
              tec_yearly=0.377876,
              eir_monthly=0.0228,
              tec_monthly=0.0271,
              total_iof=77.36,
              contract_amount=7877.36,
              contract_amount_without_tac=0.0,
              tac_amount=0.0,
              iof_percentage=8.2e-5,
              overall_iof=0.0038,
              pre_disbursement_amount=7800.02,
              paid_total_iof=77.38,
              paid_contract_amount=7877.38,
          ),
      ]

        params = Params(
            requested_amount=7800,
            first_payment_date=datetime(2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))),
            requested_date=datetime(2025, 4, 5, tzinfo=timezone(timedelta(hours=-3))),
            installments=4,
            debit_service_percentage=0,
            mdr=0.05,
            tac_percentage=0,
            iof_overall=0.0038,
            iof_percentage=0.000082,
            interest_rate=0.0235,
            min_installment_amount=100,
            max_total_amount=1000000,
            disbursement_only_on_business_days=True,
        )

        resp = calculate_payment_plan(params)

        self.assertEqual(len(resp), len(expected), "Response length does not match expected length")

        for i, (r, e) in enumerate(zip(resp, expected)):
            self.assertEqual(r.installment, e.installment, f"Installment {i + 1}: Installment mismatch")
            self.assertEqual(r.due_date, e.due_date, f"Installment {i + 1}: DueDate mismatch")
            self.assertEqual(r.disbursement_date, e.disbursement_date, f"Installment {i + 1}: DisbursementDate mismatch")
            self.assertEqual(r.accumulated_days, e.accumulated_days, f"Installment {i + 1}: AccumulatedDays mismatch")
            self.assertAlmostEqual(r.days_index, e.days_index, places=6, msg=f"Installment {i + 1}: DaysIndex mismatch")
            self.assertAlmostEqual(
                r.accumulated_days_index, e.accumulated_days_index, places=6, msg=f"Installment {i + 1}: AccumulatedDaysIndex mismatch"
            )
            self.assertAlmostEqual(r.interest_rate, e.interest_rate, places=6, msg=f"Installment {i + 1}: InterestRate mismatch")
            self.assertAlmostEqual(r.installment_amount, e.installment_amount, places=2, msg=f"Installment {i + 1}: InstallmentAmount mismatch")
            self.assertAlmostEqual(
                r.installment_amount_without_tac, e.installment_amount_without_tac, places=2, msg=f"Installment {i + 1}: InstallmentAmountWithoutTac mismatch"
            )
            self.assertAlmostEqual(r.total_amount, e.total_amount, places=2, msg=f"Installment {i + 1}: TotalAmount mismatch")
            self.assertAlmostEqual(r.debit_service, e.debit_service, places=2, msg=f"Installment {i + 1}: DebitService mismatch")
            self.assertAlmostEqual(
                r.customer_debit_service_amount, e.customer_debit_service_amount, places=2, msg=f"Installment {i + 1}: CustomerDebitServiceAmount mismatch"
            )
            self.assertAlmostEqual(r.customer_amount, e.customer_amount, places=2, msg=f"Installment {i + 1}: CustomerAmount mismatch")
            self.assertAlmostEqual(
                r.calculation_basis_for_effective_interest_rate,
                e.calculation_basis_for_effective_interest_rate,
                places=2,
                msg=f"Installment {i + 1}: CalculationBasisForEffectiveInterestRate mismatch",
            )
            self.assertAlmostEqual(
                r.merchant_debit_service_amount, e.merchant_debit_service_amount, places=2, msg=f"Installment {i + 1}: MerchantDebitServiceAmount mismatch"
            )
            self.assertAlmostEqual(r.merchant_total_amount, e.merchant_total_amount, places=2, msg=f"Installment {i + 1}: MerchantTotalAmount mismatch")
            self.assertAlmostEqual(r.settled_to_merchant, e.settled_to_merchant, places=2, msg=f"Installment {i + 1}: SettledToMerchant mismatch")
            self.assertAlmostEqual(r.mdr_amount, e.mdr_amount, places=2, msg=f"Installment {i + 1}: MdrAmount mismatch")
            self.assertAlmostEqual(r.effective_interest_rate, e.effective_interest_rate, places=6, msg=f"Installment {i + 1}: EffectiveInterestRate mismatch")
            self.assertAlmostEqual(r.total_effective_cost, e.total_effective_cost, places=6, msg=f"Installment {i + 1}: TotalEffectiveCost mismatch")
            self.assertAlmostEqual(r.eir_yearly, e.eir_yearly, places=6, msg=f"Installment {i + 1}: EirYearly mismatch")
            self.assertAlmostEqual(r.tec_yearly, e.tec_yearly, places=6, msg=f"Installment {i + 1}: TecYearly mismatch")
            self.assertAlmostEqual(r.eir_monthly, e.eir_monthly, places=6, msg=f"Installment {i + 1}: EirMonthly mismatch")
            self.assertAlmostEqual(r.tec_monthly, e.tec_monthly, places=6, msg=f"Installment {i + 1}: TecMonthly mismatch")
            self.assertAlmostEqual(r.total_iof, e.total_iof, places=2, msg=f"Installment {i + 1}: TotalIof mismatch")
            self.assertAlmostEqual(r.contract_amount, e.contract_amount, places=2, msg=f"Installment {i + 1}: ContractAmount mismatch")
            self.assertAlmostEqual(
                r.contract_amount_without_tac, e.contract_amount_without_tac, places=2, msg=f"Installment {i + 1}: ContractAmountWithoutTac mismatch"
            )
            self.assertAlmostEqual(r.tac_amount, e.tac_amount, places=2, msg=f"Installment {i + 1}: TacAmount mismatch")
            self.assertAlmostEqual(r.iof_percentage, e.iof_percentage, places=6, msg=f"Installment {i + 1}: IofPercentage mismatch")
            self.assertAlmostEqual(r.overall_iof, e.overall_iof, places=6, msg=f"Installment {i + 1}: OverallIof mismatch")
            self.assertAlmostEqual(r.pre_disbursement_amount, e.pre_disbursement_amount, places=2, msg=f"Installment {i + 1}: PreDisbursementAmount mismatch")
            self.assertAlmostEqual(r.paid_total_iof, e.paid_total_iof, places=2, msg=f"Installment {i + 1}: PaidTotalIof mismatch")
            self.assertAlmostEqual(r.paid_contract_amount, e.paid_contract_amount, places=2, msg=f"Installment {i + 1}: PaidContractAmount mismatch")


if __name__ == "__main__":
    unittest.main()