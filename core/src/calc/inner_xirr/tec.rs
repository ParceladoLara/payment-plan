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

#[cfg(test)]
mod test {
    use xirr::Payment;

    use crate::{calc::inner_xirr::tec::calculate_tec_monthly, Params};

    #[test]
    fn test_calculate_tec_monthly_test_7() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 2900.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 30).unwrap(),
            installments: 6,
            debit_service_percentage: 0,
            mdr: 0.029900000000000003,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.035,
        };

        let tec_params = vec![Payment {
            amount: -3024.0190557363553,
            date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
        }];

        let tec_monthly = calculate_tec_monthly(params, tec_params).unwrap();

        assert_eq!(tec_monthly, 0.041357534253765094);

        let tec_params = vec![
            Payment {
                amount: -1539.8988271991445,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -1539.8988271991445,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
        ];

        let tec_monthly = calculate_tec_monthly(params, tec_params).unwrap();

        assert_eq!(tec_monthly, 0.0401413181284036);

        let tec_params = vec![
            Payment {
                amount: -1045.8446791163315,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -1045.8446791163315,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -1045.8446791163315,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
        ];

        let tec_monthly = calculate_tec_monthly(params, tec_params).unwrap();

        assert_eq!(tec_monthly, 0.039521601442900955);

        let tec_params = vec![
            Payment {
                amount: -798.8498495930802,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -798.8498495930802,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -798.8498495930802,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
            Payment {
                amount: -798.8498495930802,
                date: chrono::NaiveDate::from_ymd_opt(2022, 07, 30).unwrap(),
            },
        ];

        let tec_monthly = calculate_tec_monthly(params, tec_params).unwrap();

        assert_eq!(tec_monthly, 0.03915824678675084);

        let tec_params = vec![
            Payment {
                amount: -650.8993291092211,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -650.8993291092211,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -650.8993291092211,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
            Payment {
                amount: -650.8993291092211,
                date: chrono::NaiveDate::from_ymd_opt(2022, 07, 30).unwrap(),
            },
            Payment {
                amount: -650.8993291092211,
                date: chrono::NaiveDate::from_ymd_opt(2022, 08, 30).unwrap(),
            },
        ];

        let tec_monthly = calculate_tec_monthly(params, tec_params).unwrap();

        assert_eq!(tec_monthly, 0.038918973894719766);

        let tec_params = vec![
            Payment {
                amount: -552.4322553512001,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -552.4322553512001,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -552.4322553512001,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
            Payment {
                amount: -552.4322553512001,
                date: chrono::NaiveDate::from_ymd_opt(2022, 07, 30).unwrap(),
            },
            Payment {
                amount: -552.4322553512001,
                date: chrono::NaiveDate::from_ymd_opt(2022, 08, 30).unwrap(),
            },
            Payment {
                amount: -552.4322553512001,
                date: chrono::NaiveDate::from_ymd_opt(2022, 09, 30).unwrap(),
            },
        ];

        let tec_monthly = calculate_tec_monthly(params, tec_params).unwrap();

        assert_eq!(tec_monthly, 0.03875204347989669);
    }
}
