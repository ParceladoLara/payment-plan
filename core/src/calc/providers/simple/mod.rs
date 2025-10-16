use amounts::calculate_amounts;
use iof::calculate_iof;
use prepare::{prepare_calculation, PreparedCalculation};
use rust_decimal::{dec, Decimal, MathematicalOps};

use crate::{
    calc::{
        inner_xirr::{eir::calculate_eir_monthly, prepare_xirr_params, tec::calculate_tec_monthly},
        PaymentPlan,
    },
    err::PaymentPlanError,
    Params, Response,
};

mod amounts;
mod iof;
mod prepare;

const CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE: Decimal = dec!(0.0821917808219178);

/**
 * This is a simpler implementation of the payment plan calculation.
 * It estimates the iof final iof value based on the overall iof percentage
 * This results in less precise results, because the real iof value comes from a iterative calculation that uses the result of the previous iteration to calculate the next one.
 * But it is much faster and simpler to understand.
 * And as is cannot calculate the plan on business days only
 * Right now it is here for legacy reasons, but it is not recommended to use it in new code.
 */
pub struct Simple;

impl PaymentPlan for Simple {
    fn calculate_payment_plan(&self, params: Params) -> Result<Vec<Response>, PaymentPlanError> {
        let prepared_calculations = prepare_calculation(params);
        let calculated = calculate(params, prepared_calculations);

        return calculated;
    }
}

fn calculate(
    params: Params,
    prepared_calculations: Vec<PreparedCalculation>,
) -> Result<Vec<Response>, PaymentPlanError> {
    if params.requested_amount <= Decimal::ZERO {
        return Err(PaymentPlanError::InvalidRequestedAmount);
    }
    if params.installments == 0 {
        return Err(PaymentPlanError::InvalidNumberOfInstallments);
    }

    let mut responses = Vec::new();
    let requested_amount = params.requested_amount;
    let debit_service_percentage = params.debit_service_percentage;
    let interest_rate = params.interest_rate;
    let tac_percentage = params.tac_percentage;
    let iof_overall = params.iof_overall;
    let iof_percentage = params.iof_percentage;

    let customer_debit_service_proportion =
        Decimal::ONE - Decimal::from(debit_service_percentage) / Decimal::ONE_HUNDRED;
    let tac_amount = requested_amount * tac_percentage;

    for (i, prepared_calculation) in prepared_calculations.iter().enumerate() {
        let aux_accumulated_days_index: Vec<i64> = prepared_calculations
            .iter()
            .take(prepared_calculation.installment as usize)
            .map(|calc| calc.accumulated_days)
            .collect();

        let total_iof = calculate_iof(
            params,
            aux_accumulated_days_index,
            prepared_calculation.installment.into(),
        );

        let amounts = calculate_amounts(
            params,
            prepared_calculation.accumulated_days_index,
            prepared_calculation.installment.into(),
            customer_debit_service_proportion,
            total_iof,
        );

        let due_dates = prepared_calculations
            .iter()
            .map(|calc| calc.due_date)
            .collect();

        let (eir_params, tec_params) = prepare_xirr_params(
            prepared_calculation.installment,
            &due_dates,
            amounts.calculation_basis_for_effective_interest_rate,
            amounts.customer_amount,
        );

        let eir_monthly = calculate_eir_monthly(
            params,
            eir_params,
            customer_debit_service_proportion,
            CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE,
        )?;

        let eir_yearly = (Decimal::ONE + eir_monthly).powf(12.0) - Decimal::ONE;

        let tec_monthly = calculate_tec_monthly(
            params,
            tec_params,
            CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE,
        )?;

        let tec_yearly = (Decimal::ONE + tec_monthly).powf(12.0) - Decimal::ONE;

        let installment_amount = amounts.installment_amount;

        if installment_amount < params.min_installment_amount && i != 0 {
            break;
        }

        if amounts.total_amount > params.max_total_amount {
            break;
        }

        let response = Response {
            installment: prepared_calculation.installment,
            due_date: prepared_calculation.due_date,
            accumulated_days: prepared_calculation.accumulated_days,
            days_index: prepared_calculation.days_index,
            accumulated_days_index: prepared_calculation.accumulated_days_index,
            interest_rate,
            installment_amount: amounts.installment_amount,
            installment_amount_without_tac: amounts.installment_amount_without_tac,
            total_amount: amounts.total_amount,
            debit_service: amounts.debit_service,
            customer_debit_service_amount: amounts.customer_debit_service_amount,
            customer_amount: amounts.customer_amount,
            calculation_basis_for_effective_interest_rate: amounts
                .calculation_basis_for_effective_interest_rate,
            merchant_debit_service_amount: amounts.merchant_debit_service_amount,
            merchant_total_amount: amounts.merchant_total_amount,
            settled_to_merchant: amounts.settled_to_merchant,
            mdr_amount: amounts.mdr_amount,
            effective_interest_rate: eir_monthly,
            total_effective_cost: tec_monthly,
            eir_yearly,
            tec_yearly,
            eir_monthly,
            tec_monthly,
            total_iof,
            contract_amount: amounts.contract_amount,
            contract_amount_without_tac: amounts.contract_amount_without_tac,
            tac_amount,
            iof_percentage,
            overall_iof: iof_overall,
            disbursement_date: params.disbursement_date,
            pre_disbursement_amount: amounts.total_amount,
            paid_total_iof: total_iof,
            paid_contract_amount: amounts.contract_amount,
            invoices: vec![prepared_calculation.invoice],
        };

        responses.push(response);
    }

    return Ok(responses);
}

