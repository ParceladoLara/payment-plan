use chrono::NaiveDate;

use crate::{
    util::{add_months, diff_in_business_days, get_next_business_day, round_decimal_cases},
    Installment,
};

use super::QiTechParams;

#[derive(Debug, PartialEq)]
pub struct InstallmentData {
    pub accumulated_days: Vec<i64>,
    pub accumulated_business_days: Vec<i64>,
    pub diffs: Vec<i64>,
    pub business_diffs: Vec<i64>,
    pub amount: f64,
    pub factor: f64,
    pub accumulated_factor: f64,
    pub last_due_date: NaiveDate,
    pub due_dates: Vec<NaiveDate>,
    pub installments: Vec<Installment>,
}

pub fn calc(qi_params: &QiTechParams) -> InstallmentData {
    if qi_params.params.disbursement_only_on_business_days {
        return calc_installments_on_business_days(qi_params);
    } else {
        return calc_installments(qi_params);
    }
}

fn calc_installments(qi_params: &QiTechParams) -> InstallmentData {
    let daily_interest_rate = qi_params.daily_interest_rate;

    let params = qi_params.params;

    let disbursement_date = params.disbursement_date;
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;

    let mut last_due_date = disbursement_date;
    let mut due_date = first_payment_date;
    let mut accumulated_days = 0;
    let mut accumulated_factor = 0.0;

    let mut diffs = Vec::with_capacity(installments as usize);
    let mut accumulated_days_v = Vec::with_capacity(installments as usize);
    let mut due_dates = Vec::with_capacity(installments as usize);
    let mut installments_v = Vec::with_capacity(installments as usize);

    let mut instalment_amount_result = 0.0;

    let mut factor = 0.0;

    let base_factor = 1.0 / (1.0 + daily_interest_rate);

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
        factor = base_factor.powf(accumulated_days as f64);
        factor = round_decimal_cases(factor, 15);

        accumulated_factor += factor;
        let installment_amount = main_value / accumulated_factor;
        let installment_amount = round_decimal_cases(installment_amount, 2);
        accumulated_days_v.push(accumulated_days);

        instalment_amount_result = installment_amount;
        installments_v.push(Installment {
            accumulated_days: accumulated_days,
            factor,
            accumulated_factor,
            installment_amount: instalment_amount_result,
            due_date,
        });
    }
    return InstallmentData {
        business_diffs: diffs.clone(),
        accumulated_business_days: accumulated_days_v.clone(),
        accumulated_days: accumulated_days_v,
        diffs,
        amount: instalment_amount_result,
        factor,
        accumulated_factor,
        last_due_date: due_date,
        due_dates,
        installments: installments_v,
    };
}

fn calc_installments_on_business_days(qi_params: &QiTechParams) -> InstallmentData {
    let daily_interest_rate = qi_params.daily_interest_rate;

    let params = qi_params.params;

    let disbursement_date = params.disbursement_date;
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;

    let mut last_due_date = disbursement_date;
    let mut due_date = first_payment_date;
    let base_due_date = due_date;
    let mut accumulated_days = 0;
    let mut accumulated_business_days = 0;
    let mut accumulated_factor = 0.0;

    let mut diffs = Vec::with_capacity(installments as usize);
    let mut business_diffs = Vec::with_capacity(installments as usize);
    let mut accumulated_days_v = Vec::with_capacity(installments as usize);
    let mut accumulated_business_days_v = Vec::with_capacity(installments as usize);
    let mut due_dates = Vec::with_capacity(installments as usize);
    let mut installments_v = Vec::with_capacity(installments as usize);

    let mut instalment_amount_result = 0.0;

    let mut factor = 0.0;

    let base_factor = 1.0 / (1.0 + daily_interest_rate);
    for i in 0..installments {
        let main_value = qi_params.main_value;

        due_date = add_months(base_due_date, i);
        due_date = get_next_business_day(due_date);

        due_dates.push(due_date);

        let diff = due_date.signed_duration_since(last_due_date).num_days();
        let b_diff = diff_in_business_days(last_due_date, due_date);

        diffs.push(diff);
        business_diffs.push(b_diff);

        accumulated_days += diff;
        accumulated_business_days += b_diff;

        factor = base_factor.powf(accumulated_business_days as f64);
        factor = round_decimal_cases(factor, 15);

        accumulated_factor += factor;
        let installment_amount = main_value / accumulated_factor;
        let installment_amount = round_decimal_cases(installment_amount, 2);
        accumulated_days_v.push(accumulated_days);
        accumulated_business_days_v.push(accumulated_business_days);

        instalment_amount_result = installment_amount;
        installments_v.push(Installment {
            accumulated_days,
            factor,
            accumulated_factor,
            installment_amount,
            due_date,
        });

        last_due_date = due_date;
    }
    return InstallmentData {
        accumulated_days: accumulated_days_v,
        diffs,
        accumulated_business_days: accumulated_business_days_v,
        business_diffs,
        amount: instalment_amount_result,
        factor,
        accumulated_factor,
        last_due_date: due_date,
        due_dates,
        installments: installments_v,
    };
}

