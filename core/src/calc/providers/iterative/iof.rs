use rust_decimal::{Decimal, MathematicalOps};

use super::{installment::InstallmentData, InnerParams};

pub fn calc(inner_params: &InnerParams, data: &InstallmentData) -> Decimal {
    let mut total_iof = Decimal::ZERO;
    let params = inner_params.params;
    let installments = params.installments;

    let iof_percentage = params.iof_percentage;

    let iof_overall = params.iof_overall;

    let daily_interest_rate = inner_params.daily_interest_rate;

    let main_value = inner_params.main_value;
    let mut main_value_l = main_value.round_dp(8);

    let installment_amount = data.amount;
    let mut acc_installment_amount_without_fee = Decimal::ZERO;
    for j in 0..installments {
        let mut accumulated_days = data.accumulated_days[j as usize];
        let business_diff = Decimal::from(data.business_diffs[j as usize]);
        let fee = main_value_l
            * ((Decimal::ONE + daily_interest_rate).powd(business_diff) - Decimal::ONE);

        let fee = fee.round_dp(7);

        let installment_amount_without_fee: Decimal;
        if j == installments - 1 {
            installment_amount_without_fee = main_value - acc_installment_amount_without_fee;
        } else {
            installment_amount_without_fee = installment_amount - fee;
        }

        let installment_amount_without_fee = installment_amount_without_fee.round_dp(8);

        let main_iof = installment_amount_without_fee * iof_overall;
        if accumulated_days >= 365 {
            accumulated_days = 365;
        }
        let main_iof = main_iof.round_dp(2);

        let installment_iof =
            installment_amount_without_fee * Decimal::from(accumulated_days) * iof_percentage;

        let installment_iof = installment_iof.round_dp(8);

        let iof = main_iof + installment_iof;

        total_iof += iof;
        main_value_l = main_value_l + fee - installment_amount;
        main_value_l = main_value_l.round_dp(8);
        acc_installment_amount_without_fee += installment_amount_without_fee;
    }
    let total_iof = total_iof.round_dp(2);
    return total_iof;
}

#[cfg(test)]
mod test {
    use rust_decimal::{dec, Decimal};

    use crate::{
        calc::providers::iterative::{installment::InstallmentData, InnerParams},
        Params,
    };

    #[test]
    fn test_calc() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 09, 24).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap();
        let params = InnerParams {
            params: Params {
                disbursement_only_on_business_days: false,
                requested_amount: dec!(7431.0),
                first_payment_date,
                disbursement_date: disbursement_date,
                installments: 18,
                debit_service_percentage: 0,
                mdr: dec!(0.05),
                tac_percentage: Decimal::ZERO,
                iof_overall: dec!(0.0038),
                iof_percentage: dec!(0.000082),
                interest_rate: dec!(0.04),
                min_installment_amount: Decimal::ONE_HUNDRED,
                max_total_amount: Decimal::MAX,
            },
            main_value: dec!(7431.0),
            daily_interest_rate: dec!(0.00130821),
            base_date: first_payment_date,
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
            accumulated_business_days: vec![
                30, 61, 91, 122, 153, 181, 212, 242, 273, 303, 334, 365, 395, 426, 456, 487, 518,
                546,
            ],
            diffs: vec![
                30, 31, 30, 31, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 28,
            ],
            business_diffs: vec![
                30, 31, 30, 31, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 28,
            ],
            amount: dec!(589.4399638402917),
            factor: dec!(0.48977173114928746),
            accumulated_factor: dec!(12.606881880871965),
            last_due_date,
            due_dates,
            invoices: vec![],
        };

        let iof = super::calc(&params, &i_cal);

        assert_eq!(iof, dec!(195.9));
    }
}
