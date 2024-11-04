use crate::{plan::Params, util::add_months};

#[derive(Debug, Clone, Copy)]
pub struct PreparedCalculation {
    pub installment: u32,
    pub due_date: chrono::NaiveDate,
    pub accumulated_days: i64,
    pub days_index: f64,
    pub accumulated_days_index: f64,
}

pub fn prepare_calculation(params: Params) -> Vec<PreparedCalculation> {
    let requested_date = params.requested_date;
    let mut prepared_calculations: Vec<PreparedCalculation> = Vec::new();
    let first_payment_date = params.first_payment_date;
    let installments = params.installments;
    let interest_rate = params.interest_rate;

    let mut due_date = first_payment_date;
    let divisor = 1.0 + interest_rate;
    let base = 1.0 / divisor;

    for i in 0..installments {
        if i != 0 {
            due_date = add_months(due_date, 1);
        }

        let accumulated_days = due_date.signed_duration_since(requested_date).num_days();
        let exponent = (accumulated_days as f64) / 30.0;
        let days_index = base.powf(exponent);
        let mut accumulated_days_index = days_index;
        for j in 0..i {
            accumulated_days_index += prepared_calculations[j as usize].days_index;
        }

        prepared_calculations.push(PreparedCalculation {
            installment: i + 1,
            due_date,
            accumulated_days,
            days_index,
            accumulated_days_index,
        });
    }

    return prepared_calculations;
}

#[cfg(test)]
mod test {

    /*
    Test 0 - (8800 / 24) = (11980.77027564256 / 499.1987614851067)
    Test 1 - (6000 / 18) = (7739.024786678216 / 429.9458214821231)
    Test 2 - (1300 / 12) = (1541.6623345164212 / 128.47186120970176)
    Test 3 - (1600 / 9) = (1831.00234095926 / 203.44470455102888)
    Test 4 - (1000 / 9) = (1140.115221691851 / 126.67946907687234)
    Test 5 - (4580 / 24) = (7070.7838245293115 / 294.6159926887213)
    Test 6 - (1500 / 12) = (1795.186723818578 / 149.59889365154817)
    Test 7 - (2900 / 6) = (3314.5935321072 / 552.4322553512001)
     */

    use crate::{calc::providers::bmp::prepare::prepare_calculation, plan::Params};

