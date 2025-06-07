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
struct InnerParams {
    params: Params,
    main_value: f64,
    daily_interest_rate: f64,
}

/**
 * This is the main implementation of the payment plan calculation.
 * It uses an iterative approach to calculate the payment plan based on the provided parameters.
 * It calculates the payment plan by iterating through the installments over and over again to better approximate the real iof value.
 * It's slower than the simple implementation, but it provides a more accurate result.
 * This is the recommended implementation for most cases.
 */
pub struct Iterative;

impl PaymentPlan for Iterative {
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
            params.disbursement_date = get_next_business_day(params.disbursement_date);
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
            let params = InnerParams {
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

fn calc(mut params: InnerParams) -> Result<Response, PaymentPlanError> {
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
        disbursement_date: params.disbursement_date,
        pre_disbursement_amount,
        paid_total_iof: paid_iof,
        paid_contract_amount: requested_amount + paid_iof,
        installments: data.installments,
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
    use crate::{Installment, Params};

    #[test]
    fn test_iterative() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 12853.43,
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let iterative = Iterative;
        let mut resp = iterative.calculate_payment_plan(params).unwrap();
        assert_eq!(resp.len(), 48);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2028, 10, 23).unwrap();

        let expected = Response {
            installment: 48,
            disbursement_date: disbursement_date,
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
            installments: vec![
                Installment {
                    accumulated_days: 31,
                    factor: 0.950484847767414,
                    accumulated_factor: 0.950484847767414,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 61,
                    factor: 0.904902610445393,
                    accumulated_factor: 1.8553874582128072,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 92,
                    factor: 0.860096219933525,
                    accumulated_factor: 2.7154836781463323,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 1, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 123,
                    factor: 0.817508424668844,
                    accumulated_factor: 3.532992102815176,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 2, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 151,
                    factor: 0.780857472156382,
                    accumulated_factor: 4.313849574971558,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 182,
                    factor: 0.742193195550606,
                    accumulated_factor: 5.056042770522164,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 4, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 212,
                    factor: 0.706599964940101,
                    accumulated_factor: 5.762642735462265,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 5, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 243,
                    factor: 0.671612560108552,
                    accumulated_factor: 6.434255295570817,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 273,
                    factor: 0.639404152814921,
                    accumulated_factor: 7.0736594483857385,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 7, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 304,
                    factor: 0.607743958850142,
                    accumulated_factor: 7.68140340723588,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 8, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 335,
                    factor: 0.577651424209242,
                    accumulated_factor: 8.259054831445122,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 9, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 365,
                    factor: 0.549949094845911,
                    accumulated_factor: 8.809003926291032,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 396,
                    factor: 0.522718281694443,
                    accumulated_factor: 9.331722207985475,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 426,
                    factor: 0.497650371538146,
                    accumulated_factor: 9.829372579523621,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 12, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 457,
                    factor: 0.473009137632831,
                    accumulated_factor: 10.302381717156452,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 488,
                    factor: 0.449588018175537,
                    accumulated_factor: 10.75196973533199,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 2, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 516,
                    factor: 0.429431860016068,
                    accumulated_factor: 11.181401595348058,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 547,
                    factor: 0.40816847609385,
                    accumulated_factor: 11.589570071441909,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 4, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 577,
                    factor: 0.388594011136961,
                    accumulated_factor: 11.97816408257887,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 608,
                    factor: 0.369352719518843,
                    accumulated_factor: 12.347516802097713,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 6, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 638,
                    factor: 0.351639735081282,
                    accumulated_factor: 12.699156537178995,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 7, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 669,
                    factor: 0.334228240067706,
                    accumulated_factor: 13.0333847772467,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 700,
                    factor: 0.317678877880324,
                    accumulated_factor: 13.351063655127025,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 730,
                    factor: 0.302444006921837,
                    accumulated_factor: 13.653507662048861,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 10, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 761,
                    factor: 0.287468445877269,
                    accumulated_factor: 13.940976107926131,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 11, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 791,
                    factor: 0.273682371377135,
                    accumulated_factor: 14.214658479303266,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 12, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 822,
                    factor: 0.26013094709502,
                    accumulated_factor: 14.474789426398285,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 1, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 853,
                    factor: 0.247250523649204,
                    accumulated_factor: 14.722039950047488,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 2, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 881,
                    factor: 0.236165662713833,
                    accumulated_factor: 14.958205612761322,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 3, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 912,
                    factor: 0.224471883972448,
                    accumulated_factor: 15.18267749673377,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 4, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 942,
                    factor: 0.213706924687313,
                    accumulated_factor: 15.396384421421084,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 5, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 973,
                    factor: 0.203125193778263,
                    accumulated_factor: 15.599509615199347,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 6, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1003,
                    factor: 0.193383954019807,
                    accumulated_factor: 15.792893569219153,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 7, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1034,
                    factor: 0.183808518097177,
                    accumulated_factor: 15.97670208731633,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 8, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1065,
                    factor: 0.174707211341949,
                    accumulated_factor: 16.15140929865828,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 9, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1095,
                    factor: 0.166328807848234,
                    accumulated_factor: 16.317738106506514,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 10, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1126,
                    factor: 0.158093011606965,
                    accumulated_factor: 16.47583111811348,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 11, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1156,
                    factor: 0.150511372414138,
                    accumulated_factor: 16.626342490527616,
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 12, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1187,
                    factor: 0.143058778896316,
                    accumulated_factor: 16.76940126942393,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 1, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1218,
                    factor: 0.135975201681057,
                    accumulated_factor: 16.905376471104987,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 2, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1247,
                    factor: 0.129666503471616,
                    accumulated_factor: 17.0350429745766,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 3, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1278,
                    factor: 0.123246046812752,
                    accumulated_factor: 17.158289021389354,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 4, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1308,
                    factor: 0.117335557478792,
                    accumulated_factor: 17.275624578868147,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 5, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1339,
                    factor: 0.111525669487934,
                    accumulated_factor: 17.387150248356082,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 6, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1369,
                    factor: 0.106177252260624,
                    accumulated_factor: 17.493327500616708,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 7, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1400,
                    factor: 0.100919869451302,
                    accumulated_factor: 17.59424737006801,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 8, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1431,
                    factor: 0.095922806752128,
                    accumulated_factor: 17.69017017682014,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 9, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 1461,
                    factor: 0.091322653312291,
                    accumulated_factor: 17.78149283013243,
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 10, 23).unwrap(),
                },
            ],
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_iterative_wrong_amount() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 0.0,
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let iterative = Iterative;
        let resp = iterative.calculate_payment_plan(params);
        assert_eq!(resp.is_err(), true);

        assert_eq!(resp.unwrap_err(), PaymentPlanError::InvalidRequestedAmount);
    }