#[cfg(test)]
mod test {
    use crate::{
        calc::providers::qi_tech::{installment::InstallmentData, QiTechParams},
        Installment, Params,
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
            amount: 589.44,
            factor: 0.489771731149302,
            accumulated_factor: 12.60688188087214,
            last_due_date,
            due_dates,
            installments: vec![
                Installment {
                    accumulated_days: 30,
                    factor: 0.961538521141742,
                    accumulated_factor: 0.961538521141742,
                    installment_amount: 7728.24,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 61,
                    factor: 0.923348394036885,
                    accumulated_factor: 1.884886915178627,
                    installment_amount: 3942.41,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 91,
                    factor: 0.887835049300829,
                    accumulated_factor: 2.772721964479456,
                    installment_amount: 2680.04,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 122,
                    factor: 0.852572256770495,
                    accumulated_factor: 3.625294221249951,
                    installment_amount: 2049.76,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 01, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 153,
                    factor: 0.818710022303302,
                    accumulated_factor: 4.444004243553253,
                    installment_amount: 1672.14,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 02, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 181,
                    factor: 0.789282272705526,
                    accumulated_factor: 5.2332865162587785,
                    installment_amount: 1419.95,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 03, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 212,
                    factor: 0.757933772719854,
                    accumulated_factor: 5.991220288978632,
                    installment_amount: 1240.31,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 04, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 242,
                    factor: 0.72878251894443,
                    accumulated_factor: 6.720002807923063,
                    installment_amount: 1105.8,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 05, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 273,
                    factor: 0.699836931827195,
                    accumulated_factor: 7.419839739750258,
                    installment_amount: 1001.5,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 06, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 303,
                    factor: 0.672920168469495,
                    accumulated_factor: 8.092759908219753,
                    installment_amount: 918.23,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 07, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 334,
                    factor: 0.646193307090341,
                    accumulated_factor: 8.738953215310094,
                    installment_amount: 850.33,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 08, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 365,
                    factor: 0.620527975967898,
                    accumulated_factor: 9.359481191277991,
                    installment_amount: 793.95,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 09, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 395,
                    factor: 0.596661552339251,
                    accumulated_factor: 9.956142743617242,
                    installment_amount: 746.37,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 426,
                    factor: 0.572963510064917,
                    accumulated_factor: 10.529106253682158,
                    installment_amount: 705.76,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 456,
                    factor: 0.550926486136002,
                    accumulated_factor: 11.08003273981816,
                    installment_amount: 670.67,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 487,
                    factor: 0.529044936860178,
                    accumulated_factor: 11.609077676678337,
                    installment_amount: 640.1,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 01, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 518,
                    factor: 0.5080324730445,
                    accumulated_factor: 12.117110149722837,
                    installment_amount: 613.27,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 02, 24).unwrap(),
                },
                Installment {
                    accumulated_days: 546,
                    factor: 0.489771731149302,
                    accumulated_factor: 12.60688188087214,
                    installment_amount: 589.44,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 03, 24).unwrap(),
                },
            ],
        };

        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 09, 24).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap();
        let params = QiTechParams {
            params: Params {
                disbursement_only_on_business_days: false,
                requested_amount: 7431.0,
                first_payment_date,
                disbursement_date: disbursement_date,
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
