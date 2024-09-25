use chrono::NaiveDate;

use crate::util::add_months;

use super::QiTechParams;

#[derive(Debug, PartialEq)]
pub struct InstallmentData {
    pub accumulated_days: Vec<i64>,
    pub diffs: Vec<i64>,
    pub amount: f64,
    pub factor: f64,
    pub accumulated_factor: f64,
    pub last_due_date: NaiveDate,
    pub due_dates: Vec<NaiveDate>,
}

pub fn calc(qi_params: &QiTechParams) -> InstallmentData {
    let daily_interest_rate = qi_params.daily_interest_rate;

    let params = qi_params.params;

    let requested_date = params.requested_date;
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;

    let mut last_due_date = requested_date;
    let mut due_date = first_payment_date;
    let mut accumulated_days = 0;
    let mut accumulated_factor = 0.0;

    let mut diffs = Vec::with_capacity(installments as usize);
    let mut accumulated_days_v = Vec::with_capacity(installments as usize);
    let mut due_dates = Vec::with_capacity(installments as usize);

    let mut instalment_amount_result = 0.0;

    let mut factor = 0.0;

    for i in 0..installments {
        let main_value = qi_params.main_value;
        if i != 0 {
            last_due_date = due_date;
            due_date = add_months(due_date, 1);
        }

        due_dates.push(due_date);

        let diff = due_date.signed_duration_since(last_due_date).num_days();
        diffs.push(diff);
        accumulated_days += diff;
        factor = 1.0 / (1.0 + daily_interest_rate).powf(accumulated_days as f64);

        accumulated_factor += factor;
        let installment_amount = main_value / accumulated_factor;
        let installment_amount = installment_amount;
        accumulated_days_v.push(accumulated_days);

        instalment_amount_result = installment_amount;
    }
    return InstallmentData {
        accumulated_days: accumulated_days_v,
        diffs,
        amount: instalment_amount_result,
        factor,
        accumulated_factor,
        last_due_date: due_date,
        due_dates,
    };
}

#[cfg(test)]
mod test {
    use crate::{
        calc::providers::qi_tech::{installment::InstallmentData, QiTechParams},
        Params,
    };

    #[test]
    fn test_calc() {
        let last_due_date = chrono::NaiveDate::from_ymd_opt(2026, 03, 24).unwrap();
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

        let expected = InstallmentData {
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

        let data = super::calc(&params);

        assert_eq!(data, expected);
    }
}
