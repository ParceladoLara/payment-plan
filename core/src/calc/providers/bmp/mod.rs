use amounts::calculate_amounts;
use inner_xirr::{eir::calculate_eir_monthly, prepare_xirr_params, tec::calculate_tec_monthly};
use iof::calculate_iof;
use prepare::{prepare_calculation, PreparedCalculation};

use crate::{
    calc::PaymentPlan,
    err::PaymentPlanError,
    util::{add_days, add_months},
    DownPaymentParams, DownPaymentResponse, Params, Response,
};

mod amounts;
mod inner_xirr;
mod iof;
mod prepare;

pub struct BMP;

impl PaymentPlan for BMP {
    fn calculate_payment_plan(&self, params: Params) -> Result<Vec<Response>, PaymentPlanError> {
        let prepared_calculations = prepare_calculation(params);
        let calculated = calculate(params, prepared_calculations);

        return calculated;
    }

    fn calculate_down_payment_plan(
        &self,
        params: DownPaymentParams,
    ) -> Result<Vec<DownPaymentResponse>, PaymentPlanError> {
        if params.requested_amount <= 0.0 {
            return Err(PaymentPlanError::InvalidRequestedAmount);
        }
        if params.installments == 0 {
            return Err(PaymentPlanError::InvalidNumberOfInstallments);
        }
        let mut resp = Vec::new();

        let mut base_params = params.params;
        let min_installment_amount = params.min_installment_amount;
        let down_payment_amount = params.requested_amount;
        let down_payment_first_payment_date = params.first_payment_date;

        // The start of the actual payment plan for 1 installment (we will update this in every iteration)
        let mut contract_start_date = add_days(down_payment_first_payment_date, 1);
        // The first payment date of the actual payment plan for 1 installment (we will update this in every iteration)
        let mut contract_first_payment_date = add_months(down_payment_first_payment_date, 1);

        for i in 1..=params.installments {
            base_params.first_payment_date = contract_first_payment_date;
            base_params.requested_date = contract_start_date;
            let installment_amount = down_payment_amount / i as f64;

            if installment_amount < min_installment_amount && i != 1 {
                break;
            }

            let plans = self.calculate_payment_plan(base_params)?;

            resp.push(DownPaymentResponse {
                first_payment_date: down_payment_first_payment_date,
                installment_amount,
                installment_quantity: i,
                plans,
                total_amount: down_payment_amount,
            });

            // Update the start date and first payment date by a month for the next iteration
            contract_start_date = add_months(contract_start_date, 1);
            contract_first_payment_date = add_months(contract_first_payment_date, 1);
        }

        return Ok(resp);
    }
}

