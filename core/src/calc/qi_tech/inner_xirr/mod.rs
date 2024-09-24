use chrono::NaiveDate;
use xirr::Payment;

const MONTH_AS_YEAR_FRACTION: f64 = 0.0821917808219178; // 30/365
pub mod eir;
pub mod tec;

pub fn prepare_xirr_params(
    installments: u32,
    due_dates: &Vec<NaiveDate>,
    calculation_basis_for_eir: f64,
    customer_amount: f64,
) -> (Vec<Payment>, Vec<Payment>) {
    let mut eir_params = Vec::new();
    let mut tec_params = Vec::new();

    let eir_amount = -1.0 * calculation_basis_for_eir;
    let tec_amount = -1.0 * customer_amount;

    for i in 0..installments {
        let date = due_dates[i as usize];

        eir_params.push(Payment {
            amount: eir_amount,
            date,
        });
        tec_params.push(Payment {
            amount: tec_amount,
            date,
        });
    }

    return (eir_params, tec_params);
}
