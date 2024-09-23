use crate::{
    err::PaymentPlanError, util::add_months, DownPaymentParams, DownPaymentResponse, Params,
    Response,
};

const POTENCY: f64 = 0.0333333333333333333333333333333333333;

use super::PaymentPlan;

pub struct QiTech;

impl PaymentPlan for QiTech {
    fn calculate_payment_plan(&self, params: Params) -> Result<Vec<Response>, PaymentPlanError> {
        let main_value = params.requested_amount;

        let mut resp = foo(&params, main_value);

        println!("resp: {:?}", resp);

        let main_value = resp.pop().unwrap();
        println!("Main Value: {}", main_value);

        let mut resp = foo(&params, main_value);

        let main_value = resp.pop().unwrap();

        let mut resp = foo(&params, main_value);

        let main_value = resp.pop().unwrap();

        let mut resp = foo(&params, main_value);

        let main_value = resp.pop().unwrap();

        let mut resp = foo(&params, main_value);

        let main_value = resp.pop().unwrap();

        let cacl = calc_installments(&params, main_value).pop().unwrap();

        println!("Calc: {:?}", cacl);

        println!(
            "result {}",
            cacl.installment_amount * (cacl.installment + 1) as f64
        );

        todo!()
    }

    fn calculate_down_payment_plan(
        &self,
        params: DownPaymentParams,
    ) -> Result<Vec<DownPaymentResponse>, PaymentPlanError> {
        println!("Params: {:?}", params);
        todo!()
    }
}

fn foo(params: &Params, main_value: f64) -> Vec<f64> {
    let mut result = vec![];
    let interest_rate = params.interest_rate;

    let iof_overall = params.iof_overall;

    let daily_interest_rate = (1.0 + interest_rate).powf(POTENCY) - 1.0;
    let daily_interest_rate = round_decimal_cases(daily_interest_rate, 8);

    let rs = calc_installments(&params, main_value);

    for r in &rs {
        let mut main_value_l = main_value;
        let mut accumulated_iof = 0.0;
        for i in 0..=r.installment {
            let diff = r.diffs[i as usize];
            let mut accumulated_days = r.accumulated_days[i as usize];
            let fee = main_value_l * ((1.0 + daily_interest_rate).powf(diff as f64) - 1.0);

            let installment_amount_without_fee = r.installment_amount - fee;
            let main_iof = installment_amount_without_fee * iof_overall;
            let main_iof = round_decimal_cases(main_iof, 8);
            if accumulated_days >= 365 {
                accumulated_days = 365;
            }
            let installment_iof =
                installment_amount_without_fee * accumulated_days as f64 * 0.000082;
            let installment_iof = round_decimal_cases(installment_iof, 8);

            let iof = main_iof + installment_iof;

            accumulated_iof += iof;
            if r.installment == 17 {
                println!("installments_amount: {}", r.installment_amount);
                println!(
                    "installments_amount_without_fee: {}",
                    installment_amount_without_fee
                );
                println!("main_value_l: {}", main_value_l);
                println!("main_iof: {}", main_iof);
                println!("installment_iof: {}", installment_iof);
                println!("iof: {}", iof);
                println!("accumulated_iof: {}", accumulated_iof);
                println!("-----------------------------------------------")
            }

            main_value_l = main_value_l + fee - r.installment_amount;
        }
        result.push(params.requested_amount + accumulated_iof);
    }

    return result;
}

fn round_decimal_cases(value: f64, round: i32) -> f64 {
    let factor = 10f64.powi(round);
    (value * factor).round() / factor
}

#[derive(Debug)]
struct R {
    accumulated_days: Vec<i64>,
    diffs: Vec<i64>,
    factor: f64,
    installment_amount: f64,
    installment: u32,
}

fn calc_installments(params: &Params, main_value: f64) -> Vec<R> {
    let mut result = vec![];

    let requested_amount = main_value;
    let requested_date = params.requested_date;
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;
    let interest_rate = params.interest_rate;

    let daily_interest_rate = (1.0 + interest_rate).powf(POTENCY) - 1.0;
    let daily_interest_rate = round_decimal_cases(daily_interest_rate, 8);

    println!("daily_interest_rate: {}", daily_interest_rate);

    let mut last_due_date = requested_date;
    let mut due_date = first_payment_date;
    let mut accumulated_days = 0;
    let mut accumulated_factor = 0.0;

    let mut diffs = vec![];
    let mut accumulated_days_v = vec![];

    for i in 0..installments {
        if i != 0 {
            last_due_date = due_date;
            due_date = add_months(due_date, 1);
        }

        let diff = due_date.signed_duration_since(last_due_date).num_days();
        diffs.push(diff);
        accumulated_days += diff;
        let factor = 1.0 / (1.0 + daily_interest_rate).powf(accumulated_days as f64);

        accumulated_factor += factor;
        let installment_amount = requested_amount / accumulated_factor;
        let installment_amount = round_decimal_cases(installment_amount, 2);
        accumulated_days_v.push(accumulated_days);

        result.push(R {
            accumulated_days: accumulated_days_v.clone(),
            diffs: diffs.clone(),
            factor,
            installment_amount,
            installment: i,
        });
    }
    return result;
}