fn calculate(
    params: Params,
    prepared_calculations: Vec<PreparedCalculation>,
) -> Result<Vec<Response>, PaymentPlanError> {
    if params.requested_amount <= 0.0 {
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

    let customer_debit_service_proportion = 1.0 - debit_service_percentage as f64 / 100.0;
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
            prepared_calculation.installment as f64,
        );

        let amounts = calculate_amounts(
            params,
            prepared_calculation.accumulated_days_index,
            prepared_calculation.installment as f64,
            customer_debit_service_proportion,
            total_iof,
        );

        let (eir_params, tec_params) = prepare_xirr_params(
            prepared_calculation.installment,
            &prepared_calculations,
            amounts.calculation_basis_for_effective_interest_rate,
            amounts.customer_amount,
        );

        let eir_monthly =
            calculate_eir_monthly(params, eir_params, customer_debit_service_proportion)?;

        let eir_yearly = (1.0 + eir_monthly).powf(12.0) - 1.0;

        let tec_monthly = calculate_tec_monthly(params, tec_params)?;

        let tec_yearly = (1.0 + tec_monthly).powf(12.0) - 1.0;

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

    use crate::{calc::PaymentPlan, Params};

    const BMP: super::BMP = super::BMP {};

    #[test]
    fn test_calculate_payment_plan_test_0() {
        let expected_contract_amount = 9037.318869753424;
        let expected_contract_amount_without_tac = 9037.318869753424;
        let expected_customer_amount = 499.1987614851067;
        let expected_customer_debit_service_amount = 2943.451405889136;
        let expected_debit_service = 2943.451405889136;
        let expected_eir_monthly = 0.024085088183680048;
        let expected_eir_yearly = 0.33055401101326365;
        let expected_effective_interest_rate = 0.024085088183680048; // 0.0235 in node
        let expected_installment = 24;
        let expected_installment_amount = 499.1987614851067;
        let expected_interest_rate = 0.0235;
        let expected_mdr_amount = 440.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 440.0;
        let expected_settled_to_merchant = 8360.0;
        let expected_tec_monthly = 0.025868426671143974;
        let expected_tec_yearly = 0.3586261331729559;
        let expected_total_amount = 11980.77027564256;
        let expected_total_iof = 237.3188697534247;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 8800.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 18).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 18).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0235,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 6148.387557205479;
        let expected_contract_amount_without_tac = 6148.387557205479;
        let expected_customer_amount = 430.32094244906153;
        let expected_customer_debit_service_amount = 1597.389406877628;
        let expected_debit_service = 1597.389406877628;
        let expected_eir_monthly = 0.02557918934592962;
        let expected_eir_yearly = 0.3540365786122326;
        let expected_effective_interest_rate = 0.02557918934592962; // 0.025 in node
        let expected_installment = 18;
        let expected_installment_amount = 430.32094244906153;
        let expected_interest_rate = 0.025;
        let expected_mdr_amount = 300.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 300.0;
        let expected_settled_to_merchant = 5700.0;
        let expected_tec_monthly = 0.027793563251085507;
        let expected_tec_yearly = 0.38953894087787666;
        let expected_total_amount = 7745.7769640831075;
        let expected_total_iof = 148.38755720547942;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 6000.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 18).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 17).unwrap(),
            installments: 18,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.025,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 1326.1754959452055;
        let expected_contract_amount_without_tac = 1326.1754959452055;
        let expected_customer_amount = 128.47186120970176;
        let expected_customer_debit_service_amount = 215.4868385712157;
        let expected_debit_service = 215.4868385712157;
        let expected_eir_monthly = 0.023954074195358555;
        let expected_eir_yearly = 0.3285127909894192;
        let expected_effective_interest_rate = 0.023954074195358555; // 0.0235 in node
        let expected_installment = 12;
        let expected_installment_amount = 128.47186120970176;
        let expected_interest_rate = 0.0235;
        let expected_mdr_amount = 65.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 65.0;
        let expected_settled_to_merchant = 1235.0;
        let expected_tec_monthly = 0.026733709153886398;
        let expected_tec_yearly = 0.37244152356319127;
        let expected_total_amount = 1541.6623345164212;
        let expected_total_iof = 26.17549594520548;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1300.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 21).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 21).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0235,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 1626.1424272328768;
        let expected_contract_amount_without_tac = 1626.1424272328768;
        let expected_customer_amount = 203.44470455102888;
        let expected_customer_debit_service_amount = 204.85991372638327;
        let expected_debit_service = 204.85991372638327;
        let expected_eir_monthly = 0.024380237604045174;
        let expected_eir_yearly = 0.33516302665077946;
        let expected_effective_interest_rate = 0.024380237604045174; // 0.024 in node
        let expected_installment = 9;
        let expected_installment_amount = 203.44470455102888;
        let expected_interest_rate = 0.024;
        let expected_mdr_amount = 80.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 80.0;
        let expected_settled_to_merchant = 1520.0;
        let expected_tec_monthly = 0.027386037131249097;
        let expected_tec_yearly = 0.3829418169609544; //  0.3829418169609542 in node
        let expected_total_amount = 1831.00234095926;
        let expected_total_iof = 26.142427232876713;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1600.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 29).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 29).unwrap(),
            installments: 9,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.024,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 1016.2107967945205;
        let expected_contract_amount_without_tac = 1016.2107967945205;
        let expected_customer_amount = 126.67946907687234;
        let expected_customer_debit_service_amount = 123.9044248973305;
        let expected_debit_service = 123.9044248973305;
        let expected_eir_monthly = 0.023869886439737753;
        let expected_eir_yearly = 0.3272026469033864; //0.3272026469033862 in node
        let expected_effective_interest_rate = 0.023869886439737753; // 0.0235 in node
        let expected_installment = 9;
        let expected_installment_amount = 126.67946907687234;
        let expected_interest_rate = 0.0235;
        let expected_mdr_amount = 50.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 50.0;
        let expected_settled_to_merchant = 950.0;
        let expected_tec_monthly = 0.026891280795923622;
        let expected_tec_yearly = 0.374971182359072;
        let expected_total_amount = 1140.115221691851;
        let expected_total_iof = 16.210796794520547;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1000.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 08).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 10).unwrap(),
            installments: 9,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0235,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 4703.573142849315;
        let expected_contract_amount_without_tac = 4703.573142849315;
        let expected_customer_amount = 294.61599268872135;
        let expected_customer_debit_service_amount = 2367.2106816799965;
        let expected_debit_service = 2367.2106816799965;
        let expected_eir_monthly = 0.03574394430986261;
        let expected_eir_yearly = 0.5241539411857024;
        let expected_effective_interest_rate = 0.03574394430986261; // 0.0349 in node
        let expected_installment = 24;
        let expected_installment_amount = 294.6159926887213; // 294.61599268872135 in node
        let expected_interest_rate = 0.0349;
        let expected_mdr_amount = 45.800000000000004;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 45.800000000000004;
        let expected_settled_to_merchant = 4534.2;
        let expected_tec_monthly = 0.03740934806328888;
        let expected_tec_yearly = 0.5538242142983534;
        let expected_total_amount = 7070.7838245293115;
        let expected_total_iof = 123.57314284931509;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 4580.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 05).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 04).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0349,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 1530.1828767123288;
        let expected_contract_amount_without_tac = 1530.1828767123288;
        let expected_customer_amount = 149.59889365154822; // 149.59889365154817 in node
        let expected_customer_debit_service_amount = 265.0038471062499; // 265.0038471062492 in node
        let expected_debit_service = 265.0038471062499; // 265.0038471062492 in node
        let expected_eir_monthly = 0.025481551205442488;
        let expected_eir_yearly = 0.3524904893931555;
        let expected_effective_interest_rate = 0.025481551205442488; // 0.025 in node
        let expected_installment = 12;
        let expected_installment_amount = 149.59889365154822; // 149.59889365154817 in node
        let expected_interest_rate = 0.025;
        let expected_mdr_amount = 75.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 75.0;
        let expected_settled_to_merchant = 1425.0;
        let expected_tec_monthly = 0.02824733087479081; // 0.028247330874790588 in node
        let expected_tec_yearly = 0.396918568026396; // 0.39691856802639225 in node
        let expected_total_amount = 1795.1867238185787; // 1795.186723818578 in node
        let expected_total_iof = 30.182876712328767;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1500.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 09).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 09).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.025,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 2936.563583452055;
        let expected_contract_amount_without_tac = 2936.563583452055;
        let expected_customer_amount = 552.4322553512001;
        let expected_customer_debit_service_amount = 378.0299486551454;
        let expected_debit_service = 378.0299486551454;
        let expected_eir_monthly = 0.035429014326330055;
        let expected_eir_yearly = 0.5186019914133586;
        let expected_effective_interest_rate = 0.035429014326330055; // 0.035 in node
        let expected_installment = 6;
        let expected_installment_amount = 552.4322553512001;
        let expected_interest_rate = 0.035;
        let expected_mdr_amount = 86.71000000000001;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 86.71000000000001;
        let expected_settled_to_merchant = 2813.29;
        let expected_tec_monthly = 0.03875204347989669;
        let expected_tec_yearly = 0.5781297023988077;
        let expected_total_amount = 3314.5935321072;
        let expected_total_iof = 36.56358345205479;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 2900.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 30).unwrap(),
            installments: 6,
            debit_service_percentage: 0,
            mdr: 0.029900000000000003,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.035,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 3868.0712711232877;
        let expected_contract_amount_without_tac = 3868.0712711232877;
        let expected_customer_amount = 222.76797845520915;
        let expected_customer_debit_service_amount = 1478.3602118017322;
        let expected_debit_service = 1478.3602118017322;
        let expected_eir_monthly = 0.029698339097185666;
        let expected_eir_yearly = 0.42075811960150045;
        let expected_effective_interest_rate = 0.029698339097185666; // 0.0297 in node
        let expected_installment = 24;
        let expected_installment_amount = 222.76797845520915;
        let expected_interest_rate = 0.028999999999999998;
        let expected_mdr_amount = 112.71104000000001;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 112.71104000000001;
        let expected_settled_to_merchant = 3656.8889599999998;
        let expected_tec_monthly = 0.031514336661548015;
        let expected_tec_yearly = 0.4511196449073398;
        let expected_total_amount = 5346.43148292502;
        let expected_total_iof = 98.47127112328764;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 3769.6,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 10).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: 0.029900000000000003,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.028999999999999998,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 6249.888847589041;
        let expected_contract_amount_without_tac = 6249.888847589041;
        let expected_customer_amount = 2209.190934226094;
        let expected_customer_debit_service_amount = 377.6839550892415;
        let expected_debit_service = 377.6839550892415;
        let expected_eir_monthly = 0.03517937087855172;
        let expected_eir_yearly = 0.5142141670632188;
        let expected_effective_interest_rate = 0.03517937087855172; // 0.0349 in node
        let expected_installment = 3;
        let expected_installment_amount = 2209.190934226094;
        let expected_interest_rate = 0.0349;
        let expected_mdr_amount = 62.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 62.0;
        let expected_settled_to_merchant = 6138.0;
        let expected_tec_monthly = 0.039799198101799105;
        let expected_tec_yearly = 0.5973266517177596;
        let expected_total_amount = 6627.572802678283;
        let expected_total_iof = 49.8888475890411;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 6200.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 25).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 04).unwrap(),
            installments: 3,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0349,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 2739.974829369863;
        let expected_contract_amount_without_tac = 2739.974829369863;
        let expected_customer_amount = 269.5154467446678;
        let expected_customer_debit_service_amount = 494.21053156615125;
        let expected_debit_service = 494.21053156615125;
        let expected_eir_monthly = 0.029518034828695416;
        let expected_eir_yearly = 0.41777562837546256;
        let expected_effective_interest_rate = 0.029518034828695416; // 0.029 in node
        let expected_installment = 12;
        let expected_installment_amount = 269.5154467446678;
        let expected_interest_rate = 0.029;
        let expected_mdr_amount = 80.43399000000001;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 80.43399000000001;
        let expected_settled_to_merchant = 2609.66601;
        let expected_tec_monthly = 0.03237874905455129;
        let expected_tec_yearly = 0.46577960649402206;
        let expected_total_amount = 3234.185360936014;
        let expected_total_iof = 49.874829369863015;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 2690.1,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 15).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 04).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: 0.029900000000000003,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.029,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 1099.9854739726027;
        let expected_contract_amount_without_tac = 1099.9854739726027;
        let expected_customer_amount = 290.2409490880433;
        let expected_customer_debit_service_amount = 60.978322379570436;
        let expected_debit_service = 60.978322379570436;
        let expected_eir_monthly = 0.021714523316856305;
        let expected_eir_yearly = 0.2940611563328235;
        let expected_effective_interest_rate = 0.021714523316856305; // 0.0215 in node
        let expected_installment = 4;
        let expected_installment_amount = 290.2409490880433;
        let expected_interest_rate = 0.0215;
        let expected_mdr_amount = 10.89;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 10.89;
        let expected_settled_to_merchant = 1078.11;
        let expected_tec_monthly = 0.02557650331018846;
        let expected_tec_yearly = 0.3539940238690005; // 0.3539940238690007 in node
        let expected_total_amount = 1160.9637963521732;
        let expected_total_iof = 10.98547397260274;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1089.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 29).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 29).unwrap(),
            installments: 4,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0215,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 1782.8928;
        let expected_contract_amount_without_tac = 1782.8928;
        let expected_customer_amount = 204.24811158092817;
        let expected_customer_debit_service_amount = 259.5883158092816;
        let expected_debit_service = 259.5883158092816;
        let expected_eir_monthly = 0.025425157547064314; //0.025425157547064092 in node
        let expected_eir_yearly = 0.3515982394442816; // 0.35159823944427804 in node
        let expected_effective_interest_rate = 0.025425157547064314; // 0.025 in node
        let expected_installment = 10;
        let expected_installment_amount = 204.24811158092817;
        let expected_interest_rate = 0.025;
        let expected_mdr_amount = 87.60000000000001;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 87.60000000000001;
        let expected_settled_to_merchant = 1664.4;
        let expected_tec_monthly = 0.028331897318861987;
        let expected_tec_yearly = 0.39829783796886464;
        let expected_total_amount = 2042.4811158092816;
        let expected_total_iof = 30.8928;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1752.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 16).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 16).unwrap(),
            installments: 10,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.025,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 4107.87339030137;
        let expected_contract_amount_without_tac = 4107.87339030137;
        let expected_customer_amount = 221.81089997440975;
        let expected_customer_debit_service_amount = 1215.5882090844643;
        let expected_debit_service = 1215.5882090844643;
        let expected_eir_monthly = 0.02203837782073359;
        let expected_eir_yearly = 0.2989919143131987;
        let expected_effective_interest_rate = 0.02203837782073359; // 0.0215 in node
        let expected_installment = 24;
        let expected_installment_amount = 221.81089997440975;
        let expected_interest_rate = 0.0215;
        let expected_mdr_amount = 40.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 40.0;
        let expected_settled_to_merchant = 3960.0;
        let expected_tec_monthly = 0.02384436103270393;
        let expected_tec_yearly = 0.3268056502522887;
        let expected_total_amount = 5323.461599385834;
        let expected_total_iof = 107.8733903013699;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 4000.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 14).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 14).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0215,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 6622.710151424658;
        let expected_contract_amount_without_tac = 6622.710151424658;
        let expected_customer_amount = 740.574767711302;
        let expected_customer_debit_service_amount = 1523.6122933996642;
        let expected_debit_service = 1523.6122933996642;
        let expected_eir_monthly = 0.036134558462701305; // 0.03613455846270108 in node
        let expected_eir_yearly = 0.5310659881278654; //0.5310659881278617 in node
        let expected_effective_interest_rate = 0.036134558462701305; // 0.0355 in node
        let expected_installment = 11;
        let expected_installment_amount = 740.574767711302;
        let expected_interest_rate = 0.0355;
        let expected_mdr_amount = 65.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 65.0;
        let expected_settled_to_merchant = 6435.0;
        let expected_tec_monthly = 0.0388795124728023;
        let expected_tec_yearly = 0.5804551670400413;
        let expected_total_amount = 8146.322444824322;
        let expected_total_iof = 122.71015142465752;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 6500.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 11,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0355,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 1020.1211129315069;
        let expected_contract_amount_without_tac = 1020.1211129315069;
        let expected_customer_amount = 106.29797102861261;
        let expected_customer_debit_service_amount = 255.45453941184445;
        let expected_debit_service = 255.45453941184445;
        let expected_eir_monthly = 0.03617300584122107;
        let expected_eir_yearly = 0.531747878195636;
        let expected_effective_interest_rate = 0.03617300584122107;
        let expected_installment = 12;
        let expected_installment_amount = 106.29797102861261;
        let expected_interest_rate = 0.0355;
        let expected_mdr_amount = 10.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 10.0;
        let expected_settled_to_merchant = 990.0;
        let expected_tec_monthly = 0.0388466513010588;
        let expected_tec_yearly = 0.5798553680415293;
        let expected_total_amount = 1275.5756523433513;
        let expected_total_iof = 20.12111293150685;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 1000.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0355,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 44.420198301369865;
        let expected_contract_amount_without_tac = 44.420198301369865;
        let expected_customer_amount = 46.05063251213531;
        let expected_customer_debit_service_amount = 1.630434210765445;
        let expected_debit_service = 1.630434210765445;
        let expected_eir_monthly = 0.03572522102931042;
        let expected_eir_yearly = 0.5238233460625974;
        let expected_effective_interest_rate = 0.03572522102931042;
        let expected_installment = 1;
        let expected_installment_amount = 46.05063251213531;
        let expected_interest_rate = 0.0355;
        let expected_mdr_amount = 0.4414;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 0.4414;
        let expected_settled_to_merchant = 43.6986;
        let expected_tec_monthly = 0.041860605526351735;
        let expected_tec_yearly = 0.6357442539210962;
        let expected_total_amount = 46.05063251213531;
        let expected_total_iof = 0.280198301369863;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 80.0,
            requested_amount: 44.14,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0355,
        };

        let result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 6614.613698630137;
        let expected_contract_amount_without_tac = 6614.613698630137;
        let expected_customer_amount = 800.2876026569718;
        let expected_customer_debit_service_amount = 1388.2623279395814;
        let expected_debit_service = 1388.2623279395814;
        let expected_eir_monthly = 0.03609566937822084;
        let expected_eir_yearly = 0.5303765471934176;
        let expected_effective_interest_rate = 0.03609566937822084;
        let expected_installment = 10;
        let expected_installment_amount = 800.2876026569718;
        let expected_interest_rate = 0.0355;
        let expected_mdr_amount = 65.0;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 65.0;
        let expected_settled_to_merchant = 6435.0;
        let expected_tec_monthly = 0.03892119788126003;
        let expected_tec_yearly = 0.5812163308876515;
        let expected_total_amount = 8002.876026569718;
        let expected_total_iof = 114.61369863013698;

        let params = Params {
            max_total_amount: 8145.322444824322,
            min_installment_amount: 0.0,
            requested_amount: 6500.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 20).unwrap(),
            installments: 11,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0355,
        };

        let mut result = BMP.calculate_payment_plan(params).unwrap();

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
        let expected_contract_amount = 2781.4664277614843;
        let expected_contract_amount_without_tac = 2781.4664277614843;
        let expected_customer_amount = 2784.208338808703;
        let expected_customer_debit_service_amount = 2.741911047218844;
        let expected_debit_service = 2.741911047218844;
        let expected_eir_monthly = 0.03011814319578998;
        let expected_eir_yearly = 0.42772457911511363;
        let expected_effective_interest_rate = 0.03011814319578998;
        let expected_installment = 1;
        let expected_installment_amount = 2784.208338808703;
        let expected_interest_rate = 0.029999999329447746;
        let expected_mdr_amount = 83.12129814209416;
        let expected_merchant_debit_service_amount = 0.0;
        let expected_merchant_total_amount = 83.12129814209416;
        let expected_settled_to_merchant = 2687.588701857906;
        let expected_tec_monthly = 0.15696369491739448;
        let expected_tec_yearly = 4.752237036590522;
        let expected_total_amount = 2784.208338808703;
        let expected_total_iof = 10.756427761484167;

        let first_payment_date = chrono::DateTime::from_timestamp_millis(1719025200000)
            .unwrap()
            .date_naive();

        let requested_date = chrono::DateTime::from_timestamp_millis(1718983261490)
            .unwrap()
            .date_naive();

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 2770.71,
            first_payment_date,
            requested_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.029999999329447746,
            tac_percentage: 0.0,
            iof_overall: 0.003800000064074993,
            iof_percentage: 0.029999999329447746,
            interest_rate: 0.029999999329447746,
        };

        let result = BMP.calculate_payment_plan(params).unwrap();

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
    use crate::{calc::PaymentPlan, DownPaymentParams, Params};

    const BMP: super::BMP = super::BMP {};

    #[allow(deprecated)]
    const PLAN_PARAM: Params = Params {
        max_total_amount: f64::MAX,
        min_installment_amount: 0.0,
        requested_amount: 1000.0,
        first_payment_date: chrono::NaiveDate::from_ymd(2022, 06, 20),
        requested_date: chrono::NaiveDate::from_ymd(2022, 05, 20),
        installments: 1,
        debit_service_percentage: 0,
        mdr: 0.01,
        tac_percentage: 0.0,
        iof_overall: 0.0038,
        iof_percentage: 0.03,
        interest_rate: 0.0355,
    };

    #[test]
    fn test_1_installment() {
        let down_payment = 65.0;
        let min_installment_amount = 100.0;
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = BMP.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 1);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);
    }

    #[test]
    fn test_2_installments() {
        let down_payment = 200.0;
        let min_installment_amount = 100.0;
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = BMP.calculate_down_payment_plan(params).unwrap();

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
        let down_payment = 300.0;
        let min_installment_amount = 100.0;
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = BMP.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 3);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);

        let response = result.get(1).unwrap();

        assert_eq!(response.installment_amount, 150.0);

        let response = result.get(2).unwrap();

        assert_eq!(response.installment_amount, min_installment_amount);
    }

    #[test]
    fn test_4_installments() {
        let down_payment = 400.0;
        let min_installment_amount = 100.0;
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = BMP.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 4);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);

        let response = result.get(1).unwrap();

        assert_eq!(response.installment_amount, 200.0);

        let response = result.get(2).unwrap();

        assert_eq!(response.installment_amount, 133.33333333333334);

        let response = result.get(3).unwrap();

        assert_eq!(response.installment_amount, min_installment_amount);
    }

    #[test]
    fn test_4_installments_max() {
        let down_payment = 4000.0;
        let min_installment_amount = 100.0;
        let installments = 4;

        let params = DownPaymentParams {
            params: PLAN_PARAM,
            requested_amount: down_payment,
            min_installment_amount,
            installments,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 20).unwrap(),
        };

        let result = BMP.calculate_down_payment_plan(params).unwrap();

        assert_eq!(result.len(), 4);

        let response = result.get(0).unwrap();

        assert_eq!(response.installment_amount, down_payment);

        let response = result.get(1).unwrap();

        assert_eq!(response.installment_amount, 2000.0);

        let response = result.get(2).unwrap();

        assert_eq!(response.installment_amount, 1333.3333333333333);

        let response = result.get(3).unwrap();

        assert_eq!(response.installment_amount, 1000.0);
    }
}
