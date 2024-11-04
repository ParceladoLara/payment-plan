use crate::plan::Params;

pub fn calculate_iof(params: Params, accumulated_days: Vec<i64>, installments: f64) -> f64 {
    let requested_amount = params.requested_amount;
    let tac_amount = params.tac_percentage;
    let iof_percentage = params.iof_percentage;
    let iof_overall = params.iof_overall;

    let installment_amount_without_interest = (requested_amount + tac_amount) / installments;
    let installment_amount_without_interest = installment_amount_without_interest + f64::EPSILON;
    let installment_amount_without_interest =
        (installment_amount_without_interest * 100.0).round() / 100.0;

    let contract_iof = installment_amount_without_interest * installments * iof_overall;

    let daily_iof = iof_percentage / 365.0;

    let iof_calculation: Vec<f64> = accumulated_days
        .into_iter()
        .map(|days| {
            if days > 364 {
                return installment_amount_without_interest * iof_percentage;
            } else {
                return days as f64 * installment_amount_without_interest * daily_iof;
            }
        })
        .collect();

    let installment_iof: f64 = iof_calculation.iter().sum();

    return contract_iof + installment_iof;
}

#[cfg(test)]
mod test {
    use crate::{calc::providers::bmp::iof::calculate_iof, plan::Params};

    #[test]
    fn test_total_iof_test_6() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1500.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 09).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 09).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.025,
        };

        let aux_accumulated_days = vec![31];
        let total_iof = calculate_iof(params, aux_accumulated_days, 1.0);
        assert_eq!(total_iof, 9.521917808219179);

        let aux_accumulated_days = vec![31, 61];
        let total_iof = calculate_iof(params, aux_accumulated_days, 2.0);
        assert_eq!(total_iof, 11.37123287671233);

        let aux_accumulated_days = vec![31, 61, 92];
        let total_iof = calculate_iof(params, aux_accumulated_days, 3.0);
        assert_eq!(total_iof, 13.26164383561644);

        let aux_accumulated_days = vec![31, 61, 92, 123];
        let total_iof = calculate_iof(params, aux_accumulated_days, 4.0);
        assert_eq!(total_iof, 15.162328767123288);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153];
        let total_iof = calculate_iof(params, aux_accumulated_days, 5.0);
        assert_eq!(total_iof, 17.042465753424658);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184];
        let total_iof = calculate_iof(params, aux_accumulated_days, 6.0);
        assert_eq!(total_iof, 18.932876712328767);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214];
        let total_iof = calculate_iof(params, aux_accumulated_days, 7.0);
        assert_eq!(total_iof, 20.811962219178085);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245];
        let total_iof = calculate_iof(params, aux_accumulated_days, 8.0);
        assert_eq!(total_iof, 22.69828767123288);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276];
        let total_iof = calculate_iof(params, aux_accumulated_days, 9.0);
        assert_eq!(total_iof, 24.590902767123282);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276, 304];
        let total_iof = calculate_iof(params, aux_accumulated_days, 10.0);
        assert_eq!(total_iof, 26.449315068493153);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276, 304, 335];
        let total_iof = calculate_iof(params, aux_accumulated_days, 11.0);
        assert_eq!(total_iof, 28.316928547945206);

        let aux_accumulated_days = vec![31, 61, 92, 123, 153, 184, 214, 245, 276, 304, 335, 365];
        let total_iof = calculate_iof(params, aux_accumulated_days, 12.0);
        assert_eq!(total_iof, 30.182876712328767);
    }

    #[test]
    fn test_total_iof_test_7() {
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

        let aux_accumulated_days = vec![31];
        let total_iof = calculate_iof(params, aux_accumulated_days, 1.0);
        assert_eq!(total_iof, 18.40904109589041);

        let aux_accumulated_days = vec![31, 61];
        let total_iof = calculate_iof(params, aux_accumulated_days, 2.0);
        assert_eq!(total_iof, 21.984383561643835);

        let aux_accumulated_days = vec![31, 61, 92];
        let total_iof = calculate_iof(params, aux_accumulated_days, 3.0);
        assert_eq!(total_iof, 25.639266493150686);

        let aux_accumulated_days = vec![31, 61, 92, 122];
        let total_iof = calculate_iof(params, aux_accumulated_days, 4.0);
        assert_eq!(total_iof, 29.254246575342467);

        let aux_accumulated_days = vec![31, 61, 92, 122, 153];
        let total_iof = calculate_iof(params, aux_accumulated_days, 5.0);
        assert_eq!(total_iof, 32.90109589041096);

        let aux_accumulated_days = vec![31, 61, 92, 122, 153, 184];
        let total_iof = calculate_iof(params, aux_accumulated_days, 6.0);
        assert_eq!(total_iof, 36.56358345205479);
    }
}
