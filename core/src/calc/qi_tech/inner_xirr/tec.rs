use xirr::{compute, Payment};

use crate::{err::PaymentPlanError, Params};

use super::MONTH_AS_YEAR_FRACTION;

pub fn calculate_tec_monthly(
    params: Params,
    tec_params: Vec<Payment>,
) -> Result<f64, PaymentPlanError> {
    let mut total_effective_cost_xirr = vec![Payment {
        amount: params.requested_amount,
        date: params.requested_date,
    }];
    total_effective_cost_xirr.extend(tec_params);

    let mut tec_monthly = 0.0;
    let tec_greater_than_two = total_effective_cost_xirr.len() > 2;
    let date_on_same_day = params
        .first_payment_date
        .signed_duration_since(params.requested_date)
        .num_days()
        == 0;

    if !tec_greater_than_two && date_on_same_day {
        return Ok(tec_monthly);
    }

    let xir_result = compute(&total_effective_cost_xirr);
    match xir_result {
        Ok(xirr) => {
            tec_monthly = xirr + 1.0;
            tec_monthly = tec_monthly.powf(MONTH_AS_YEAR_FRACTION) - 1.0;
        }
        Err(_) => {
            let converged_tec_params: Vec<Payment> = total_effective_cost_xirr
                .iter()
                .map(|tec| Payment {
                    amount: -1.0 * tec.amount,
                    date: tec.date,
                })
                .collect();

            let xir_result = compute(&converged_tec_params)?;
            tec_monthly = xir_result + 1.0;
            tec_monthly = tec_monthly.powf(MONTH_AS_YEAR_FRACTION) - 1.0;
        }
    }

    if tec_monthly.is_nan() {
        return Err(PaymentPlanError::InvalidRequestedAmount);
    }
    return Ok(tec_monthly);
}
