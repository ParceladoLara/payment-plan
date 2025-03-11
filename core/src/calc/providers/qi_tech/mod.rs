use installment::InstallmentData;

use crate::{
    calc::{
        inner_xirr::{eir::calculate_eir_monthly, prepare_xirr_params, tec::calculate_tec_monthly},
        PaymentPlan,
    },
    err::PaymentPlanError,
    util::{get_next_business_day, round_decimal_cases},
    Params, Response,
};

const POTENCY: f64 = 0.003968253968253968; // 1/252
const CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE: f64 = 0.08333333333333333; // 1/12

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

        if params.disbursement_only_on_business_days {
            //Change the base date to the next business day
            params.requested_date = get_next_business_day(params.requested_date);
        }

        let mut response = Vec::with_capacity(params.installments as usize);

        let interest_rate = params.interest_rate;

        let min_installment_amount = params.min_installment_amount;
        let max_total_amount = params.max_total_amount;

        let annual_interest_rate = (1.0 + interest_rate).powf(12.0) - 1.0;
        let daily_interest_rate = (1.0 + annual_interest_rate).powf(POTENCY) - 1.0;

        let daily_interest_rate = round_decimal_cases(daily_interest_rate, 10);
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

    let iof = round_decimal_cases(iof, 2);
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

    let present_value = present_value(installment_amount, &data, params.interest_rate);
    let present_value = round_decimal_cases(present_value, 2);
    let pre_disbursement_amount = present_value - iof;
    let pre_disbursement_amount = round_decimal_cases(pre_disbursement_amount, 2);
    let diff = pre_disbursement_amount - requested_amount;
    let diff = round_decimal_cases(diff, 2);

    let paid_iof = iof + diff;
    let paid_iof = round_decimal_cases(paid_iof, 2);

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
        disbursement_date: params.requested_date,
        pre_disbursement_amount,
        paid_total_iof: paid_iof,
        paid_contract_amount: requested_amount + paid_iof,
        ..Default::default()
    };

    return Ok(resp);
}

fn present_value(
    installment_amount: f64,
    installments: &InstallmentData,
    interest_rate: f64,
) -> f64 {
    let mut present_value = 0.0;
    let annual_interest_rate = (1.0 + interest_rate).powf(12.0);
    for days in &installments.accumulated_business_days {
        let days = *days as f64;

        let days_diff = days / 252.0;
        let potency = annual_interest_rate.powf(days_diff);
        let installment_value = installment_amount / potency;
        present_value += installment_value;
    }
    return present_value;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Params;

    #[test]
    fn test_qi_tech() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
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
            disbursement_date: requested_date,
            due_date: expected_due_date,
            accumulated_days: 1461,
            days_index: 0.091322653312291,
            accumulated_days_index: 17.78149283013243,
            interest_rate: 0.035,
            installment_amount: 747.31,
            installment_amount_without_tac: 0.0,
            total_amount: 35870.88,
            debit_service: 22582.639999999996,
            customer_debit_service_amount: 22582.639999999996,
            customer_amount: 747.31,
            calculation_basis_for_effective_interest_rate: 738.2514583333332,
            merchant_debit_service_amount: 0.0,
            merchant_total_amount: 642.6715,
            settled_to_merchant: 12210.7585,
            mdr_amount: 642.6715,
            effective_interest_rate: 0.0511,
            total_effective_cost: 0.0533,
            eir_yearly: 0.818342,
            tec_yearly: 0.865534,
            eir_monthly: 0.0511,
            tec_monthly: 0.0533,
            total_iof: 434.81,
            contract_amount: 13288.24,
            contract_amount_without_tac: 0.0,
            tac_amount: 0.0,
            iof_percentage: 8.2e-5,
            overall_iof: 0.0038,
            pre_disbursement_amount: 12853.48,
            paid_total_iof: 434.86,
            paid_contract_amount: 13288.29,
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_qi_tech_wrong_amount() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
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
            disbursement_only_on_business_days: false,
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
            disbursement_only_on_business_days: false,
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
            disbursement_date: params.requested_date,
            accumulated_days: 61,
            days_index: 0.904902610445393,
            accumulated_days_index: 1.8553874582128072,
            interest_rate: 0.035,
            installment_amount: 108.85,
            installment_amount_without_tac: 0.0,
            total_amount: 217.7,
            debit_service: 15.739999999999982,
            customer_debit_service_amount: 15.739999999999982,
            customer_amount: 108.85,
            calculation_basis_for_effective_interest_rate: 108.085,
            merchant_debit_service_amount: 0.0,
            merchant_total_amount: 10.021500000000001,
            settled_to_merchant: 190.4085,
            mdr_amount: 10.021500000000001,
            effective_interest_rate: 0.0511,
            total_effective_cost: 0.0564,
            eir_yearly: 0.818895,
            tec_yearly: 0.932356,
            eir_monthly: 0.0511,
            tec_monthly: 0.0564,
            total_iof: 1.53,
            contract_amount: 201.96,
            contract_amount_without_tac: 0.0,
            tac_amount: 0.0,
            iof_percentage: 8.2e-5,
            overall_iof: 0.0038,
            pre_disbursement_amount: 200.43,
            paid_total_iof: 1.53,
            paid_contract_amount: 201.96,
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_qi_tech_max_amount_reached() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
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

        assert_eq!(resp.len(), 5);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2025, 3, 23).unwrap();

        let expected = Response {
            installment: 5,
            due_date: expected_due_date,
            disbursement_date: params.requested_date,
            accumulated_days: 151,
            days_index: 0.780857472156382,
            accumulated_days_index: 4.313849574971558,
            interest_rate: 0.035,
            installment_amount: 469.14,
            installment_amount_without_tac: 0.0,
            total_amount: 2345.7,
            debit_service: 321.87999999999977,
            customer_debit_service_amount: 321.87999999999977,
            customer_amount: 469.14,
            calculation_basis_for_effective_interest_rate: 464.462,
            merchant_debit_service_amount: 0.0,
            merchant_total_amount: 100.0215,
            settled_to_merchant: 1900.4085,
            mdr_amount: 100.0215,
            effective_interest_rate: 0.0511,
            total_effective_cost: 0.0553,
            eir_yearly: 0.818306,
            tec_yearly: 0.90758,
            eir_monthly: 0.0511,
            tec_monthly: 0.0553,
            total_iof: 23.39,
            contract_amount: 2023.8200000000002,
            contract_amount_without_tac: 0.0,
            tac_amount: 0.0,
            iof_percentage: 8.2e-5,
            overall_iof: 0.0038,
            pre_disbursement_amount: 2000.41,
            paid_total_iof: 23.37,
            paid_contract_amount: 2023.8,
        };

        assert_eq!(resp, expected);
    }
}
