use rust_decimal::{dec, Decimal};

use crate::Params;

pub fn calculate_iof(params: Params, accumulated_days: Vec<i64>, installments: Decimal) -> Decimal {
    let requested_amount = params.requested_amount;
    let tac_amount = params.tac_percentage;
    let iof_percentage = params.iof_percentage;
    let iof_overall = params.iof_overall;

    let installment_amount_without_interest = (requested_amount + tac_amount) / installments;
    let installment_amount_without_interest =
        (installment_amount_without_interest * Decimal::ONE_HUNDRED).round() / Decimal::ONE_HUNDRED;

    let contract_iof = installment_amount_without_interest * installments * iof_overall;

    let daily_iof = iof_percentage / dec!(365.0);

    let iof_calculation: Vec<Decimal> = accumulated_days
        .into_iter()
        .map(|days| {
            if days > 364 {
                return installment_amount_without_interest * iof_percentage;
            } else {
                return Decimal::from(days) * installment_amount_without_interest * daily_iof;
            }
        })
        .collect();

    let installment_iof: Decimal = iof_calculation.iter().sum();

    return contract_iof + installment_iof;
}

#[cfg(test)]
mod test {
    use rust_decimal::{dec, Decimal};

    use crate::{calc::providers::simple::iof::calculate_iof, Params};

    #[test]
    fn test_total_iof_test_6() {
        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(1500.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 09).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 09).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.025),
        };

        let aux_accumulated_days = vec![31];
        let total_iof = calculate_iof(params, aux_accumulated_days, Decimal::ONE);
        assert_eq!(total_iof, dec!(9.521917808219179));

        let aux_accumulated_days = vec![31, 61];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(2.0));
        assert_eq!(total_iof, dec!(11.37123287671233));

        let aux_accumulated_days = vec![31, 61, 92];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(3.0));
        assert_eq!(total_iof, dec!(13.26164383561644));

        let aux_accumulated_days = vec![31, 61, 92, 123];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(4.0));
        assert_eq!(total_iof, dec!(15.162328767123288));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(5.0));
        assert_eq!(total_iof, dec!(17.042465753424658));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(6.0));
        assert_eq!(total_iof, dec!(18.932876712328767));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(7.0));
        assert_eq!(total_iof, dec!(20.811962219178085));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(8.0));
        assert_eq!(total_iof, dec!(22.69828767123288));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(9.0));
        assert_eq!(total_iof, dec!(24.590902767123282));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276, 304];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(10.0));
        assert_eq!(total_iof, dec!(26.449315068493153));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276, 304, 335];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(11.0));
        assert_eq!(total_iof, dec!(28.316928547945206));

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276, 304, 335, 365];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(12.0));
        assert_eq!(total_iof, dec!(30.182876712328767));
    }

    #[test]
    fn test_total_iof_test_7() {
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

        let aux_accumulated_days = vec![31];
        let total_iof = calculate_iof(params, aux_accumulated_days, Decimal::ONE);
        assert_eq!(total_iof, dec!(18.40904109589041));

        let aux_accumulated_days = vec![31, 61];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(2.0));
        assert_eq!(total_iof, dec!(21.984383561643835));

        let aux_accumulated_days = vec![31, 61, 92];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(3.0));
        assert_eq!(total_iof, dec!(25.639266493150686));

        let aux_accumulated_days = vec![31, 61, 92, 122];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(4.0));
        assert_eq!(total_iof, dec!(29.254246575342467));

        let aux_accumulated_days = vec![31, 61, 92, 122, 153];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(5.0));
        assert_eq!(total_iof, dec!(32.90109589041096));

        let aux_accumulated_days = vec![31, 61, 92, 122, 153, 184];
        let total_iof = calculate_iof(params, aux_accumulated_days, dec!(6.0));
        assert_eq!(total_iof, dec!(36.56358345205479));
    }
}