    #[test]
    fn test_prepare_calculus_test_0() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 8800.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 18).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 18).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0235,
        };

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 24);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 04, 18).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 31);
        assert!(prepared_calculations[0].days_index == 0.9762833696137795);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9762833696137795);

        //index 10
        assert!(prepared_calculations[10].installment == 11);
        assert!(
            prepared_calculations[10].due_date
                == chrono::NaiveDate::from_ymd_opt(2023, 02, 18).unwrap()
        );
        assert!(prepared_calculations[10].accumulated_days == 337);
        assert!(prepared_calculations[10].days_index == 0.7703353931843917);
        assert!(prepared_calculations[10].accumulated_days_index == 9.568877755632895);

        //index 23
        assert!(prepared_calculations[23].installment == 24);
        assert!(
            prepared_calculations[23].due_date
                == chrono::NaiveDate::from_ymd_opt(2024, 03, 18).unwrap()
        );
        assert!(prepared_calculations[23].accumulated_days == 731);
        assert!(prepared_calculations[23].days_index == 0.567796609395405);
        assert!(prepared_calculations[23].accumulated_days_index == 18.10364842025564);
    }

    #[test]
    fn test_prepare_calculus_test_1() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 6000.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 06, 18).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 17).unwrap(),
            installments: 18,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.025,
        };

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 18);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 06, 18).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 32);
        assert!(prepared_calculations[0].days_index == 0.9740050536866598);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9740050536866598);

        //index 10
        assert!(prepared_calculations[10].installment == 11);
        assert!(
            prepared_calculations[10].due_date
                == chrono::NaiveDate::from_ymd_opt(2023, 04, 18).unwrap()
        );
        assert!(prepared_calculations[10].accumulated_days == 336);
        assert!(prepared_calculations[10].days_index == 0.7583901916983443);
        assert!(prepared_calculations[10].accumulated_days_index == 9.480262398672277);

        //index 17
        assert!(prepared_calculations[17].installment == 18);
        assert!(
            prepared_calculations[17].due_date
                == chrono::NaiveDate::from_ymd_opt(2023, 11, 18).unwrap()
        );
        assert!(prepared_calculations[17].accumulated_days == 550);
        assert!(prepared_calculations[17].days_index == 0.6359102146827685);
        assert!(prepared_calculations[17].accumulated_days_index == 14.287911534617638);
    }

    #[test]
    fn test_prepare_calculus_test_2() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1300.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 21).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 21).unwrap(),
            installments: 12,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0235,
        };

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 12);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 04, 21).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 31);
        assert!(prepared_calculations[0].days_index == 0.9762833696137795);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9762833696137795);

        //index 6
        assert!(prepared_calculations[6].installment == 7);
        assert!(
            prepared_calculations[6].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 10, 21).unwrap()
        );
        assert!(prepared_calculations[6].accumulated_days == 214);
        assert!(prepared_calculations[6].days_index == 0.8473054985378986); // 0.8473054985378985 is the expected value,see later
        assert!(prepared_calculations[6].accumulated_days_index == 6.374065955281998);

        //index 11
        assert!(prepared_calculations[11].installment == 12);
        assert!(
            prepared_calculations[11].due_date
                == chrono::NaiveDate::from_ymd_opt(2023, 03, 21).unwrap()
        );
        assert!(prepared_calculations[11].accumulated_days == 365);
        assert!(prepared_calculations[11].days_index == 0.753814571370284);
        assert!(prepared_calculations[11].accumulated_days_index == 10.322692327003178);
    }

    #[test]
    fn test_prepare_calculus_test_3() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1600.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 29).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 29).unwrap(),
            installments: 9,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.024,
        };

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 9);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 05, 29).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 30);
        assert!(prepared_calculations[0].days_index == 0.9765625);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9765625);

        //index 3
        assert!(prepared_calculations[3].installment == 4);
        assert!(
            prepared_calculations[3].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 08, 29).unwrap()
        );
        assert!(prepared_calculations[3].accumulated_days == 122);
        assert!(prepared_calculations[3].days_index == 0.9080578343022551); //908057834302255
        assert!(prepared_calculations[3].accumulated_days_index == 3.7681276282380630); //3.7681276282380627

        //index 8
        assert!(prepared_calculations[8].installment == 9);
        assert!(
            prepared_calculations[8].due_date
                == chrono::NaiveDate::from_ymd_opt(2023, 01, 29).unwrap()
        );
        assert!(prepared_calculations[8].accumulated_days == 275);
        assert!(prepared_calculations[8].days_index == 0.8046068596259314); //8046068596259313
        assert!(prepared_calculations[8].accumulated_days_index == 7.993043764995125);
    }

    #[test]
    fn test_prepare_calculus_test_4() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 1000.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 08).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 10).unwrap(),
            installments: 9,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0235,
        };

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 9);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 04, 08).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 29);
        assert!(prepared_calculations[0].days_index == 0.9777963563221375);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9777963563221375);

        //index 3
        assert!(prepared_calculations[3].installment == 4);
        assert!(
            prepared_calculations[3].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 07, 08).unwrap()
        );
        assert!(prepared_calculations[3].accumulated_days == 120);
        assert!(prepared_calculations[3].days_index == 0.9112732291360666);
        assert!(prepared_calculations[3].accumulated_days_index == 3.7771034671078274);

        //index 8
        assert!(prepared_calculations[8].installment == 9);
        assert!(
            prepared_calculations[8].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 12, 08).unwrap()
        );
        assert!(prepared_calculations[8].accumulated_days == 273);
        assert!(prepared_calculations[8].days_index == 0.8094696914138441);
        assert!(prepared_calculations[8].accumulated_days_index == 8.021906029443949);
    }

    #[test]
    fn test_prepare_calculus_test_5() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 4580.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 05, 05).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 04).unwrap(),
            installments: 24,
            debit_service_percentage: 0,
            mdr: 0.01,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.0349,
        };

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 24);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 05, 05).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 31);
        assert!(prepared_calculations[0].days_index == 0.9651726351175118);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9651726351175118);

        //index 10
        assert!(prepared_calculations[10].installment == 11);
        assert!(
            prepared_calculations[10].due_date
                == chrono::NaiveDate::from_ymd_opt(2023, 03, 05).unwrap()
        );
        assert!(prepared_calculations[10].accumulated_days == 335);
        assert!(prepared_calculations[10].days_index == 0.6817649641421434);
        assert!(prepared_calculations[10].accumulated_days_index == 8.967225722964963);

        //index 23
        assert!(prepared_calculations[23].installment == 24);
        assert!(
            prepared_calculations[23].due_date
                == chrono::NaiveDate::from_ymd_opt(2024, 04, 05).unwrap()
        );
        assert!(prepared_calculations[23].accumulated_days == 732);
        assert!(prepared_calculations[23].days_index == 0.43299148769124457); // 0.4329914876912446
        assert!(prepared_calculations[23].accumulated_days_index == 15.96509782080605);
    }

    #[test]
    fn test_prepare_calculus_test_6() {
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

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 12);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 06, 09).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 31);
        assert!(prepared_calculations[0].days_index == 0.9748070746896711);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9748070746896711);

        //index 6
        assert!(prepared_calculations[6].installment == 7);
        assert!(
            prepared_calculations[6].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 12, 09).unwrap()
        );
        assert!(prepared_calculations[6].accumulated_days == 214);
        assert!(prepared_calculations[6].days_index == 0.8385000513504884); // 0.8385000513504886
        assert!(prepared_calculations[6].accumulated_days_index == 6.336263891503212);

        //index 11
        assert!(prepared_calculations[11].installment == 12);
        assert!(
            prepared_calculations[11].due_date
                == chrono::NaiveDate::from_ymd_opt(2023, 05, 09).unwrap()
        );
        assert!(prepared_calculations[11].accumulated_days == 365);
        assert!(prepared_calculations[11].days_index == 0.7405021169133975);
        assert!(prepared_calculations[11].accumulated_days_index == 10.228570809330265);
        //10.228570809330268
    }

    #[test]
    fn test_prepare_calculus_test_7() {
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

        let prepared_calculations = prepare_calculation(params);

        assert!(prepared_calculations.len() == 6);

        // index 0
        assert!(prepared_calculations[0].installment == 1);
        assert!(
            prepared_calculations[0].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap()
        );
        assert!(prepared_calculations[0].accumulated_days == 31);
        assert!(prepared_calculations[0].days_index == 0.9650762734315015);
        assert!(prepared_calculations[0].accumulated_days_index == 0.9650762734315015);

        //index 3
        assert!(prepared_calculations[3].installment == 4);
        assert!(
            prepared_calculations[3].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 07, 30).unwrap()
        );
        assert!(prepared_calculations[3].accumulated_days == 122);
        assert!(prepared_calculations[3].days_index == 0.8694459273639379);
        assert!(prepared_calculations[3].accumulated_days_index == 3.6668395795122857);

        //index 5
        assert!(prepared_calculations[5].installment == 6);
        assert!(
            prepared_calculations[5].due_date
                == chrono::NaiveDate::from_ymd_opt(2022, 09, 30).unwrap()
        );
        assert!(prepared_calculations[5].accumulated_days == 184);
        assert!(prepared_calculations[5].days_index == 0.8097777779226664); // 0.8097777779226665 is the expected value,see later
        assert!(prepared_calculations[5].accumulated_days_index == 5.315698992965537);
    }
}