#[cfg(test)]
mod test {
    //Test 0 - (8800 / 24) = (11980.77027564256 / 499.1987614851067)
    //Test 1 - (6000 / 18) = (7739.024786678216 / 429.9458214821231)
    //Test 2 - (1300 / 12) = (1541.6623345164212 / 128.47186120970176)
    //Test 3 - (1600 / 9) = (1831.00234095926 / 203.44470455102888)
    //Test 4 - (1000 / 9) = (1140.115221691851 / 126.67946907687234)
    //Test 5 - (4580 / 24) = (7070.7838245293115 / 294.6159926887213)
    //Test 6 - (1500 / 12) = (1795.186723818578 / 149.59889365154817)
    //Test 7 - (2900 / 6) = (3314.5935321072 / 552.4322553512001)
    //Test 8 - (3769.6 / 24) = (5346.43148292502 / 222.76797845520915)
    //Test 9 - (6200 / 3) = (6627.572802678283 / 2209.190934226094)
    //Test 10 - (2690.1 / 12) = (3234.185360936014 / 269.5154467446678)
    //Test 11 - (1089 / 4) = (1160.9637963521732 / 290.2409490880433)
    //Test 12 - (1752 / 10) = (2040.6358370477474 / 204.06358370477474)
    //Test 13 - (4000 / 24) = (5323.461599385834 / 221.81089997440975)
    //Test 14 - (6500 / 11) = (8146.322444824322 / 740.574767711302)
    //Test 15 - (1000 / 24) = (1275.5756523433513 / 106.29797102861261) max installment amount 100
    //Test 16 - (44 / 48) = (46.05063251213531 / 46.05063251213531) min installment amount 80

    use rust_decimal::{dec, Decimal};

    use crate::{calc::PaymentPlan, Params};

    const SIMPLE: super::Simple = super::Simple {};

