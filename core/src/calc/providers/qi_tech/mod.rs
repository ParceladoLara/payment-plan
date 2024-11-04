use crate::{
    calc::{
        inner_xirr::{eir::calculate_eir_monthly, prepare_xirr_params, tec::calculate_tec_monthly},
        PaymentPlan,
    },
    err::PaymentPlanError,
    plan::Params,
    plan::Response,
    util::round_decimal_cases,
};

const POTENCY: f64 = 0.0027397260273972603; // 1/365
const CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE: f64 = 0.08333333333333333; // 30/360

const NUM_OF_RUNS: u32 = 7;

mod amounts;
mod installment;
mod iof;

#[derive(Default, Debug, Clone, Copy)]
struct QiTechParams {
    params: Params,
    main_value: f64,
    daily_interest_rate: f64,
}

pub struct QiTech;

impl PaymentPlan for QiTech {
    fn calculate_payment_plan(
        &self,
        mut params: Params,
    ) -> Result<Vec<Response>, PaymentPlanError> {
        if params.requested_amount <= 0.0 {
            return Err(PaymentPlanError::InvalidRequestedAmount);
        }
        if params.installments == 0 {
            return Err(PaymentPlanError::InvalidNumberOfInstallments);
        }
        let mut response = Vec::with_capacity(params.installments as usize);

        let interest_rate = params.interest_rate;

        let min_installment_amount = params.min_installment_amount;
        let max_total_amount = params.max_total_amount;

        let annual_interest_rate = (1.0 + interest_rate).powf(12.0) - 1.0;
        let daily_interest_rate = (1.0 + annual_interest_rate).powf(POTENCY) - 1.0;

        let daily_interest_rate = round_decimal_cases(daily_interest_rate, 8);

        let main_value = params.requested_amount;

        for i in 1..=params.installments {
            params.installments = i;
            let params = QiTechParams {
                params,
                main_value,
                daily_interest_rate,
            };

            let resp = calc(params)?;
            if resp.installment_amount < min_installment_amount {
                break;
            }
            if resp.total_amount > max_total_amount {
                break;
            }
            response.push(resp);
        }

        Ok(response)
    }
}

