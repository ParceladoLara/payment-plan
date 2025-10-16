use chrono::NaiveDate;
use rust_decimal::{Decimal, MathematicalOps};

use crate::{
    util::{add_months, diff_in_business_days, get_next_business_day},
    Invoice,
};

use super::InnerParams;

#[derive(Debug, PartialEq)]
pub struct InstallmentData {
    pub accumulated_days: Vec<i64>,
    pub accumulated_business_days: Vec<i64>,
    pub diffs: Vec<i64>,
    pub business_diffs: Vec<i64>,
    pub amount: Decimal,
    pub factor: Decimal,
    pub accumulated_factor: Decimal,
    pub last_due_date: NaiveDate,
    pub due_dates: Vec<NaiveDate>,
    pub invoices: Vec<Invoice>,
}

pub fn insert_price_table_on_invoices(
    invoices: &mut Vec<Invoice>,
    contract_amount: Decimal,
    installment_amount: Decimal,
    interest_rate: Decimal,
) {
    let mut amortization_amount = contract_amount;

    for i in invoices {
        let debit_service = interest_rate * amortization_amount;
        let main_iof_tac = installment_amount - debit_service;
        i.main_iof_tac = main_iof_tac;
        i.debit_service = debit_service;
        amortization_amount -= main_iof_tac;
    }
}

pub fn calc(inner_params: &InnerParams) -> InstallmentData {
    if inner_params.params.disbursement_only_on_business_days {
        return calc_installments_on_business_days(inner_params);
    } else {
        return calc_installments(inner_params);
    }
}

fn calc_installments(inner_params: &InnerParams) -> InstallmentData {
    let daily_interest_rate = inner_params.daily_interest_rate;
    let main_value = inner_params.main_value;

    let params = inner_params.params;

    let disbursement_date = params.disbursement_date;
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;

    let mut last_due_date = disbursement_date;
    let mut due_date = first_payment_date;
    let mut accumulated_days = 0;
    let mut accumulated_factor = Decimal::ZERO;

    let mut diffs = Vec::with_capacity(installments as usize);
    let mut accumulated_days_v = Vec::with_capacity(installments as usize);
    let mut due_dates = Vec::with_capacity(installments as usize);
    let mut invoices = Vec::with_capacity(installments as usize);

    let mut factor = Decimal::ZERO;

    let base_factor = Decimal::ONE / (Decimal::ONE + daily_interest_rate);

    for i in 0..installments {
        if i != 0 {
            last_due_date = due_date;
            due_date = add_months(due_date, 1);
        }

        due_dates.push(due_date);

        let diff = due_date.signed_duration_since(last_due_date).num_days();
        diffs.push(diff);
        accumulated_days += diff;
        factor = base_factor.powd(Decimal::from(accumulated_days));
        factor = factor.round_dp(15);

        accumulated_factor += factor;

        accumulated_days_v.push(accumulated_days);

        invoices.push(Invoice {
            accumulated_days: accumulated_days,
            factor,
            accumulated_factor,
            main_iof_tac: Decimal::ZERO,
            debit_service: Decimal::ZERO,
            due_date,
        });
    }

    let installment_amount = main_value / accumulated_factor;
    let installment_amount = installment_amount.round_dp(2);
    let amount = installment_amount;

    return InstallmentData {
        business_diffs: diffs.clone(),
        accumulated_business_days: accumulated_days_v.clone(),
        accumulated_days: accumulated_days_v,
        diffs,
        amount,
        factor,
        accumulated_factor,
        last_due_date: due_date,
        due_dates,
        invoices,
    };
}

