use crate::util::round_decimal_cases;

use super::{installment, QiTechParams};

pub fn calc(qi_params: &QiTechParams) -> f64 {
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
