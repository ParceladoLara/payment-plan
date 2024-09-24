use amounts::calculate_amounts;

use crate::{
    calc::{
        inner_xirr::{eir::calculate_eir_monthly, prepare_xirr_params, tec::calculate_tec_monthly},
        PaymentPlan,
    },
    err::PaymentPlanError,
    util::round_decimal_cases,
    DownPaymentParams, DownPaymentResponse, Params, Response,
};

const POTENCY: f64 = 0.0333333333333333333333333333333333333;

const NUM_OF_RUNS: u32 = 7; //7 passes is the minimum to get the same data as the xml

mod amounts;
mod installment;

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
        let mut response = Vec::with_capacity(params.installments as usize);

        let min_installment_amount = params.min_installment_amount;
        let max_total_amount = params.max_total_amount;
        let interest_rate = params.interest_rate;

        let daily_interest_rate = (1.0 + interest_rate).powf(POTENCY) - 1.0;
        let daily_interest_rate = round_decimal_cases(daily_interest_rate, 8);

        let main_value = params.requested_amount;

        for i in 1..=params.installments {
            params.installments = i;
            let params = QiTechParams {
                params,
                main_value,
                daily_interest_rate,
            };

            let resp = calc(params);
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

    fn calculate_down_payment_plan(
        &self,
        params: DownPaymentParams,
    ) -> Result<Vec<DownPaymentResponse>, PaymentPlanError> {
        println!("Params: {:?}", params);
        todo!()
    }
}

fn calc(mut params: QiTechParams) -> Response {
    let debit_service_percentage = params.params.debit_service_percentage;
    let requested_amount = params.params.requested_amount;
    let mut iof = 0.0;
    for _ in 0..NUM_OF_RUNS {
        iof = total_iof(&params);
        params.main_value = requested_amount + iof;
    }

    let mut data = installment::calc(&params);

    let installment_amount = data.amount;
    let installments = params.params.installments;
    let total_amount = installment_amount * installments as f64;
    let contract_amount = params.params.requested_amount + iof;
    let accumulated_days = data.accumulated_days.pop().unwrap();
    let accumulated_days_index = data.accumulated_factor;
    let customer_debit_service_proportion = 1.0 - debit_service_percentage as f64 / 100.0;

    let params = params.params;

    let amounts = calculate_amounts(
        params,
        installments as f64,
        customer_debit_service_proportion,
        iof,
        total_amount,
    );

    let (eir_params, tec_params) = prepare_xirr_params(
        installments,
        &data.due_dates,
        amounts.calculation_basis_for_effective_interest_rate,
        amounts.customer_amount,
    );

    let eir_monthly =
        calculate_eir_monthly(params, eir_params, customer_debit_service_proportion).unwrap(); //TODO: handle error

    let eir_yearly = (1.0 + eir_monthly).powf(12.0) - 1.0;

    let tec_monthly = calculate_tec_monthly(params, tec_params).unwrap(); //TODO: handle error

    let tec_yearly = (1.0 + tec_monthly).powf(12.0) - 1.0;

    return Response {
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
        customer_amount: amounts.customer_amount,
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
}

fn total_iof(qi_params: &QiTechParams) -> f64 {
    let mut total_iof = 0.0;
    let params = qi_params.params;
    let installments = params.installments;

    let iof_overall = params.iof_overall;

    let daily_interest_rate = qi_params.daily_interest_rate;

    let data = installment::calc(&qi_params);

    let main_value = qi_params.main_value;
    let mut main_value_l = main_value;

    let installment_amount = data.amount;
    for j in 0..installments {
        let diff = data.diffs[j as usize];
        let mut accumulated_days = data.accumulated_days[j as usize];
        let fee = main_value_l * ((1.0 + daily_interest_rate).powf(diff as f64) - 1.0);

        let installment_amount_without_fee = installment_amount - fee;
        let main_iof = installment_amount_without_fee * iof_overall;
        let main_iof = round_decimal_cases(main_iof, 8);
        if accumulated_days >= 365 {
            accumulated_days = 365;
        }
        let installment_iof = installment_amount_without_fee * accumulated_days as f64 * 0.000082; //TODO: hardcoded value
        let installment_iof = round_decimal_cases(installment_iof, 8);

        let iof = main_iof + installment_iof;

        total_iof += iof;
        main_value_l = main_value_l + fee - installment_amount;
    }
    return total_iof;
}