fn calc(mut params: QiTechParams) -> Result<Response, PaymentPlanError> {
    let debit_service_percentage = params.params.debit_service_percentage;
    let requested_amount = params.params.requested_amount;

    let mut data = installment::calc(&params);
    let mut iof = iof::calc(&params, &data);

    let debit_service = data.amount;

    params.main_value = requested_amount + iof;
    for _ in 1..NUM_OF_RUNS {
        data = installment::calc(&params);
        iof = iof::calc(&params, &data);
        params.main_value = requested_amount + iof;
    }

    let mut data = installment::calc(&params);

    let customer_amount = data.amount;

    let installment_amount = data.amount;
    let installments = params.params.installments;
    let total_amount = installment_amount * installments as f64;
    let total_amount = round_decimal_cases(total_amount, 2);
    let contract_amount = params.params.requested_amount + iof;
    let accumulated_days = data.accumulated_days.pop().unwrap();
    let accumulated_days_index = data.accumulated_factor;
    let customer_debit_service_proportion = 1.0 - debit_service_percentage as f64 / 100.0;

    let params = params.params;

    let amounts = amounts::calc(
        params,
        installments as f64,
        customer_debit_service_proportion,
        iof,
        total_amount,
    );

    let (eir_params, tec_params) = prepare_xirr_params(
        installments,
        &data.due_dates,
        debit_service,
        customer_amount,
    );

    let eir_monthly = calculate_eir_monthly(
        params,
        eir_params,
        customer_debit_service_proportion,
        CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE,
    )?;

    let eir_yearly = (1.0 + eir_monthly).powf(12.0) - 1.0;

    let tec_monthly = calculate_tec_monthly(
        params,
        tec_params,
        CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE,
    )?;

    let tec_yearly = (1.0 + tec_monthly).powf(12.0) - 1.0;

    let eir_monthly = round_decimal_cases(eir_monthly, 4);
    let tec_monthly = round_decimal_cases(tec_monthly, 4);

    let eir_yearly = round_decimal_cases(eir_yearly, 6);
    let tec_yearly = round_decimal_cases(tec_yearly, 6);

    let resp = Response {
        contract_amount,
        total_amount,
        installment_amount,
        installment: installments,
        due_date: data.last_due_date,
        accumulated_days,
        interest_rate: params.interest_rate,
        iof_percentage: params.iof_percentage,
        overall_iof: params.iof_overall,
        total_iof: iof,
        days_index: data.factor,
        accumulated_days_index,
        calculation_basis_for_effective_interest_rate: amounts
            .calculation_basis_for_effective_interest_rate,
        customer_amount,
        customer_debit_service_amount: amounts.customer_debit_service_amount,
        debit_service: amounts.debit_service,
        mdr_amount: amounts.mdr_amount,
        settled_to_merchant: amounts.settled_to_merchant,
        merchant_debit_service_amount: amounts.merchant_debit_service_amount,
        merchant_total_amount: amounts.merchant_total_amount,
        eir_yearly,
        tec_yearly,
        eir_monthly,
        tec_monthly,
        effective_interest_rate: eir_monthly,
        total_effective_cost: tec_monthly,
        ..Default::default()
    };

    return Ok(resp);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::plan::Params;

    #[test]
    fn test_qi_tech() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 12853.43,
            first_payment_date,
            requested_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let qi_tech = QiTech;
        let mut resp = qi_tech.calculate_payment_plan(params).unwrap();
        assert_eq!(resp.len(), 48);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2028, 10, 23).unwrap();

        let expected = Response {
            installment: 48,
            due_date: expected_due_date,
            accumulated_days: 1461,
            days_index: 0.191588419996811,
            accumulated_days_index: 23.089551521795638,
            interest_rate: 0.035,
            installment_amount: 575.19,
            installment_amount_without_tac: 0.0,
            total_amount: 27609.12,
            debit_service: 14328.21,
            customer_debit_service_amount: 14328.21,
            customer_amount: 575.19,
            calculation_basis_for_effective_interest_rate: 566.2841666666667,
            merchant_debit_service_amount: 0.0,
            merchant_total_amount: 642.6715,
            settled_to_merchant: 12210.7585,
            mdr_amount: 642.6715,
            effective_interest_rate: 0.035,
            total_effective_cost: 0.0369,
            eir_yearly: 0.511076,
            tec_yearly: 0.544336,
            eir_monthly: 0.035,
            tec_monthly: 0.0369,
            total_iof: 427.48,
            contract_amount: 13280.91,
            contract_amount_without_tac: 0.0,
            tac_amount: 0.0,
            iof_percentage: 0.000082,
            overall_iof: 0.0038,
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_qi_tech_wrong_amount() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 0.0,
            first_payment_date,
            requested_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let qi_tech = QiTech;
        let resp = qi_tech.calculate_payment_plan(params);
        assert_eq!(resp.is_err(), true);

        assert_eq!(resp.unwrap_err(), PaymentPlanError::InvalidRequestedAmount);
    }

    #[test]
    fn test_qi_tech_wrong_installments() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 12853.43,
            first_payment_date,
            requested_date,
            installments: 0,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let qi_tech = QiTech;
        let resp = qi_tech.calculate_payment_plan(params);
        assert_eq!(resp.is_err(), true);

        assert_eq!(
            resp.unwrap_err(),
            PaymentPlanError::InvalidNumberOfInstallments
        );
    }

    #[test]
    fn test_qi_tech_min_installment_amount_reached() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 200.43,
            first_payment_date,
            requested_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let qi_tech = QiTech;

        let mut resp = qi_tech.calculate_payment_plan(params).unwrap();

        assert_eq!(resp.len(), 2);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap();

        let expected = Response {
            installment: 2,
            due_date: expected_due_date,
            accumulated_days: 61,
            days_index: 0.93333450122513,
            accumulated_days_index: 1.898880713036613,
            interest_rate: 0.035,
            installment_amount: 106.36,
            installment_amount_without_tac: 0.0,
            total_amount: 212.72,
            debit_service: 10.759999999999993,
            customer_debit_service_amount: 10.759999999999993,
            customer_amount: 106.36,
            calculation_basis_for_effective_interest_rate: 105.595,
            merchant_debit_service_amount: 0.0,
            merchant_total_amount: 10.021500000000001,
            settled_to_merchant: 190.4085,
            mdr_amount: 10.021500000000001,
            effective_interest_rate: 0.035,
            total_effective_cost: 0.0403,
            eir_yearly: 0.510882,
            tec_yearly: 0.605951,
            eir_monthly: 0.035,
            tec_monthly: 0.0403,
            total_iof: 1.53,
            contract_amount: 201.96,
            contract_amount_without_tac: 0.0,
            tac_amount: 0.0,
            iof_percentage: 0.000082,
            overall_iof: 0.0038,
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_qi_tech_max_amount_reached() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            max_total_amount: 2400.43,
            min_installment_amount: 100.0,
            requested_amount: 2000.43,
            first_payment_date,
            requested_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let qi_tech = QiTech;

        let mut resp = qi_tech.calculate_payment_plan(params).unwrap();

        assert_eq!(resp.len(), 8);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2025, 6, 23).unwrap();

        let expected = Response {
            installment: 8,
            due_date: expected_due_date,
            accumulated_days: 243,
            days_index: 0.759697105502466,
            accumulated_days_index: 6.873654600884539,
            interest_rate: 0.035,
            installment_amount: 295.6,
            installment_amount_without_tac: 0.0,
            total_amount: 2364.8,
            debit_service: 332.92000000000013,
            customer_debit_service_amount: 332.92000000000013,
            customer_amount: 295.6,
            calculation_basis_for_effective_interest_rate: 291.66875000000005,
            merchant_debit_service_amount: 0.0,
            merchant_total_amount: 100.0215,
            settled_to_merchant: 1900.4085,
            mdr_amount: 100.0215,
            effective_interest_rate: 0.035,
            total_effective_cost: 0.0387,
            eir_yearly: 0.511091,
            tec_yearly: 0.578041,
            eir_monthly: 0.035,
            tec_monthly: 0.0387,
            total_iof: 31.45,
            contract_amount: 2031.88,
            contract_amount_without_tac: 0.0,
            tac_amount: 0.0,
            iof_percentage: 0.000082,
            overall_iof: 0.0038,
        };

        assert_eq!(resp, expected);
    }
}
