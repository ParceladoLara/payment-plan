use chrono::NaiveDate;

use crate::util::add_months;

use super::QiTechParams;

#[derive(Debug)]
pub struct InstallmentData {
    pub accumulated_days: Vec<i64>,
    pub diffs: Vec<i64>,
    pub amount: f64,
    pub first_due_date: NaiveDate,
    pub last_due_date: NaiveDate,
}

pub fn calc(qi_params: &QiTechParams) -> InstallmentData {
    let daily_interest_rate = qi_params.daily_interest_rate;

    let params = qi_params.params;

    let requested_date = params.requested_date;
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;

    let mut last_due_date = requested_date;
    let mut due_date = first_payment_date;
    let mut accumulated_days = 0;
    let mut accumulated_factor = 0.0;

    let first_due_date = due_date;

    let mut diffs = Vec::with_capacity(installments as usize);
    let mut accumulated_days_v = Vec::with_capacity(installments as usize);

    let mut instalment_amount_result = 0.0;

    for i in 0..installments {
        let main_value = qi_params.main_value;
        if i != 0 {
            last_due_date = due_date;
            due_date = add_months(due_date, 1);
        }

        let diff = due_date.signed_duration_since(last_due_date).num_days();
        diffs.push(diff);
        accumulated_days += diff;
        let factor = 1.0 / (1.0 + daily_interest_rate).powf(accumulated_days as f64);

        accumulated_factor += factor;
        let installment_amount = main_value / accumulated_factor;
        let installment_amount = installment_amount;
        accumulated_days_v.push(accumulated_days);

        instalment_amount_result = installment_amount;
    }
    return InstallmentData {
        accumulated_days: accumulated_days_v,
        diffs,
        amount: instalment_amount_result,
        first_due_date,
        last_due_date: due_date,
    };
}
