use xirr::{compute, Payment};

use crate::{err::PaymentPlanError, Params};

use super::MONTH_AS_YEAR_FRACTION;

pub fn calculate_eir_monthly(
    params: Params,
    eir_params: Vec<Payment>,
    customer_debit_service_proportion: f64,
) -> Result<f64, PaymentPlanError> {
    /*
        Para calcular a taxa efetiva de juros, calcula-se o valor de parcela considerando-se apenas o valor requisitado
        e o fator de multiplicação.
        Em outras palavras, desconsidera-se a TAC e o IOF. Estes dois últimos componentes são considerados no calculo do
        custo efetivo total (Que é diferente da Taxa efetiva de juros).
    */
    let mut effective_interest_rate_xirr = vec![Payment {
        amount: params.requested_amount,
        date: params.requested_date,
    }];
    effective_interest_rate_xirr.extend(eir_params);

    let mut eir_monthly = 0.0;

    let customer_dsp_ok =
        customer_debit_service_proportion > 0.0 && customer_debit_service_proportion <= 1.0;

    let date_ok = params
        .first_payment_date
        .signed_duration_since(params.requested_date)
        .num_days()
        > 0;

    if customer_dsp_ok && date_ok {
        let xir_result = compute(&effective_interest_rate_xirr);
        match xir_result {
            Ok(xirr) => {
                eir_monthly = xirr + 1.0;
                eir_monthly = eir_monthly.powf(MONTH_AS_YEAR_FRACTION) - 1.0;
            }
            Err(_) => {
                let converged_eir_params: Vec<Payment> = effective_interest_rate_xirr
                    .iter()
                    .map(|eir| Payment {
                        amount: -1.0 * eir.amount,
                        date: eir.date,
                    })
                    .collect();

                let xir_result = compute(&converged_eir_params)?;
                eir_monthly = xir_result + 1.0;
                eir_monthly = eir_monthly.powf(MONTH_AS_YEAR_FRACTION) - 1.0;
            }
        }
    }
    if eir_monthly.is_nan() {
        return Err(PaymentPlanError::InvalidRequestedAmount);
    }
    return Ok(eir_monthly);
}