fn calc_installments_on_business_days(inner_params: &InnerParams) -> InstallmentData {
    let daily_interest_rate = inner_params.daily_interest_rate;

    let params = inner_params.params;
    let main_value = inner_params.main_value;

    let disbursement_date = params.disbursement_date;
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;

    let mut last_due_date = disbursement_date;
    let mut due_date = first_payment_date;
    let base_due_date = inner_params.base_date;
    let mut accumulated_days = 0;
    let mut accumulated_business_days = 0;
    let mut accumulated_factor = Decimal::ZERO;

    let mut diffs = Vec::with_capacity(installments as usize);
    let mut business_diffs = Vec::with_capacity(installments as usize);
    let mut accumulated_days_v = Vec::with_capacity(installments as usize);
    let mut accumulated_business_days_v = Vec::with_capacity(installments as usize);
    let mut due_dates = Vec::with_capacity(installments as usize);
    let mut invoices = Vec::with_capacity(installments as usize);

    let mut factor = Decimal::ZERO;

    let base_factor = Decimal::ONE / (Decimal::ONE + daily_interest_rate);
    for i in 0..installments {
        due_date = add_months(base_due_date, i);
        due_date = get_next_business_day(due_date);

        due_dates.push(due_date);

        let diff = due_date.signed_duration_since(last_due_date).num_days();
        let b_diff = diff_in_business_days(last_due_date, due_date);

        diffs.push(diff);
        business_diffs.push(b_diff);

        accumulated_days += diff;
        accumulated_business_days += b_diff;

        factor = base_factor.powd(Decimal::from(accumulated_business_days));
        factor = factor.round_dp(15);

        accumulated_factor += factor;
        accumulated_days_v.push(accumulated_days);
        accumulated_business_days_v.push(accumulated_business_days);

        invoices.push(Invoice {
            accumulated_days,
            factor,
            accumulated_factor,
            main_iof_tac: Decimal::ZERO,
            debit_service: Decimal::ZERO,
            due_date,
        });

        last_due_date = due_date;
    }

    let installment_amount = main_value / accumulated_factor;
    let installment_amount = installment_amount.round_dp(2);
    let amount = installment_amount;

    return InstallmentData {
        accumulated_days: accumulated_days_v,
        diffs,
        accumulated_business_days: accumulated_business_days_v,
        business_diffs,
        amount,
        factor,
        accumulated_factor,
        last_due_date: due_date,
        due_dates,
        invoices,
    };
}

#[cfg(test)]
mod test {
    use rust_decimal::{dec, Decimal};

    use crate::{
        calc::providers::iterative::{installment::InstallmentData, InnerParams},
        Invoice, Params,
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
            amount: dec!(589.44),
            factor: dec!(0.489771731149302),
            accumulated_factor: dec!(12.60688188087214),
            last_due_date,
            due_dates,
            invoices: vec![
                Invoice {
                    accumulated_days: 30,
                    factor: dec!(0.961538521141742),
                    accumulated_factor: dec!(0.961538521141742),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 61,
                    factor: dec!(0.923348394036885),
                    accumulated_factor: dec!(1.884886915178627),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 91,
                    factor: dec!(0.887835049300829),
                    accumulated_factor: dec!(2.772721964479456),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 122,
                    factor: dec!(0.852572256770495),
                    accumulated_factor: dec!(3.625294221249951),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 01, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 153,
                    factor: dec!(0.818710022303302),
                    accumulated_factor: dec!(4.444004243553253),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 02, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 181,
                    factor: dec!(0.789282272705526),
                    accumulated_factor: dec!(5.2332865162587785),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 03, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 212,
                    factor: dec!(0.757933772719854),
                    accumulated_factor: dec!(5.991220288978632),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 04, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 242,
                    factor: dec!(0.72878251894443),
                    accumulated_factor: dec!(6.720002807923063),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 05, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 273,
                    factor: dec!(0.699836931827195),
                    accumulated_factor: dec!(7.419839739750258),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 06, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 303,
                    factor: dec!(0.672920168469495),
                    accumulated_factor: dec!(8.092759908219753),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 07, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 334,
                    factor: dec!(0.646193307090341),
                    accumulated_factor: dec!(8.738953215310094),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 08, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 365,
                    factor: dec!(0.620527975967898),
                    accumulated_factor: dec!(9.359481191277991),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 09, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 395,
                    factor: dec!(0.596661552339251),
                    accumulated_factor: dec!(9.956142743617242),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 426,
                    factor: dec!(0.572963510064917),
                    accumulated_factor: dec!(10.529106253682158),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 456,
                    factor: dec!(0.550926486136002),
                    accumulated_factor: dec!(11.08003273981816),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 12, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 487,
                    factor: dec!(0.529044936860178),
                    accumulated_factor: dec!(11.609077676678337),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 01, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 518,
                    factor: dec!(0.5080324730445),
                    accumulated_factor: dec!(12.117110149722837),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 02, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 546,
                    factor: dec!(0.489771731149302),
                    accumulated_factor: dec!(12.60688188087214),
                    main_iof_tac: Decimal::ZERO,
                    debit_service: Decimal::ZERO,
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 03, 24).unwrap(),
                },
            ],
        };

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
                iof_percentage: dec!(0.03),
                interest_rate: dec!(0.04),
                min_installment_amount: Decimal::ONE_HUNDRED,
                max_total_amount: Decimal::MAX,
            },
            main_value: dec!(7431.0),
            daily_interest_rate: dec!(0.00130821),
            base_date: first_payment_date,
        };

        let data = super::calc(&params);

        assert_eq!(data, expected);
    }
}
