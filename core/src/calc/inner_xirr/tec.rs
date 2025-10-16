use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal, MathematicalOps,
};
use xirr::{compute, Payment};

use crate::{err::PaymentPlanError, Params};

pub fn calculate_tec_monthly(
    params: Params,
    tec_params: Vec<Payment>,
    calculation_basis_for_effective_interest_rate: Decimal,
) -> Result<Decimal, PaymentPlanError> {
    let mut total_effective_cost_xirr = vec![Payment {
        amount: params.requested_amount.to_f64().unwrap(),
        date: params.disbursement_date,
    }];

    total_effective_cost_xirr.extend(tec_params);

    let mut tec_monthly = Decimal::ZERO;
    let tec_greater_than_two = total_effective_cost_xirr.len() > 2;
    let date_on_same_day = params
        .first_payment_date
        .signed_duration_since(params.disbursement_date)
        .num_days()
        == 0;

    if !tec_greater_than_two && date_on_same_day {
        return Ok(tec_monthly);
    }

    let xir_result = compute(&total_effective_cost_xirr);
    match xir_result {
        Ok(xirr) => {
            if xirr.is_nan() {
                return Err(PaymentPlanError::XirCalculationError(params));
            }
            tec_monthly = Decimal::from_f64(xirr).unwrap() + Decimal::ONE;
            tec_monthly =
                tec_monthly.powd(calculation_basis_for_effective_interest_rate) - Decimal::ONE;
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
            if xir_result.is_nan() {
                return Err(PaymentPlanError::XirCalculationError(params));
            }
            tec_monthly = Decimal::from_f64(xir_result).unwrap() + Decimal::ONE;
            tec_monthly =
                tec_monthly.powd(calculation_basis_for_effective_interest_rate) - Decimal::ONE;
        }
    }

    return Ok(tec_monthly);
}

#[cfg(test)]
mod test {
    use rust_decimal::{dec, Decimal};
    use xirr::Payment;

    use crate::{calc::inner_xirr::tec::calculate_tec_monthly, Params};

    #[test]
    fn test_calculate_tec_monthly_test_7() {
        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(2900.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 30).unwrap(),
            installments: 6,
            debit_service_percentage: 0,
            mdr: dec!(0.029900000000000003),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.035),
        };

        let tec_params = vec![Payment {
            amount: -3024.0190557363553,
            date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
        }];

        let tec_monthly =
            calculate_tec_monthly(params, tec_params, dec!(0.0821917808219178)).unwrap();

        assert_eq!(tec_monthly, dec!(0.041357534253765094));

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

        let tec_monthly =
            calculate_tec_monthly(params, tec_params, dec!(0.0821917808219178)).unwrap();

        assert_eq!(tec_monthly, dec!(0.0401413181284036));

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

        let tec_monthly =
            calculate_tec_monthly(params, tec_params, dec!(0.0821917808219178)).unwrap();

        assert_eq!(tec_monthly, dec!(0.039521601442900955));

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

        let tec_monthly =
            calculate_tec_monthly(params, tec_params, dec!(0.0821917808219178)).unwrap();

        assert_eq!(tec_monthly, dec!(0.03915824678675084));

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

        let tec_monthly =
            calculate_tec_monthly(params, tec_params, dec!(0.0821917808219178)).unwrap();

        assert_eq!(tec_monthly, dec!(0.038918973894719766));

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

        let tec_monthly =
            calculate_tec_monthly(params, tec_params, dec!(0.0821917808219178)).unwrap();

        assert_eq!(tec_monthly, dec!(0.03875204347989669));
    }
}
