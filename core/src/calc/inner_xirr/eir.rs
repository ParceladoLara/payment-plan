use rust_decimal::{prelude::*, Decimal};
use xirr::{compute, Payment};

use crate::{err::PaymentPlanError, Params};

pub fn calculate_eir_monthly(
    params: Params,
    eir_params: Vec<Payment>,
    customer_debit_service_proportion: Decimal,
    calculation_basis_for_effective_interest_rate: Decimal,
) -> Result<Decimal, PaymentPlanError> {
    /*
        Para calcular a taxa efetiva de juros, calcula-se o valor de parcela considerando-se apenas o valor requisitado
        e o fator de multiplicação.
        Em outras palavras, desconsidera-se a TAC e o IOF. Estes dois últimos componentes são considerados no calculo do
        custo efetivo total (Que é diferente da Taxa efetiva de juros).
    */
    let mut effective_interest_rate_xirr = vec![Payment {
        amount: params.requested_amount.to_f64().unwrap(),
        date: params.disbursement_date,
    }];
    effective_interest_rate_xirr.extend(eir_params);

    let mut eir_monthly = Decimal::ZERO;

    let customer_dsp_ok = customer_debit_service_proportion > Decimal::ZERO
        && customer_debit_service_proportion <= Decimal::ONE;

    let date_ok = params
        .first_payment_date
        .signed_duration_since(params.disbursement_date)
        .num_days()
        > 0;

    if customer_dsp_ok && date_ok {
        let xir_result = compute(&effective_interest_rate_xirr);
        match xir_result {
            Ok(xirr) => {
                if xirr.is_nan() {
                    return Err(PaymentPlanError::XirCalculationError(params));
                }
                let xirr = Decimal::from_f64(xirr).unwrap();
                eir_monthly = xirr + Decimal::ONE;
                eir_monthly =
                    eir_monthly.powd(calculation_basis_for_effective_interest_rate) - Decimal::ONE;
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
                if xir_result.is_nan() {
                    return Err(PaymentPlanError::XirCalculationError(params));
                }
                eir_monthly = Decimal::from_f64(xir_result).unwrap() + Decimal::ONE;
                eir_monthly =
                    eir_monthly.powd(calculation_basis_for_effective_interest_rate) - Decimal::ONE;
            }
        }
    }
    return Ok(eir_monthly);
}

#[cfg(test)]
mod test {
    use rust_decimal::{dec, Decimal};
    use xirr::Payment;

    use crate::{calc::inner_xirr::eir::calculate_eir_monthly, Params};

    #[test]
    fn test_calculate_eir_monthly_test_7() {
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
        let customer_debit_service_proportion = Decimal::ONE;

        let eir_params = vec![Payment {
            amount: -3005.610014640465,
            date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
        }];

        let eir_monthly = calculate_eir_monthly(
            params,
            eir_params,
            customer_debit_service_proportion,
            dec!(0.0821917808219178),
        )
        .unwrap();

        assert_eq!(eir_monthly, dec!(0.03522205067950779));

        let eir_params = vec![
            Payment {
                amount: -1528.9066354183226,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -1528.9066354183226,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
        ];

        let eir_monthly = calculate_eir_monthly(
            params,
            eir_params,
            customer_debit_service_proportion,
            dec!(0.0821917808219178),
        )
        .unwrap();

        assert_eq!(eir_monthly, dec!(0.03526367198542446));

        let eir_params = vec![
            Payment {
                amount: -1037.298256951948,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -1037.298256951948,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -1037.298256951948,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
        ];

        let eir_monthly = calculate_eir_monthly(
            params,
            eir_params,
            customer_debit_service_proportion,
            dec!(0.0821917808219178),
        )
        .unwrap();

        assert_eq!(eir_monthly, dec!(0.03530579035535042));

        let eir_params = vec![
            Payment {
                amount: -791.5362879492445,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -791.5362879492445,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -791.5362879492445,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
            Payment {
                amount: -791.5362879492445,
                date: chrono::NaiveDate::from_ymd_opt(2022, 07, 30).unwrap(),
            },
        ];

        let eir_monthly = calculate_eir_monthly(
            params,
            eir_params,
            customer_debit_service_proportion,
            dec!(0.0821917808219178),
        )
        .unwrap();

        assert_eq!(eir_monthly, dec!(0.0353469732503493));

        let eir_params = vec![
            Payment {
                amount: -644.3191099311389,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -644.3191099311389,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -644.3191099311389,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
            Payment {
                amount: -644.3191099311389,
                date: chrono::NaiveDate::from_ymd_opt(2022, 07, 30).unwrap(),
            },
            Payment {
                amount: -644.3191099311389,
                date: chrono::NaiveDate::from_ymd_opt(2022, 08, 30).unwrap(),
            },
        ];

        let eir_monthly = calculate_eir_monthly(
            params,
            eir_params,
            customer_debit_service_proportion,
            dec!(0.0821917808219178),
        )
        .unwrap();

        assert_eq!(eir_monthly, dec!(0.03538811206577974));

        let eir_params = vec![
            Payment {
                amount: -546.3383247758576,
                date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            },
            Payment {
                amount: -546.3383247758576,
                date: chrono::NaiveDate::from_ymd_opt(2022, 05, 30).unwrap(),
            },
            Payment {
                amount: -546.3383247758576,
                date: chrono::NaiveDate::from_ymd_opt(2022, 06, 30).unwrap(),
            },
            Payment {
                amount: -546.3383247758576,
                date: chrono::NaiveDate::from_ymd_opt(2022, 07, 30).unwrap(),
            },
            Payment {
                amount: -546.3383247758576,
                date: chrono::NaiveDate::from_ymd_opt(2022, 08, 30).unwrap(),
            },
            Payment {
                amount: -546.3383247758576,
                date: chrono::NaiveDate::from_ymd_opt(2022, 09, 30).unwrap(),
            },
        ];

        let eir_monthly = calculate_eir_monthly(
            params,
            eir_params,
            customer_debit_service_proportion,
            dec!(0.0821917808219178),
        )
        .unwrap();

        assert_eq!(eir_monthly, dec!(0.035429014326330055));
    }
}