    #[test]
    fn test_calculate_payment_plan_test_0() {
        let expected_contract_amount = dec!(9037.318869753424);
        let expected_contract_amount_without_tac = dec!(9037.318869753424);
        let expected_customer_amount = dec!(499.1987614851067);
        let expected_customer_debit_service_amount = dec!(2943.451405889136);
        let expected_debit_service = dec!(2943.451405889136);
        let expected_eir_monthly = dec!(0.024085088183680048);
        let expected_eir_yearly = dec!(0.33055401101326365);
        let expected_effective_interest_rate = dec!(0.024085088183680048); // dec!(0.0235) in node
        let expected_installment = 24;
        let expected_installment_amount = dec!(499.1987614851067);
        let expected_interest_rate = dec!(0.0235);
        let expected_mdr_amount = dec!(440.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(440.0);
        let expected_settled_to_merchant = dec!(8360.0);
        let expected_tec_monthly = dec!(0.025868426671143974);
        let expected_tec_yearly = dec!(0.3586261331729559);
        let expected_total_amount = dec!(11980.77027564256);
        let expected_total_iof = dec!(237.3188697534247);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(8800.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 18).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 18).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0235),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 24);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_1() {
        let expected_contract_amount = dec!(6148.387557205479);
        let expected_contract_amount_without_tac = dec!(6148.387557205479);
        let expected_customer_amount = dec!(430.32094244906153);
        let expected_customer_debit_service_amount = dec!(1597.389406877628);
        let expected_debit_service = dec!(1597.389406877628);
        let expected_eir_monthly = dec!(0.02557918934592962);
        let expected_eir_yearly = dec!(0.3540365786122326);
        let expected_effective_interest_rate = dec!(0.02557918934592962); // 0.025 in node
        let expected_installment = 18;
        let expected_installment_amount = dec!(430.32094244906153);
        let expected_interest_rate = dec!(0.025);
        let expected_mdr_amount = dec!(300.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(300.0);
        let expected_settled_to_merchant = dec!(5700.0);
        let expected_tec_monthly = dec!(0.027793563251085507);
        let expected_tec_yearly = dec!(0.38953894087787666);
        let expected_total_amount = dec!(7745.7769640831075);
        let expected_total_iof = dec!(148.38755720547942);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(6000.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 18).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 17).unwrap(),
            installments: 18,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.025),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 18);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_2() {
        let expected_contract_amount = dec!(1326.1754959452055);
        let expected_contract_amount_without_tac = dec!(1326.1754959452055);
        let expected_customer_amount = dec!(128.47186120970176);
        let expected_customer_debit_service_amount = dec!(215.4868385712157);
        let expected_debit_service = dec!(215.4868385712157);
        let expected_eir_monthly = dec!(0.023954074195358555);
        let expected_eir_yearly = dec!(0.3285127909894192);
        let expected_effective_interest_rate = dec!(0.023954074195358555); // dec!(0.0235) in node
        let expected_installment = 12;
        let expected_installment_amount = dec!(128.47186120970176);
        let expected_interest_rate = dec!(0.0235);
        let expected_mdr_amount = dec!(65.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(65.0);
        let expected_settled_to_merchant = dec!(1235.0);
        let expected_tec_monthly = dec!(0.026733709153886398);
        let expected_tec_yearly = dec!(0.37244152356319127);
        let expected_total_amount = dec!(1541.6623345164212);
        let expected_total_iof = dec!(26.17549594520548);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(1300.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 21).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 21).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0235),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 12);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_3() {
        let expected_contract_amount = dec!(1626.1424272328768);
        let expected_contract_amount_without_tac = dec!(1626.1424272328768);
        let expected_customer_amount = dec!(203.44470455102888);
        let expected_customer_debit_service_amount = dec!(204.85991372638327);
        let expected_debit_service = dec!(204.85991372638327);
        let expected_eir_monthly = dec!(0.024380237604045174);
        let expected_eir_yearly = dec!(0.33516302665077946);
        let expected_effective_interest_rate = dec!(0.024380237604045174); // 0.024 in node
        let expected_installment = 9;
        let expected_installment_amount = dec!(203.44470455102888);
        let expected_interest_rate = dec!(0.024);
        let expected_mdr_amount = dec!(80.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(80.0);
        let expected_settled_to_merchant = dec!(1520.0);
        let expected_tec_monthly = dec!(0.027386037131249097);
        let expected_tec_yearly = dec!(0.3829418169609544); //  0.3829418169609542 in node
        let expected_total_amount = dec!(1831.00234095926);
        let expected_total_iof = dec!(26.142427232876713);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(1600.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 29).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 29).unwrap(),
            installments: 9,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.024),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 9);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_4() {
        let expected_contract_amount = dec!(1016.2107967945205);
        let expected_contract_amount_without_tac = dec!(1016.2107967945205);
        let expected_customer_amount = dec!(126.67946907687234);
        let expected_customer_debit_service_amount = dec!(123.9044248973305);
        let expected_debit_service = dec!(123.9044248973305);
        let expected_eir_monthly = dec!(0.023869886439737753);
        let expected_eir_yearly = dec!(0.3272026469033864); //0.3272026469033862 in node
        let expected_effective_interest_rate = dec!(0.023869886439737753); // dec!(0.0235) in node
        let expected_installment = 9;
        let expected_installment_amount = dec!(126.67946907687234);
        let expected_interest_rate = dec!(0.0235);
        let expected_mdr_amount = dec!(50.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(50.0);
        let expected_settled_to_merchant = dec!(950.0);
        let expected_tec_monthly = dec!(0.026891280795923622);
        let expected_tec_yearly = dec!(0.374971182359072);
        let expected_total_amount = dec!(1140.115221691851);
        let expected_total_iof = dec!(16.210796794520547);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(1000.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 08).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 10).unwrap(),
            installments: 9,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0235),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 9);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_5() {
        let expected_contract_amount = dec!(4703.573142849315);
        let expected_contract_amount_without_tac = dec!(4703.573142849315);
        let expected_customer_amount = dec!(294.61599268872135);
        let expected_customer_debit_service_amount = dec!(2367.2106816799965);
        let expected_debit_service = dec!(2367.2106816799965);
        let expected_eir_monthly = dec!(0.03574394430986261);
        let expected_eir_yearly = dec!(0.5241539411857024);
        let expected_effective_interest_rate = dec!(0.03574394430986261); // 0.0349 in node
        let expected_installment = 24;
        let expected_installment_amount = dec!(294.6159926887213); // 294.61599268872135 in node
        let expected_interest_rate = dec!(0.0349);
        let expected_mdr_amount = dec!(45.800000000000004);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(45.800000000000004);
        let expected_settled_to_merchant = dec!(4534.2);
        let expected_tec_monthly = dec!(0.03740934806328888);
        let expected_tec_yearly = dec!(0.5538242142983534);
        let expected_total_amount = dec!(7070.7838245293115);
        let expected_total_iof = dec!(123.57314284931509);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(4580.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 05).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 04).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.034),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 24);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_6() {
        let expected_contract_amount = dec!(1530.1828767123288);
        let expected_contract_amount_without_tac = dec!(1530.1828767123288);
        let expected_customer_amount = dec!(149.59889365154822); // 149.59889365154817 in node
        let expected_customer_debit_service_amount = dec!(265.0038471062499); // 265.0038471062492 in node
        let expected_debit_service = dec!(265.0038471062499); // 265.0038471062492 in node
        let expected_eir_monthly = dec!(0.025481551205442488);
        let expected_eir_yearly = dec!(0.3524904893931555);
        let expected_effective_interest_rate = dec!(0.025481551205442488); // 0.025 in node
        let expected_installment = 12;
        let expected_installment_amount = dec!(149.59889365154822); // 149.59889365154817 in node
        let expected_interest_rate = dec!(0.025);
        let expected_mdr_amount = dec!(75.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(75.0);
        let expected_settled_to_merchant = dec!(1425.0);
        let expected_tec_monthly = dec!(0.02824733087479081); // 0.028247330874790588 in node
        let expected_tec_yearly = dec!(0.396918568026396); // 0.39691856802639225 in node
        let expected_total_amount = dec!(1795.1867238185787); // 1795.186723818578 in node
        let expected_total_iof = dec!(30.182876712328767);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(1500.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 09).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 09).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.025),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 12);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_7() {
        let expected_contract_amount = dec!(2936.563583452055);
        let expected_contract_amount_without_tac = dec!(2936.563583452055);
        let expected_customer_amount = dec!(552.4322553512001);
        let expected_customer_debit_service_amount = dec!(378.0299486551454);
        let expected_debit_service = dec!(378.0299486551454);
        let expected_eir_monthly = dec!(0.035429014326330055);
        let expected_eir_yearly = dec!(0.5186019914133586);
        let expected_effective_interest_rate = dec!(0.035429014326330055); // 0.035 in node
        let expected_installment = 6;
        let expected_installment_amount = dec!(552.4322553512001);
        let expected_interest_rate = dec!(0.035);
        let expected_mdr_amount = dec!(86.71000000000001);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(86.71000000000001);
        let expected_settled_to_merchant = dec!(2813.29);
        let expected_tec_monthly = dec!(0.03875204347989669);
        let expected_tec_yearly = dec!(0.5781297023988077);
        let expected_total_amount = dec!(3314.5935321072);
        let expected_total_iof = dec!(36.56358345205479);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(2900.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 30).unwrap(),
            installments: 6,
            debit_service_percentage: 0,
            mdr: dec!(0.029900000000000003),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.035),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 6);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_8() {
        let expected_contract_amount = dec!(3868.0712711232877);
        let expected_contract_amount_without_tac = dec!(3868.0712711232877);
        let expected_customer_amount = dec!(222.76797845520915);
        let expected_customer_debit_service_amount = dec!(1478.3602118017322);
        let expected_debit_service = dec!(1478.3602118017322);
        let expected_eir_monthly = dec!(0.029698339097185666);
        let expected_eir_yearly = dec!(0.42075811960150045);
        let expected_effective_interest_rate = dec!(0.029698339097185666); // 0.0297 in node
        let expected_installment = 24;
        let expected_installment_amount = dec!(222.76797845520915);
        let expected_interest_rate = dec!(0.028999999999999998);
        let expected_mdr_amount = dec!(112.71104000000001);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(112.71104000000001);
        let expected_settled_to_merchant = dec!(3656.8889599999998);
        let expected_tec_monthly = dec!(0.031514336661548015);
        let expected_tec_yearly = dec!(0.4511196449073398);
        let expected_total_amount = dec!(5346.43148292502);
        let expected_total_iof = dec!(98.47127112328764);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(3769.6),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 10).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: dec!(0.029900000000000003),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.028999999999999998),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 24);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_9() {
        let expected_contract_amount = dec!(6249.888847589041);
        let expected_contract_amount_without_tac = dec!(6249.888847589041);
        let expected_customer_amount = dec!(2209.190934226094);
        let expected_customer_debit_service_amount = dec!(377.6839550892415);
        let expected_debit_service = dec!(377.6839550892415);
        let expected_eir_monthly = dec!(0.03517937087855172);
        let expected_eir_yearly = dec!(0.5142141670632188);
        let expected_effective_interest_rate = dec!(0.03517937087855172); // 0.0349 in node
        let expected_installment = 3;
        let expected_installment_amount = dec!(2209.190934226094);
        let expected_interest_rate = dec!(0.0349);
        let expected_mdr_amount = dec!(62.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(62.0);
        let expected_settled_to_merchant = dec!(6138.0);
        let expected_tec_monthly = dec!(0.039799198101799105);
        let expected_tec_yearly = dec!(0.5973266517177596);
        let expected_total_amount = dec!(6627.572802678283);
        let expected_total_iof = dec!(49.8888475890411);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(6200.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 25).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 04).unwrap(),
            installments: 3,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0349),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 3);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_10() {
        let expected_contract_amount = dec!(2739.974829369863);
        let expected_contract_amount_without_tac = dec!(2739.974829369863);
        let expected_customer_amount = dec!(269.5154467446678);
        let expected_customer_debit_service_amount = dec!(494.21053156615125);
        let expected_debit_service = dec!(494.21053156615125);
        let expected_eir_monthly = dec!(0.029518034828695416);
        let expected_eir_yearly = dec!(0.41777562837546256);
        let expected_effective_interest_rate = dec!(0.029518034828695416); // 0.029 in node
        let expected_installment = 12;
        let expected_installment_amount = dec!(269.5154467446678);
        let expected_interest_rate = dec!(0.029);
        let expected_mdr_amount = dec!(80.43399000000001);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(80.43399000000001);
        let expected_settled_to_merchant = dec!(2609.66601);
        let expected_tec_monthly = dec!(0.03237874905455129);
        let expected_tec_yearly = dec!(0.46577960649402206);
        let expected_total_amount = dec!(3234.185360936014);
        let expected_total_iof = dec!(49.874829369863015);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(2690.1),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 15).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 04).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: dec!(0.029900000000000003),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.029),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 12);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_11() {
        let expected_contract_amount = dec!(1099.9854739726027);
        let expected_contract_amount_without_tac = dec!(1099.9854739726027);
        let expected_customer_amount = dec!(290.2409490880433);
        let expected_customer_debit_service_amount = dec!(60.978322379570436);
        let expected_debit_service = dec!(60.978322379570436);
        let expected_eir_monthly = dec!(0.021714523316856305);
        let expected_eir_yearly = dec!(0.2940611563328235);
        let expected_effective_interest_rate = dec!(0.021714523316856305); // 0.0215 in node
        let expected_installment = 4;
        let expected_installment_amount = dec!(290.2409490880433);
        let expected_interest_rate = dec!(0.0215);
        let expected_mdr_amount = dec!(10.89);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(10.89);
        let expected_settled_to_merchant = dec!(1078.11);
        let expected_tec_monthly = dec!(0.02557650331018846);
        let expected_tec_yearly = dec!(0.3539940238690005); // 0.3539940238690007 in node
        let expected_total_amount = dec!(1160.9637963521732);
        let expected_total_iof = dec!(10.98547397260274);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(1089.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 29).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 29).unwrap(),
            installments: 4,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0215),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 4);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_12() {
        let expected_contract_amount = dec!(1782.8928);
        let expected_contract_amount_without_tac = dec!(1782.8928);
        let expected_customer_amount = dec!(204.24811158092817);
        let expected_customer_debit_service_amount = dec!(259.5883158092816);
        let expected_debit_service = dec!(259.5883158092816);
        let expected_eir_monthly = dec!(0.025425157547064314); //0.025425157547064092 in node
        let expected_eir_yearly = dec!(0.3515982394442816); // 0.35159823944427804 in node
        let expected_effective_interest_rate = dec!(0.025425157547064314); // 0.025 in node
        let expected_installment = 10;
        let expected_installment_amount = dec!(204.24811158092817);
        let expected_interest_rate = dec!(0.025);
        let expected_mdr_amount = dec!(87.60000000000001);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(87.60000000000001);
        let expected_settled_to_merchant = dec!(1664.4);
        let expected_tec_monthly = dec!(0.028331897318861987);
        let expected_tec_yearly = dec!(0.39829783796886464);
        let expected_total_amount = dec!(2042.4811158092816);
        let expected_total_iof = dec!(30.8928);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(1752.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 16).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 16).unwrap(),
            installments: 10,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.025),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 10);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_13() {
        let expected_contract_amount = dec!(4107.87339030137);
        let expected_contract_amount_without_tac = dec!(4107.87339030137);
        let expected_customer_amount = dec!(221.81089997440975);
        let expected_customer_debit_service_amount = dec!(1215.5882090844643);
        let expected_debit_service = dec!(1215.5882090844643);
        let expected_eir_monthly = dec!(0.02203837782073359);
        let expected_eir_yearly = dec!(0.2989919143131987);
        let expected_effective_interest_rate = dec!(0.02203837782073359); // 0.0215 in node
        let expected_installment = 24;
        let expected_installment_amount = dec!(221.81089997440975);
        let expected_interest_rate = dec!(0.0215);
        let expected_mdr_amount = dec!(40.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(40.0);
        let expected_settled_to_merchant = dec!(3960.0);
        let expected_tec_monthly = dec!(0.02384436103270393);
        let expected_tec_yearly = dec!(0.3268056502522887);
        let expected_total_amount = dec!(5323.461599385834);
        let expected_total_iof = dec!(107.8733903013699);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(4000.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 14).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 14).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0215),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 24);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_14() {
        let expected_contract_amount = dec!(6622.710151424658);
        let expected_contract_amount_without_tac = dec!(6622.710151424658);
        let expected_customer_amount = dec!(740.574767711302);
        let expected_customer_debit_service_amount = dec!(1523.6122933996642);
        let expected_debit_service = dec!(1523.6122933996642);
        let expected_eir_monthly = dec!(0.036134558462701305); // 0.03613455846270108 in node
        let expected_eir_yearly = dec!(0.5310659881278654); //0.5310659881278617 in node
        let expected_effective_interest_rate = dec!(0.036134558462701305); // 0.0355 in node
        let expected_installment = 11;
        let expected_installment_amount = dec!(740.574767711302);
        let expected_interest_rate = dec!(0.0355);
        let expected_mdr_amount = dec!(65.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(65.0);
        let expected_settled_to_merchant = dec!(6435.0);
        let expected_tec_monthly = dec!(0.0388795124728023);
        let expected_tec_yearly = dec!(0.5804551670400413);
        let expected_total_amount = dec!(8146.322444824322);
        let expected_total_iof = dec!(122.71015142465752);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(6500.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 11,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0355),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 11);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_15() {
        let expected_contract_amount = dec!(1020.1211129315069);
        let expected_contract_amount_without_tac = dec!(1020.1211129315069);
        let expected_customer_amount = dec!(106.29797102861261);
        let expected_customer_debit_service_amount = dec!(255.45453941184445);
        let expected_debit_service = dec!(255.45453941184445);
        let expected_eir_monthly = dec!(0.03617300584122107);
        let expected_eir_yearly = dec!(0.531747878195636);
        let expected_effective_interest_rate = dec!(0.03617300584122107);
        let expected_installment = 12;
        let expected_installment_amount = dec!(106.29797102861261);
        let expected_interest_rate = dec!(0.0355);
        let expected_mdr_amount = dec!(10.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(10.0);
        let expected_settled_to_merchant = dec!(990.0);
        let expected_tec_monthly = dec!(0.0388466513010588);
        let expected_tec_yearly = dec!(0.5798553680415293);
        let expected_total_amount = dec!(1275.5756523433513);
        let expected_total_iof = dec!(20.12111293150685);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: dec!(100.0),
            requested_amount: dec!(1000.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0355),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 12);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_16() {
        let expected_contract_amount = dec!(44.420198301369865);
        let expected_contract_amount_without_tac = dec!(44.420198301369865);
        let expected_customer_amount = dec!(46.05063251213531);
        let expected_customer_debit_service_amount = dec!(1.630434210765445);
        let expected_debit_service = dec!(1.630434210765445);
        let expected_eir_monthly = dec!(0.03572522102931042);
        let expected_eir_yearly = dec!(0.5238233460625974);
        let expected_effective_interest_rate = dec!(0.03572522102931042);
        let expected_installment = 1;
        let expected_installment_amount = dec!(46.05063251213531);
        let expected_interest_rate = dec!(0.0355);
        let expected_mdr_amount = dec!(0.4414);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(0.4414);
        let expected_settled_to_merchant = dec!(43.6986);
        let expected_tec_monthly = dec!(0.041860605526351735);
        let expected_tec_yearly = dec!(0.6357442539210962);
        let expected_total_amount = dec!(46.05063251213531);
        let expected_total_iof = dec!(0.280198301369863);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: dec!(80.0),
            requested_amount: dec!(44.14),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 48,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0355),
        };

        let result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 1);

        let response = result.get(0).unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_calculate_payment_plan_test_17() {
        let expected_contract_amount = dec!(6614.613698630137);
        let expected_contract_amount_without_tac = dec!(6614.613698630137);
        let expected_customer_amount = dec!(800.2876026569718);
        let expected_customer_debit_service_amount = dec!(1388.2623279395814);
        let expected_debit_service = dec!(1388.2623279395814);
        let expected_eir_monthly = dec!(0.03609566937822084);
        let expected_eir_yearly = dec!(0.5303765471934176);
        let expected_effective_interest_rate = dec!(0.03609566937822084);
        let expected_installment = 10;
        let expected_installment_amount = dec!(800.2876026569718);
        let expected_interest_rate = dec!(0.0355);
        let expected_mdr_amount = dec!(65.0);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(65.0);
        let expected_settled_to_merchant = dec!(6435.0);
        let expected_tec_monthly = dec!(0.03892119788126003);
        let expected_tec_yearly = dec!(0.5812163308876515);
        let expected_total_amount = dec!(8002.876026569718);
        let expected_total_iof = dec!(114.61369863013698);

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: dec!(8145.322444824322),
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(6500.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 11,
            debit_service_percentage: 0,
            mdr: dec!(0.01),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.0355),
        };

        let mut result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 10);

        let response = result.pop().unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }

    #[test]
    fn test_go_case() {
        let expected_contract_amount = dec!(2781.4664277614843);
        let expected_contract_amount_without_tac = dec!(2781.4664277614843);
        let expected_customer_amount = dec!(2784.208338808703);
        let expected_customer_debit_service_amount = dec!(2.741911047218844);
        let expected_debit_service = dec!(2.741911047218844);
        let expected_eir_monthly = dec!(0.03011814319578998);
        let expected_eir_yearly = dec!(0.42772457911511363);
        let expected_effective_interest_rate = dec!(0.03011814319578998);
        let expected_installment = 1;
        let expected_installment_amount = dec!(2784.208338808703);
        let expected_interest_rate = dec!(0.029999999329447746);
        let expected_mdr_amount = dec!(83.12129814209416);
        let expected_merchant_debit_service_amount = Decimal::ZERO;
        let expected_merchant_total_amount = dec!(83.12129814209416);
        let expected_settled_to_merchant = dec!(2687.588701857906);
        let expected_tec_monthly = dec!(0.15696369491739448);
        let expected_tec_yearly = dec!(4.752237036590522);
        let expected_total_amount = dec!(2784.208338808703);
        let expected_total_iof = dec!(10.756427761484167);

        let first_payment_date = chrono::DateTime::from_timestamp_millis(1719025200000)
            .unwrap()
            .date_naive();

        let disbursement_date = chrono::DateTime::from_timestamp_millis(1718983261490)
            .unwrap()
            .date_naive();

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: dec!(100.0),
            requested_amount: dec!(770.71),
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: dec!(0.029999999329447746),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.003800000064074993),
            iof_percentage: dec!(0.029999999329447746),
            interest_rate: dec!(0.029999999329447746),
        };

        let result = SIMPLE.calculate_payment_plan(params).unwrap();

        assert_eq!(result.len(), 48);

        let response = result.get(0).unwrap();

        assert_eq!(response.contract_amount, expected_contract_amount);
        assert_eq!(
            response.contract_amount_without_tac,
            expected_contract_amount_without_tac
        );
        assert_eq!(response.customer_amount, expected_customer_amount);
        assert_eq!(
            response.customer_debit_service_amount,
            expected_customer_debit_service_amount
        );
        assert_eq!(response.debit_service, expected_debit_service);
        assert_eq!(response.eir_monthly, expected_eir_monthly);
        assert_eq!(response.eir_yearly, expected_eir_yearly);
        assert_eq!(
            response.effective_interest_rate,
            expected_effective_interest_rate
        );
        assert_eq!(response.installment, expected_installment);
        assert_eq!(response.installment_amount, expected_installment_amount);
        assert_eq!(response.interest_rate, expected_interest_rate);
        assert_eq!(response.mdr_amount, expected_mdr_amount);
        assert_eq!(
            response.merchant_debit_service_amount,
            expected_merchant_debit_service_amount
        );
        assert_eq!(
            response.merchant_total_amount,
            expected_merchant_total_amount
        );
        assert_eq!(response.settled_to_merchant, expected_settled_to_merchant);
        assert_eq!(response.tec_monthly, expected_tec_monthly);
        assert_eq!(response.tec_yearly, expected_tec_yearly);
        assert_eq!(response.total_amount, expected_total_amount);
        assert_eq!(response.total_iof, expected_total_iof);
    }
}