    #[test]
    fn test_iterative_wrong_installments() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 12853.43,
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 0,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let iterative = Iterative;
        let resp = iterative.calculate_payment_plan(params);
        assert_eq!(resp.is_err(), true);

        assert_eq!(
            resp.unwrap_err(),
            PaymentPlanError::InvalidNumberOfInstallments
        );
    }

    #[test]
    fn test_iterative_min_installment_amount_reached() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount: 200.43,
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let iterative = Iterative;

        let mut resp = iterative.calculate_payment_plan(params).unwrap();

        assert_eq!(resp.len(), 2);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap();

        let expected = Response {
            installment: 2,
            due_date: expected_due_date,
            disbursement_date: params.disbursement_date,
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
            installments: vec![
                Installment {
                    accumulated_days: 31,
                    factor: 0.950484847767414,
                    accumulated_factor: 0.950484847767414,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 61,
                    factor: 0.904902610445393,
                    accumulated_factor: 1.8553874582128072,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
                },
            ],
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_iterative_max_amount_reached() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: 2400.43,
            min_installment_amount: 100.0,
            requested_amount: 2000.43,
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,      // %0.38
            iof_percentage: 0.000082, // 0.0082%
            interest_rate: 0.035,
        };

        let iterative = Iterative;

        let mut resp = iterative.calculate_payment_plan(params).unwrap();

        assert_eq!(resp.len(), 5);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2025, 3, 23).unwrap();

        let expected = Response {
            installment: 5,
            due_date: expected_due_date,
            disbursement_date: params.disbursement_date,
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
            installments: vec![
                Installment {
                    accumulated_days: 31,
                    factor: 0.950484847767414,
                    accumulated_factor: 0.950484847767414,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 61,
                    factor: 0.904902610445393,
                    accumulated_factor: 1.8553874582128072,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 92,
                    factor: 0.860096219933525,
                    accumulated_factor: 2.7154836781463323,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 1, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 123,
                    factor: 0.817508424668844,
                    accumulated_factor: 3.532992102815176,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 2, 23).unwrap(),
                },
                Installment {
                    accumulated_days: 151,
                    factor: 0.780857472156382,
                    accumulated_factor: 4.313849574971558,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 3, 23).unwrap(),
                },
            ],
        };

        assert_eq!(resp, expected);
    }
}
