import unittest
from datetime import datetime, timedelta, timezone
from payment_plan import (
    disbursement_date_range,
    next_disbursement_date,
    get_non_business_days_between,
    Response,
    Params,
    calculate_payment_plan,
    calculate_down_payment_plan,
    DownPaymentParams,
    DownPaymentResponse,
    Invoice,
)


class TestPaymentPlanUtilities(unittest.TestCase):
    def test_calculate_payment_plan(self):
        disbursement_date = datetime(
            2025, 4, 7, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
        )

        expected = [
            Response(
                installment=1,
                due_date=datetime(
                    2025, 5, 5, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                ),
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
                invoices=[
                    Invoice(
                        accumulated_days=28,
                        accumulated_factor=0.981371965896169,
                        factor=0.981371965896169,
                        due_date=datetime(
                            2025, 5, 5, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                ],
            ),
            Response(
                installment=2,
                due_date=datetime(
                    2025, 6, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                ),
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
                invoices=[
                    Invoice(
                        accumulated_days=28,
                        accumulated_factor=0.981371965896169,
                        factor=0.981371965896169,
                        due_date=datetime(
                            2025, 5, 5, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                    Invoice(
                        accumulated_days=57,
                        accumulated_factor=1.94021120955322,
                        factor=0.958839243657051,
                        due_date=datetime(
                            2025, 6, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                ],
            ),
            Response(
                installment=3,
                due_date=datetime(
                    2025, 7, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                ),
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
                invoices=[
                    Invoice(
                        accumulated_days=28,
                        accumulated_factor=0.981371965896169,
                        factor=0.981371965896169,
                        due_date=datetime(
                            2025, 5, 5, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                    Invoice(
                        accumulated_days=57,
                        accumulated_factor=1.94021120955322,
                        factor=0.958839243657051,
                        due_date=datetime(
                            2025, 6, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                    Invoice(
                        accumulated_days=87,
                        accumulated_factor=2.8770350919608187,
                        factor=0.936823882407599,
                        due_date=datetime(
                            2025, 7, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                ],
            ),
            Response(
                installment=4,
                due_date=datetime(
                    2025, 8, 4, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                ),
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
                invoices=[
                    Invoice(
                        accumulated_days=28,
                        accumulated_factor=0.981371965896169,
                        factor=0.981371965896169,
                        due_date=datetime(
                            2025, 5, 5, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                    Invoice(
                        accumulated_days=57,
                        accumulated_factor=1.94021120955322,
                        factor=0.958839243657051,
                        due_date=datetime(
                            2025, 6, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                    Invoice(
                        accumulated_days=87,
                        accumulated_factor=2.8770350919608187,
                        factor=0.936823882407599,
                        due_date=datetime(
                            2025, 7, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                    Invoice(
                        accumulated_days=119,
                        accumulated_factor=3.791337225038424,
                        factor=0.914302133077605,
                        due_date=datetime(
                            2025, 8, 4, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
                        ),
                    ),
                ],
            ),
        ]

        params = Params(
            requested_amount=7800,
            first_payment_date=datetime(
                2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))
            ),
            disbursement_date=datetime(
                2025, 4, 5, tzinfo=timezone(timedelta(hours=-3))
            ),
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

        self.assertEqual(
            len(resp), len(expected), "Response length does not match expected length"
        )

        for i, (r, e) in enumerate(zip(resp, expected)):
            self.assertEqual(
                r.installment,
                e.installment,
                f"Installment {i + 1}: Installment mismatch",
            )
            self.assertEqual(
                r.due_date, e.due_date, f"Installment {i + 1}: DueDate mismatch"
            )
            self.assertEqual(
                r.disbursement_date,
                e.disbursement_date,
                f"Installment {i + 1}: DisbursementDate mismatch",
            )
            self.assertEqual(
                r.accumulated_days,
                e.accumulated_days,
                f"Installment {i + 1}: AccumulatedDays mismatch",
            )
            self.assertAlmostEqual(
                r.days_index,
                e.days_index,
                places=6,
                msg=f"Installment {i + 1}: DaysIndex mismatch",
            )
            self.assertAlmostEqual(
                r.accumulated_days_index,
                e.accumulated_days_index,
                places=6,
                msg=f"Installment {i + 1}: AccumulatedDaysIndex mismatch",
            )
            self.assertAlmostEqual(
                r.interest_rate,
                e.interest_rate,
                places=6,
                msg=f"Installment {i + 1}: InterestRate mismatch",
            )
            self.assertAlmostEqual(
                r.installment_amount,
                e.installment_amount,
                places=2,
                msg=f"Installment {i + 1}: InstallmentAmount mismatch",
            )
            self.assertAlmostEqual(
                r.installment_amount_without_tac,
                e.installment_amount_without_tac,
                places=2,
                msg=f"Installment {i + 1}: InstallmentAmountWithoutTac mismatch",
            )
            self.assertAlmostEqual(
                r.total_amount,
                e.total_amount,
                places=2,
                msg=f"Installment {i + 1}: TotalAmount mismatch",
            )
            self.assertAlmostEqual(
                r.debit_service,
                e.debit_service,
                places=2,
                msg=f"Installment {i + 1}: DebitService mismatch",
            )
            self.assertAlmostEqual(
                r.customer_debit_service_amount,
                e.customer_debit_service_amount,
                places=2,
                msg=f"Installment {i + 1}: CustomerDebitServiceAmount mismatch",
            )
            self.assertAlmostEqual(
                r.customer_amount,
                e.customer_amount,
                places=2,
                msg=f"Installment {i + 1}: CustomerAmount mismatch",
            )
            self.assertAlmostEqual(
                r.calculation_basis_for_effective_interest_rate,
                e.calculation_basis_for_effective_interest_rate,
                places=2,
                msg=f"Installment {i + 1}: CalculationBasisForEffectiveInterestRate mismatch",
            )
            self.assertAlmostEqual(
                r.merchant_debit_service_amount,
                e.merchant_debit_service_amount,
                places=2,
                msg=f"Installment {i + 1}: MerchantDebitServiceAmount mismatch",
            )
            self.assertAlmostEqual(
                r.merchant_total_amount,
                e.merchant_total_amount,
                places=2,
                msg=f"Installment {i + 1}: MerchantTotalAmount mismatch",
            )
            self.assertAlmostEqual(
                r.settled_to_merchant,
                e.settled_to_merchant,
                places=2,
                msg=f"Installment {i + 1}: SettledToMerchant mismatch",
            )
            self.assertAlmostEqual(
                r.mdr_amount,
                e.mdr_amount,
                places=2,
                msg=f"Installment {i + 1}: MdrAmount mismatch",
            )
            self.assertAlmostEqual(
                r.effective_interest_rate,
                e.effective_interest_rate,
                places=6,
                msg=f"Installment {i + 1}: EffectiveInterestRate mismatch",
            )
            self.assertAlmostEqual(
                r.total_effective_cost,
                e.total_effective_cost,
                places=6,
                msg=f"Installment {i + 1}: TotalEffectiveCost mismatch",
            )
            self.assertAlmostEqual(
                r.eir_yearly,
                e.eir_yearly,
                places=6,
                msg=f"Installment {i + 1}: EirYearly mismatch",
            )
            self.assertAlmostEqual(
                r.tec_yearly,
                e.tec_yearly,
                places=6,
                msg=f"Installment {i + 1}: TecYearly mismatch",
            )
            self.assertAlmostEqual(
                r.eir_monthly,
                e.eir_monthly,
                places=6,
                msg=f"Installment {i + 1}: EirMonthly mismatch",
            )
            self.assertAlmostEqual(
                r.tec_monthly,
                e.tec_monthly,
                places=6,
                msg=f"Installment {i + 1}: TecMonthly mismatch",
            )
            self.assertAlmostEqual(
                r.total_iof,
                e.total_iof,
                places=2,
                msg=f"Installment {i + 1}: TotalIof mismatch",
            )
            self.assertAlmostEqual(
                r.contract_amount,
                e.contract_amount,
                places=2,
                msg=f"Installment {i + 1}: ContractAmount mismatch",
            )
            self.assertAlmostEqual(
                r.contract_amount_without_tac,
                e.contract_amount_without_tac,
                places=2,
                msg=f"Installment {i + 1}: ContractAmountWithoutTac mismatch",
            )
            self.assertAlmostEqual(
                r.tac_amount,
                e.tac_amount,
                places=2,
                msg=f"Installment {i + 1}: TacAmount mismatch",
            )
            self.assertAlmostEqual(
                r.iof_percentage,
                e.iof_percentage,
                places=6,
                msg=f"Installment {i + 1}: IofPercentage mismatch",
            )
            self.assertAlmostEqual(
                r.overall_iof,
                e.overall_iof,
                places=6,
                msg=f"Installment {i + 1}: OverallIof mismatch",
            )
            self.assertAlmostEqual(
                r.pre_disbursement_amount,
                e.pre_disbursement_amount,
                places=2,
                msg=f"Installment {i + 1}: PreDisbursementAmount mismatch",
            )
            self.assertAlmostEqual(
                r.paid_total_iof,
                e.paid_total_iof,
                places=2,
                msg=f"Installment {i + 1}: PaidTotalIof mismatch",
            )
            self.assertAlmostEqual(
                r.paid_contract_amount,
                e.paid_contract_amount,
                places=2,
                msg=f"Installment {i + 1}: PaidContractAmount mismatch",
            )
            for j, (inv_r, inv_e) in enumerate(zip(r.invoices, e.invoices)):
                self.assertEqual(
                    inv_r.accumulated_days,
                    inv_e.accumulated_days,
                    f"Installment {i + 1}, Invoice {j + 1}: AccumulatedDays mismatch",
                )
                self.assertAlmostEqual(
                    inv_r.accumulated_factor,
                    inv_e.accumulated_factor,
                    places=6,
                    msg=f"Installment {i + 1}, Invoice {j + 1}: AccumulatedFactor mismatch",
                )
                self.assertAlmostEqual(
                    inv_r.factor,
                    inv_e.factor,
                    places=6,
                    msg=f"Installment {i + 1}, Invoice {j + 1}: Factor mismatch",
                )
                self.assertEqual(
                    inv_r.due_date,
                    inv_e.due_date,
                    f"Installment {i + 1}, Invoice {j + 1}: DueDate mismatch",
                )

    def test_disbursement_date_range(self):
        base_date = datetime(2025, 4, 3, tzinfo=timezone.utc)
        days = 5

        start, end = disbursement_date_range(base_date, days)

        expected_start = datetime(
            2025, 4, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
        )
        expected_end = datetime(
            2025, 4, 9, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
        )

        self.assertEqual(
            start, expected_start, f"Expected start date {expected_start}, got {start}"
        )
        self.assertEqual(
            end, expected_end, f"Expected end date {expected_end}, got {end}"
        )

    def test_next_disbursement_date(self):
        base_date = datetime(2025, 4, 3, tzinfo=timezone.utc)

        next_date = next_disbursement_date(base_date)

        expected_date = datetime(
            2025, 4, 3, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))
        )

        self.assertEqual(
            next_date,
            expected_date,
            f"Expected next disbursement date {expected_date}, got {next_date}",
        )

    def test_next_disbursement_date_not_today(self):
        today = datetime.now(timezone.utc)

        next_date = next_disbursement_date(today)

        self.assertNotEqual(
            (next_date.year, next_date.month, next_date.day),
            (today.year, today.month, today.day),
            f"Next disbursement date should not be the same as today. Got: {next_date}",
        )

    def test_get_non_business_days_between(self):
        start_date = datetime(2025, 4, 1, tzinfo=timezone.utc)
        end_date = datetime(2025, 4, 30, tzinfo=timezone.utc)

        expected_non_business_days = [
            datetime(2025, 4, 5, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 6, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 12, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 13, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 18, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 19, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 20, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 21, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 26, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
            datetime(2025, 4, 27, 7, 0, 0, tzinfo=timezone(timedelta(hours=-3))),
        ]

        non_business_days = get_non_business_days_between(start_date, end_date)

        self.assertGreater(
            len(non_business_days),
            0,
            f"Expected non-business days between {start_date} and {end_date}, but got none",
        )

        for expected_date in expected_non_business_days:
            self.assertIn(
                expected_date,
                non_business_days,
                f"Expected non-business day {expected_date} not found in the result",
            )

    def test_calculate_down_payment_plan(self):

        expected = [
            DownPaymentResponse(
                installment_amount=1000,
                total_amount=1000,
                installment_quantity=1,
                first_payment_date=datetime(
                    2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))
                ),
                plans=[
                    Response(
                        installment=1,
                        due_date=datetime(
                            2025, 6, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 5, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=25,
                        days_index=0.981371965896169,
                        accumulated_days_index=0.981371965896169,
                        interest_rate=0.0235,
                        installment_amount=7994.82,
                        installment_amount_without_tac=0,
                        total_amount=7994.82,
                        debit_service=148.92999999999972,
                        customer_debit_service_amount=148.92999999999972,
                        customer_amount=7994.82,
                        calculation_basis_for_effective_interest_rate=7948.929999999999,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0231,
                        total_effective_cost=0.0305,
                        eir_yearly=0.315926,
                        tec_yearly=0.433592,
                        eir_monthly=0.0231,
                        tec_monthly=0.0305,
                        total_iof=45.89,
                        contract_amount=7845.89,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=45.89,
                        paid_contract_amount=7845.89,
                        invoices=[
                            Invoice(
                                accumulated_days=25,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    6,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=2,
                        due_date=datetime(
                            2025, 7, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 5, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=55,
                        days_index=0.958839243657051,
                        accumulated_days_index=1.94021120955322,
                        interest_rate=0.0235,
                        installment_amount=4048.88,
                        installment_amount_without_tac=0,
                        total_amount=8097.76,
                        debit_service=242.07000000000022,
                        customer_debit_service_amount=242.07000000000022,
                        customer_amount=4048.88,
                        calculation_basis_for_effective_interest_rate=4021.0350000000003,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0234,
                        total_effective_cost=0.029,
                        eir_yearly=0.319877,
                        tec_yearly=0.408833,
                        eir_monthly=0.0234,
                        tec_monthly=0.029,
                        total_iof=55.69,
                        contract_amount=7855.69,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7799.99,
                        paid_total_iof=55.68,
                        paid_contract_amount=7855.68,
                        invoices=[
                            Invoice(
                                accumulated_days=25,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    6,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=55,
                                accumulated_factor=1.94021120955322,
                                factor=0.958839243657051,
                                due_date=datetime(
                                    2025,
                                    7,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=3,
                        due_date=datetime(
                            2025, 8, 4, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 5, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=87,
                        days_index=0.935788233217493,
                        accumulated_days_index=2.875999442770713,
                        interest_rate=0.0235,
                        installment_amount=2735.05,
                        installment_amount_without_tac=0,
                        total_amount=8205.15,
                        debit_service=339.13999999999965,
                        customer_debit_service_amount=339.13999999999965,
                        customer_amount=2735.05,
                        calculation_basis_for_effective_interest_rate=2713.0466666666666,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0234,
                        total_effective_cost=0.0282,
                        eir_yearly=0.320481,
                        tec_yearly=0.396244,
                        eir_monthly=0.0234,
                        tec_monthly=0.0282,
                        total_iof=66.01,
                        contract_amount=7866.01,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7799.99,
                        paid_total_iof=66,
                        paid_contract_amount=7866,
                        invoices=[
                            Invoice(
                                accumulated_days=25,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    6,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=55,
                                accumulated_factor=1.94021120955322,
                                factor=0.958839243657051,
                                due_date=datetime(
                                    2025,
                                    7,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=87,
                                accumulated_factor=2.875999442770713,
                                factor=0.935788233217493,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=4,
                        due_date=datetime(
                            2025, 9, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 5, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=117,
                        days_index=0.913291381450307,
                        accumulated_days_index=3.78929082422102,
                        interest_rate=0.0235,
                        installment_amount=2078.54,
                        installment_amount_without_tac=0,
                        total_amount=8314.16,
                        debit_service=437.9499999999999,
                        customer_debit_service_amount=437.9499999999999,
                        customer_amount=2078.54,
                        calculation_basis_for_effective_interest_rate=2059.4875,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0236,
                        total_effective_cost=0.0279,
                        eir_yearly=0.323112,
                        tec_yearly=0.391907,
                        eir_monthly=0.0236,
                        tec_monthly=0.0279,
                        total_iof=76.21,
                        contract_amount=7876.21,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7799.98,
                        paid_total_iof=76.19,
                        paid_contract_amount=7876.19,
                        invoices=[
                            Invoice(
                                accumulated_days=25,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    6,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=55,
                                accumulated_factor=1.94021120955322,
                                factor=0.958839243657051,
                                due_date=datetime(
                                    2025,
                                    7,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=87,
                                accumulated_factor=2.875999442770713,
                                factor=0.935788233217493,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=117,
                                accumulated_factor=3.78929082422102,
                                factor=0.913291381450307,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            DownPaymentResponse(
                installment_amount=500,
                total_amount=1000,
                installment_quantity=2,
                first_payment_date=datetime(
                    2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))
                ),
                plans=[
                    Response(
                        installment=1,
                        due_date=datetime(
                            2025, 7, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 6, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=24,
                        days_index=0.981371965896169,
                        accumulated_days_index=0.981371965896169,
                        interest_rate=0.0235,
                        installment_amount=7994.17,
                        installment_amount_without_tac=0,
                        total_amount=7994.17,
                        debit_service=148.92000000000007,
                        customer_debit_service_amount=148.92000000000007,
                        customer_amount=7994.17,
                        calculation_basis_for_effective_interest_rate=7948.92,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0241,
                        total_effective_cost=0.0317,
                        eir_yearly=0.331065,
                        tec_yearly=0.453471,
                        eir_monthly=0.0241,
                        tec_monthly=0.0317,
                        total_iof=45.25,
                        contract_amount=7845.25,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=45.25,
                        paid_contract_amount=7845.25,
                        invoices=[
                            Invoice(
                                accumulated_days=24,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    7,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=2,
                        due_date=datetime(
                            2025, 8, 4, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 6, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=56,
                        days_index=0.957779256710963,
                        accumulated_days_index=1.9391512226071321,
                        interest_rate=0.0235,
                        installment_amount=4051.09,
                        installment_amount_without_tac=0,
                        total_amount=8102.18,
                        debit_service=246.50000000000028,
                        customer_debit_service_amount=246.50000000000028,
                        customer_amount=4051.09,
                        calculation_basis_for_effective_interest_rate=4023.25,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0238,
                        total_effective_cost=0.0294,
                        eir_yearly=0.326624,
                        tec_yearly=0.416087,
                        eir_monthly=0.0238,
                        tec_monthly=0.0294,
                        total_iof=55.68,
                        contract_amount=7855.68,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=55.68,
                        paid_contract_amount=7855.68,
                        invoices=[
                            Invoice(
                                accumulated_days=24,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    7,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=56,
                                accumulated_factor=1.9391512226071321,
                                factor=0.957779256710963,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=3,
                        due_date=datetime(
                            2025, 9, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 6, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=86,
                        days_index=0.93475372892694,
                        accumulated_days_index=2.873904951534072,
                        interest_rate=0.0235,
                        installment_amount=2736.97,
                        installment_amount_without_tac=0,
                        total_amount=8210.91,
                        debit_service=345.11999999999983,
                        customer_debit_service_amount=345.11999999999983,
                        customer_amount=2736.97,
                        calculation_basis_for_effective_interest_rate=2715.04,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.024,
                        total_effective_cost=0.0288,
                        eir_yearly=0.329156,
                        tec_yearly=0.405648,
                        eir_monthly=0.024,
                        tec_monthly=0.0288,
                        total_iof=65.79,
                        contract_amount=7865.79,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=65.79,
                        paid_contract_amount=7865.79,
                        invoices=[
                            Invoice(
                                accumulated_days=24,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    7,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=56,
                                accumulated_factor=1.9391512226071321,
                                factor=0.957779256710963,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=86,
                                accumulated_factor=2.873904951534072,
                                factor=0.93475372892694,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=4,
                        due_date=datetime(
                            2025, 10, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 6, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=116,
                        days_index=0.912281747198563,
                        accumulated_days_index=3.786186698732635,
                        interest_rate=0.0235,
                        installment_amount=2080.16,
                        installment_amount_without_tac=0,
                        total_amount=8320.64,
                        debit_service=444.74999999999943,
                        customer_debit_service_amount=444.74999999999943,
                        customer_amount=2080.16,
                        calculation_basis_for_effective_interest_rate=2061.1875,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0241,
                        total_effective_cost=0.0285,
                        eir_yearly=0.331464,
                        tec_yearly=0.400907,
                        eir_monthly=0.0241,
                        tec_monthly=0.0285,
                        total_iof=75.89,
                        contract_amount=7875.89,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7799.98,
                        paid_total_iof=75.87,
                        paid_contract_amount=7875.87,
                        invoices=[
                            Invoice(
                                accumulated_days=24,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    7,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=56,
                                accumulated_factor=1.9391512226071321,
                                factor=0.957779256710963,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=86,
                                accumulated_factor=2.873904951534072,
                                factor=0.93475372892694,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=116,
                                accumulated_factor=3.786186698732635,
                                factor=0.912281747198563,
                                due_date=datetime(
                                    2025,
                                    10,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            DownPaymentResponse(
                installment_amount=333.3333333333333,
                total_amount=1000,
                installment_quantity=3,
                first_payment_date=datetime(
                    2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))
                ),
                plans=[
                    Response(
                        installment=1,
                        due_date=datetime(
                            2025, 8, 4, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 7, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=26,
                        days_index=0.980287069256833,
                        accumulated_days_index=0.980287069256833,
                        interest_rate=0.0235,
                        installment_amount=8004.34,
                        installment_amount_without_tac=0,
                        total_amount=8004.34,
                        debit_service=157.79000000000013,
                        customer_debit_service_amount=157.79000000000013,
                        customer_amount=8004.34,
                        calculation_basis_for_effective_interest_rate=7957.79,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0236,
                        total_effective_cost=0.0307,
                        eir_yearly=0.322466,
                        tec_yearly=0.437689,
                        eir_monthly=0.0236,
                        tec_monthly=0.0307,
                        total_iof=46.55,
                        contract_amount=7846.55,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=46.55,
                        paid_contract_amount=7846.55,
                        invoices=[
                            Invoice(
                                accumulated_days=26,
                                accumulated_factor=0.980287069256833,
                                factor=0.980287069256833,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=2,
                        due_date=datetime(
                            2025, 9, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 7, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=56,
                        days_index=0.956720441569568,
                        accumulated_days_index=1.937007510826401,
                        interest_rate=0.0235,
                        installment_amount=4055.92,
                        installment_amount_without_tac=0,
                        total_amount=8111.84,
                        debit_service=255.50000000000014,
                        customer_debit_service_amount=255.50000000000014,
                        customer_amount=4055.92,
                        calculation_basis_for_effective_interest_rate=4027.75,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0241,
                        total_effective_cost=0.0296,
                        eir_yearly=0.330449,
                        tec_yearly=0.418932,
                        eir_monthly=0.0241,
                        tec_monthly=0.0296,
                        total_iof=56.34,
                        contract_amount=7856.34,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800.01,
                        paid_total_iof=56.35,
                        paid_contract_amount=7856.35,
                        invoices=[
                            Invoice(
                                accumulated_days=26,
                                accumulated_factor=0.980287069256833,
                                factor=0.980287069256833,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=56,
                                accumulated_factor=1.937007510826401,
                                factor=0.956720441569568,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=3,
                        due_date=datetime(
                            2025, 10, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 7, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=86,
                        days_index=0.933720368270266,
                        accumulated_days_index=2.870727879096667,
                        interest_rate=0.0235,
                        installment_amount=2740.16,
                        installment_amount_without_tac=0,
                        total_amount=8220.48,
                        debit_service=354.23999999999955,
                        customer_debit_service_amount=354.23999999999955,
                        customer_amount=2740.16,
                        calculation_basis_for_effective_interest_rate=2718.08,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0243,
                        total_effective_cost=0.0291,
                        eir_yearly=0.334168,
                        tec_yearly=0.410516,
                        eir_monthly=0.0243,
                        tec_monthly=0.0291,
                        total_iof=66.24,
                        contract_amount=7866.24,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800.01,
                        paid_total_iof=66.25,
                        paid_contract_amount=7866.25,
                        invoices=[
                            Invoice(
                                accumulated_days=26,
                                accumulated_factor=0.980287069256833,
                                factor=0.980287069256833,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=56,
                                accumulated_factor=1.937007510826401,
                                factor=0.956720441569568,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=86,
                                accumulated_factor=2.870727879096667,
                                factor=0.933720368270266,
                                due_date=datetime(
                                    2025,
                                    10,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=4,
                        due_date=datetime(
                            2025, 11, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 7, 9, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=117,
                        days_index=0.912281747198563,
                        accumulated_days_index=3.78300962629523,
                        interest_rate=0.0235,
                        installment_amount=2082.05,
                        installment_amount_without_tac=0,
                        total_amount=8328.2,
                        debit_service=451.7800000000007,
                        customer_debit_service_amount=451.7800000000007,
                        customer_amount=2082.05,
                        calculation_basis_for_effective_interest_rate=2062.945,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0243,
                        total_effective_cost=0.0286,
                        eir_yearly=0.33315,
                        tec_yearly=0.402404,
                        eir_monthly=0.0243,
                        tec_monthly=0.0286,
                        total_iof=76.42,
                        contract_amount=7876.42,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=76.42,
                        paid_contract_amount=7876.42,
                        invoices=[
                            Invoice(
                                accumulated_days=26,
                                accumulated_factor=0.980287069256833,
                                factor=0.980287069256833,
                                due_date=datetime(
                                    2025,
                                    8,
                                    4,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=56,
                                accumulated_factor=1.937007510826401,
                                factor=0.956720441569568,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=86,
                                accumulated_factor=2.870727879096667,
                                factor=0.933720368270266,
                                due_date=datetime(
                                    2025,
                                    10,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=117,
                                accumulated_factor=3.78300962629523,
                                factor=0.912281747198563,
                                due_date=datetime(
                                    2025,
                                    11,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            DownPaymentResponse(
                installment_amount=250,
                total_amount=1000,
                installment_quantity=4,
                first_payment_date=datetime(
                    2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))
                ),
                plans=[
                    Response(
                        installment=1,
                        due_date=datetime(
                            2025, 9, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 8, 11, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=23,
                        days_index=0.981371965896169,
                        accumulated_days_index=0.981371965896169,
                        interest_rate=0.0235,
                        installment_amount=7993.5,
                        installment_amount_without_tac=0,
                        total_amount=7993.5,
                        debit_service=148.9,
                        customer_debit_service_amount=148.9,
                        customer_amount=7993.5,
                        calculation_basis_for_effective_interest_rate=7948.9,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0252,
                        total_effective_cost=0.0329,
                        eir_yearly=0.347719,
                        tec_yearly=0.475332,
                        eir_monthly=0.0252,
                        tec_monthly=0.0329,
                        total_iof=44.6,
                        contract_amount=7844.6,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=44.6,
                        paid_contract_amount=7844.6,
                        invoices=[
                            Invoice(
                                accumulated_days=23,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=2,
                        due_date=datetime(
                            2025, 10, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 8, 11, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=53,
                        days_index=0.957779256710963,
                        accumulated_days_index=1.9391512226071321,
                        interest_rate=0.0235,
                        installment_amount=4050.43,
                        installment_amount_without_tac=0,
                        total_amount=8100.86,
                        debit_service=246.4699999999997,
                        customer_debit_service_amount=246.4699999999997,
                        customer_amount=4050.43,
                        calculation_basis_for_effective_interest_rate=4023.2349999999997,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0251,
                        total_effective_cost=0.0308,
                        eir_yearly=0.34648,
                        tec_yearly=0.439943,
                        eir_monthly=0.0251,
                        tec_monthly=0.0308,
                        total_iof=54.39,
                        contract_amount=7854.39,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800.01,
                        paid_total_iof=54.4,
                        paid_contract_amount=7854.4,
                        invoices=[
                            Invoice(
                                accumulated_days=23,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=53,
                                accumulated_factor=1.9391512226071321,
                                factor=0.957779256710963,
                                due_date=datetime(
                                    2025,
                                    10,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=3,
                        due_date=datetime(
                            2025, 11, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 8, 11, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=84,
                        days_index=0.935788233217493,
                        accumulated_days_index=2.874939455824625,
                        interest_rate=0.0235,
                        installment_amount=2735.54,
                        installment_amount_without_tac=0,
                        total_amount=8206.62,
                        debit_service=342.1200000000008,
                        customer_debit_service_amount=342.1200000000008,
                        customer_amount=2735.54,
                        calculation_basis_for_effective_interest_rate=2714.0400000000004,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0247,
                        total_effective_cost=0.0296,
                        eir_yearly=0.340141,
                        tec_yearly=0.418684,
                        eir_monthly=0.0247,
                        tec_monthly=0.0296,
                        total_iof=64.5,
                        contract_amount=7864.5,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800.01,
                        paid_total_iof=64.51,
                        paid_contract_amount=7864.51,
                        invoices=[
                            Invoice(
                                accumulated_days=23,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=53,
                                accumulated_factor=1.9391512226071321,
                                factor=0.957779256710963,
                                due_date=datetime(
                                    2025,
                                    10,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=84,
                                accumulated_factor=2.874939455824625,
                                factor=0.935788233217493,
                                due_date=datetime(
                                    2025,
                                    11,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                    Response(
                        installment=4,
                        due_date=datetime(
                            2025, 12, 3, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        disbursement_date=datetime(
                            2025, 8, 11, tzinfo=timezone(timedelta(hours=-3))
                        ),
                        accumulated_days=114,
                        days_index=0.914302133077605,
                        accumulated_days_index=3.78924158890223,
                        interest_rate=0.0235,
                        installment_amount=2078.15,
                        installment_amount_without_tac=0,
                        total_amount=8312.6,
                        debit_service=437.99000000000035,
                        customer_debit_service_amount=437.99000000000035,
                        customer_amount=2078.15,
                        calculation_basis_for_effective_interest_rate=2059.4975,
                        merchant_debit_service_amount=0,
                        merchant_total_amount=390,
                        settled_to_merchant=7410,
                        mdr_amount=390,
                        effective_interest_rate=0.0245,
                        total_effective_cost=0.0289,
                        eir_yearly=0.336923,
                        tec_yearly=0.407548,
                        eir_monthly=0.0245,
                        tec_monthly=0.0289,
                        total_iof=74.61,
                        contract_amount=7874.61,
                        contract_amount_without_tac=0,
                        tac_amount=0,
                        iof_percentage=0.000082,
                        overall_iof=0.0038,
                        pre_disbursement_amount=7800,
                        paid_total_iof=74.61,
                        paid_contract_amount=7874.61,
                        invoices=[
                            Invoice(
                                accumulated_days=23,
                                accumulated_factor=0.981371965896169,
                                factor=0.981371965896169,
                                due_date=datetime(
                                    2025,
                                    9,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=53,
                                accumulated_factor=1.9391512226071321,
                                factor=0.957779256710963,
                                due_date=datetime(
                                    2025,
                                    10,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=84,
                                accumulated_factor=2.874939455824625,
                                factor=0.935788233217493,
                                due_date=datetime(
                                    2025,
                                    11,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                            Invoice(
                                accumulated_days=114,
                                accumulated_factor=3.78924158890223,
                                factor=0.914302133077605,
                                due_date=datetime(
                                    2025,
                                    12,
                                    3,
                                    7,
                                    0,
                                    0,
                                    tzinfo=timezone(timedelta(hours=-3)),
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ]

        params = Params(
            requested_amount=7800,
            first_payment_date=datetime(
                2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))
            ),
            disbursement_date=datetime(
                2025, 4, 5, tzinfo=timezone(timedelta(hours=-3))
            ),
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

        down_payment_params = DownPaymentParams(
            first_payment_date=datetime(
                2025, 5, 3, tzinfo=timezone(timedelta(hours=-3))
            ),
            requested_amount=1000,
            installments=4,
            min_installment_amount=100,
            params=params,
        )

        resp = calculate_down_payment_plan(down_payment_params)

        self.assertEqual(
            len(resp), len(expected), "Response length does not match expected length"
        )

        for i, (dr, de) in enumerate(zip(resp, expected)):
            self.assertEqual(
                dr.installment_amount,
                de.installment_amount,
                f"Installment {i + 1}: InstallmentAmount mismatch",
            )
            self.assertEqual(
                dr.total_amount,
                de.total_amount,
                f"Installment {i + 1}: TotalAmount mismatch",
            )
            self.assertEqual(
                dr.installment_quantity,
                de.installment_quantity,
                f"Installment {i + 1}: InstallmentQuantity mismatch",
            )

            for _, (r, e) in enumerate(zip(dr.plans, de.plans)):
                self.assertEqual(
                    r.installment,
                    e.installment,
                    f"Installment {i + 1}: Installment mismatch",
                )
                self.assertEqual(
                    r.accumulated_days,
                    e.accumulated_days,
                    f"Installment {i + 1}: AccumulatedDays mismatch",
                )
                self.assertAlmostEqual(
                    r.days_index,
                    e.days_index,
                    places=6,
                    msg=f"Installment {i + 1}: DaysIndex mismatch",
                )
                self.assertAlmostEqual(
                    r.accumulated_days_index,
                    e.accumulated_days_index,
                    places=6,
                    msg=f"Installment {i + 1}: AccumulatedDaysIndex mismatch",
                )
                self.assertAlmostEqual(
                    r.interest_rate,
                    e.interest_rate,
                    places=6,
                    msg=f"Installment {i + 1}: InterestRate mismatch",
                )
                self.assertAlmostEqual(
                    r.installment_amount,
                    e.installment_amount,
                    places=2,
                    msg=f"Installment {i + 1}: InstallmentAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.installment_amount_without_tac,
                    e.installment_amount_without_tac,
                    places=2,
                    msg=f"Installment {i + 1}: InstallmentAmountWithoutTac mismatch",
                )
                self.assertAlmostEqual(
                    r.total_amount,
                    e.total_amount,
                    places=2,
                    msg=f"Installment {i + 1}: TotalAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.debit_service,
                    e.debit_service,
                    places=2,
                    msg=f"Installment {i + 1}: DebitService mismatch",
                )
                self.assertAlmostEqual(
                    r.customer_debit_service_amount,
                    e.customer_debit_service_amount,
                    places=2,
                    msg=f"Installment {i + 1}: CustomerDebitServiceAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.customer_amount,
                    e.customer_amount,
                    places=2,
                    msg=f"Installment {i + 1}: CustomerAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.calculation_basis_for_effective_interest_rate,
                    e.calculation_basis_for_effective_interest_rate,
                    places=2,
                    msg=f"Installment {i + 1}: CalculationBasisForEffectiveInterestRate mismatch",
                )
                self.assertAlmostEqual(
                    r.merchant_debit_service_amount,
                    e.merchant_debit_service_amount,
                    places=2,
                    msg=f"Installment {i + 1}: MerchantDebitServiceAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.merchant_total_amount,
                    e.merchant_total_amount,
                    places=2,
                    msg=f"Installment {i + 1}: MerchantTotalAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.settled_to_merchant,
                    e.settled_to_merchant,
                    places=2,
                    msg=f"Installment {i + 1}: SettledToMerchant mismatch",
                )
                self.assertAlmostEqual(
                    r.mdr_amount,
                    e.mdr_amount,
                    places=2,
                    msg=f"Installment {i + 1}: MdrAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.effective_interest_rate,
                    e.effective_interest_rate,
                    places=6,
                    msg=f"Installment {i + 1}: EffectiveInterestRate mismatch",
                )
                self.assertAlmostEqual(
                    r.total_effective_cost,
                    e.total_effective_cost,
                    places=6,
                    msg=f"Installment {i + 1}: TotalEffectiveCost mismatch",
                )
                self.assertAlmostEqual(
                    r.eir_yearly,
                    e.eir_yearly,
                    places=6,
                    msg=f"Installment {i + 1}: EirYearly mismatch",
                )
                self.assertAlmostEqual(
                    r.tec_yearly,
                    e.tec_yearly,
                    places=6,
                    msg=f"Installment {i + 1}: TecYearly mismatch",
                )
                self.assertAlmostEqual(
                    r.eir_monthly,
                    e.eir_monthly,
                    places=6,
                    msg=f"Installment {i + 1}: EirMonthly mismatch",
                )
                self.assertAlmostEqual(
                    r.tec_monthly,
                    e.tec_monthly,
                    places=6,
                    msg=f"Installment {i + 1}: TecMonthly mismatch",
                )
                self.assertAlmostEqual(
                    r.total_iof,
                    e.total_iof,
                    places=2,
                    msg=f"Installment {i + 1}: TotalIof mismatch",
                )
                self.assertAlmostEqual(
                    r.contract_amount,
                    e.contract_amount,
                    places=2,
                    msg=f"Installment {i + 1}: ContractAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.contract_amount_without_tac,
                    e.contract_amount_without_tac,
                    places=2,
                    msg=f"Installment {i + 1}: ContractAmountWithoutTac mismatch",
                )
                self.assertAlmostEqual(
                    r.tac_amount,
                    e.tac_amount,
                    places=2,
                    msg=f"Installment {i + 1}: TacAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.iof_percentage,
                    e.iof_percentage,
                    places=6,
                    msg=f"Installment {i + 1}: IofPercentage mismatch",
                )
                self.assertAlmostEqual(
                    r.overall_iof,
                    e.overall_iof,
                    places=6,
                    msg=f"Installment {i + 1}: OverallIof mismatch",
                )
                self.assertAlmostEqual(
                    r.pre_disbursement_amount,
                    e.pre_disbursement_amount,
                    places=2,
                    msg=f"Installment {i + 1}: PreDisbursementAmount mismatch",
                )
                self.assertAlmostEqual(
                    r.paid_total_iof,
                    e.paid_total_iof,
                    places=2,
                    msg=f"Installment {i + 1}: PaidTotalIof mismatch",
                )
                self.assertAlmostEqual(
                    r.paid_contract_amount,
                    e.paid_contract_amount,
                    places=2,
                    msg=f"Installment {i + 1}: PaidContractAmount mismatch",
                )
                for j, (ri, ei) in enumerate(zip(r.invoices, e.invoices)):
                    self.assertEqual(
                        ri.accumulated_days,
                        ei.accumulated_days,
                        f"Installment {i + 1}, Invoice {j + 1}: AccumulatedDays mismatch",
                    )
                    self.assertAlmostEqual(
                        ri.accumulated_factor,
                        ei.accumulated_factor,
                        places=6,
                        msg=f"Installment {i + 1}, Invoice {j + 1}: AccumulatedFactor mismatch",
                    )
                    self.assertAlmostEqual(
                        ri.factor,
                        ei.factor,
                        places=6,
                        msg=f"Installment {i + 1}, Invoice {j + 1}: Factor mismatch",
                    )
                    self.assertEqual(
                        ri.due_date,
                        ei.due_date,
                        f"Installment {i + 1}, Invoice {j + 1}: DueDate mismatch",
                    )


if __name__ == "__main__":
    unittest.main()

if __name__ == "__main__":
    unittest.main()