#[cfg(test)]
mod down_payment_test {
    use rust_decimal::{dec, Decimal};

    use crate::{calc::PaymentPlan, DownPaymentParams, Params};

    const SIMPLE: super::Simple = super::Simple {};

    #[allow(deprecated)]
    const PLAN_PARAM: Params = Params {
        disbursement_only_on_business_days: false,
        max_total_amount: Decimal::MAX,
        min_installment_amount: Decimal::ZERO,
        requested_amount: dec!(1000.0),
        first_payment_date: chrono::NaiveDate::from_ymd(2022, 06, 20),
        disbursement_date: chrono::NaiveDate::from_ymd(2022, 05, 20),
        installments: 1,
        debit_service_percentage: 0,
        mdr: dec!(0.01),
        tac_percentage: Decimal::ZERO,
        iof_overall: dec!(0.0038),
        iof_percentage: dec!(0.03),
        interest_rate: dec!(0.0355),
    };

    #[test]
    fn test_1_installment() {
        let down_payment = dec!(65.0);
        let min_installment_amount = dec!(100.0);
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = SIMPLE.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 1);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);
    }

    #[test]
    fn test_2_installments() {
        let down_payment = dec!(200.0);
        let min_installment_amount = dec!(100.0);
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = SIMPLE.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 2);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);

        let plans = &response.plans;
        let first_plan = plans.get(0).unwrap();

        // if the first payment is 20/06/2022, the first plan should be 20/07/2022 because we have 1 down payment to pay
        let plan_due_date = chrono::NaiveDate::from_ymd_opt(2022, 07, 20).unwrap();

        assert_eq!(first_plan.due_date, plan_due_date);

        let response = result.get(1).unwrap();

        assert_eq!(response.installment_amount, min_installment_amount);

        let plans = &response.plans;
        let first_plan = plans.get(0).unwrap();

        // if the first payment is 20/06/2022, the first plan should be 20/08/2022 because we have 2 down payments to pay
        let plan_due_date = chrono::NaiveDate::from_ymd_opt(2022, 08, 20).unwrap();

        assert_eq!(first_plan.due_date, plan_due_date);
    }

    #[test]
    fn test_3_installments() {
        let down_payment = dec!(300.0);
        let min_installment_amount = dec!(100.0);
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = SIMPLE.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 3);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);

        let response = result.get(1).unwrap();

        assert_eq!(response.installment_amount, dec!(150.0));

        let response = result.get(2).unwrap();

        assert_eq!(response.installment_amount, min_installment_amount);
    }

    #[test]
    fn test_4_installments() {
        let down_payment = dec!(400.0);
        let min_installment_amount = dec!(100.0);
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = SIMPLE.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 4);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);

        let response = result.get(1).unwrap();

        assert_eq!(response.installment_amount, dec!(200.0));

        let response = result.get(2).unwrap();

        assert_eq!(response.installment_amount, dec!(133.33333333333334));

        let response = result.get(3).unwrap();

        assert_eq!(response.installment_amount, min_installment_amount);
    }

    #[test]
    fn test_4_installments_max() {
        let down_payment = dec!(4000.0);
        let min_installment_amount = dec!(100.0);
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = SIMPLE.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 4);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);

        let response = result.get(1).unwrap();

        assert_eq!(response.installment_amount, dec!(2000.0));

        let response = result.get(2).unwrap();

        assert_eq!(response.installment_amount, dec!(1333.3333333333333));

        let response = result.get(3).unwrap();

        assert_eq!(response.installment_amount, dec!(1000.0));
    }
}
