use crate::util::round_decimal_cases;

use super::{installment::InstallmentData, QiTechParams};

pub fn calc(qi_params: &QiTechParams, data: &InstallmentData) -> f64 {
    let mut total_iof = 0.0;
    let params = qi_params.params;
    let installments = params.installments;

    let iof_overall = params.iof_overall;

    let daily_interest_rate = qi_params.daily_interest_rate;

    let main_value = qi_params.main_value;
    let mut main_value_l = main_value;

    let installment_amount = data.amount;
    for j in 0..installments {
        let diff = data.diffs[j as usize];
        let mut accumulated_days = data.accumulated_days[j as usize];
        let fee = main_value_l * ((1.0 + daily_interest_rate).powf(diff as f64) - 1.0);

        let installment_amount_without_fee = installment_amount - fee;
        let main_iof = installment_amount_without_fee * iof_overall;
        if accumulated_days >= 365 {
            accumulated_days = 365;
        }
        let installment_iof = installment_amount_without_fee * accumulated_days as f64 * 0.000082; //TODO: hardcoded value

        let iof = main_iof + installment_iof;

        total_iof += iof;
        main_value_l = main_value_l + fee - installment_amount;
    }
    let total_iof = round_decimal_cases(total_iof, 2);
    return total_iof;
}

#[cfg(test)]
mod test {
    use crate::{
        calc::providers::qi_tech::{installment::InstallmentData, QiTechParams},
        Params,
    };

    #[test]
    fn test_calc() {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 09, 24).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap();
        let params = QiTechParams {
            params: Params {
                requested_amount: 7431.0,
                first_payment_date,
                requested_date,
                installments: 18,
                debit_service_percentage: 0,
                mdr: 0.05,
                tac_percentage: 0.0,
                iof_overall: 0.0038,
                iof_percentage: 0.03,
                interest_rate: 0.04,
                min_installment_amount: 100.0,
                max_total_amount: f64::MAX,
            },
            main_value: 7431.0,
            daily_interest_rate: 0.00130821,
        };

        let due_dates = vec![
            chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 11, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2024, 12, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 01, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 02, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 03, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 04, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 05, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 06, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 07, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 08, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 09, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 10, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 11, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2026, 01, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2026, 02, 24).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2026, 03, 24).unwrap(),
        ];

        let last_due_date = chrono::NaiveDate::from_ymd_opt(2026, 03, 24).unwrap();

        let i_cal = InstallmentData {
            accumulated_days: vec![
                30, 61, 91, 122, 153, 181, 212, 242, 273, 303, 334, 365, 395, 426, 456, 487, 518,
                546,
            ],
            diffs: vec![
                30, 31, 30, 31, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 28,
            ],
            amount: 589.4399638402917,
            factor: 0.48977173114928746,
            accumulated_factor: 12.606881880871965,
            last_due_date,
            due_dates,
        };

        println!("{:?}", i_cal);

        let iof = super::calc(&params, &i_cal);

        assert_eq!(iof, 195.90259933000002);
    }
}
