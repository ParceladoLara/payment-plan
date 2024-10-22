use chrono::NaiveDate;
use xirr::Payment;

const MONTH_AS_YEAR_FRACTION: f64 = 0.0821917808219178; // 30/365

pub mod eir;
pub mod tec;

pub fn prepare_xirr_params(
    installments: u32,
    due_dates: &Vec<NaiveDate>,
    calculation_basis_for_eir: f64,
    customer_amount: f64,
) -> (Vec<Payment>, Vec<Payment>) {
    let mut eir_params = Vec::new();
    let mut tec_params = Vec::new();

    let eir_amount = -1.0 * calculation_basis_for_eir;
    let tec_amount = -1.0 * customer_amount;

    for i in 0..installments {
        let date = due_dates[i as usize];

        eir_params.push(Payment {
            amount: eir_amount,
            date,
        });
        tec_params.push(Payment {
            amount: tec_amount,
            date,
        });
    }

    return (eir_params, tec_params);
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::calc::inner_xirr::prepare_xirr_params;

    #[test]
    fn test_prepare_xirr_params_test_7() {
        let base_month = 4;

        let due_dates = vec![
            chrono::NaiveDate::from_ymd_opt(2022, 4, 30).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2022, 5, 30).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2022, 6, 30).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2022, 7, 30).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2022, 8, 30).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2022, 9, 30).unwrap(),
        ];

        let installments = 1;
        let calculation_basis_for_effective_interest_rate = 3005.610014640465;
        let customer_amount = 3024.0190557363553;

        let (eir_params, tec_params) = prepare_xirr_params(
            installments,
            &due_dates,
            calculation_basis_for_effective_interest_rate,
            customer_amount,
        );

        assert_eq!(eir_params.len(), 1);
        assert_eq!(tec_params.len(), 1);

        assert_eq!(eir_params[0].amount, -3005.610014640465);
        assert_eq!(
            eir_params[0].date,
            NaiveDate::from_ymd_opt(2022, 04, 30).unwrap()
        );

        assert_eq!(tec_params[0].amount, -3024.0190557363553);
        assert_eq!(
            tec_params[0].date,
            NaiveDate::from_ymd_opt(2022, 04, 30).unwrap()
        );

        let installments = 2;
        let calculation_basis_for_effective_interest_rate = 1528.9066354183226;
        let customer_amount = 1539.8988271991445;

        let (eir_params, tec_params) = prepare_xirr_params(
            installments,
            &due_dates,
            calculation_basis_for_effective_interest_rate,
            customer_amount,
        );

        for i in 0..2 {
            assert_eq!(eir_params[i].amount, -1528.9066354183226);
            assert_eq!(
                eir_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );

            assert_eq!(tec_params[i].amount, -1539.8988271991445);
            assert_eq!(
                tec_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );
        }

        let installments = 3;
        let calculation_basis_for_effective_interest_rate = 1037.298256951948;
        let customer_amount = 1045.8446791163315;

        let (eir_params, tec_params) = prepare_xirr_params(
            installments,
            &due_dates,
            calculation_basis_for_effective_interest_rate,
            customer_amount,
        );

        for i in 0..3 {
            assert_eq!(eir_params[i].amount, -1037.298256951948);
            assert_eq!(
                eir_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );

            assert_eq!(tec_params[i].amount, -1045.8446791163315);
            assert_eq!(
                tec_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );
        }

        let installments = 4;
        let calculation_basis_for_effective_interest_rate = 791.5362879492445;
        let customer_amount = 798.8498495930802;

        let (eir_params, tec_params) = prepare_xirr_params(
            installments,
            &due_dates,
            calculation_basis_for_effective_interest_rate,
            customer_amount,
        );

        for i in 0..4 {
            assert_eq!(eir_params[i].amount, -791.5362879492445);
            assert_eq!(
                eir_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );

            assert_eq!(tec_params[i].amount, -798.8498495930802);
            assert_eq!(
                tec_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );
        }

        let installments = 5;
        let calculation_basis_for_effective_interest_rate = 644.3191099311389;
        let customer_amount = 650.8993291092211;

        let (eir_params, tec_params) = prepare_xirr_params(
            installments,
            &due_dates,
            calculation_basis_for_effective_interest_rate,
            customer_amount,
        );

        for i in 0..5 {
            assert_eq!(eir_params[i].amount, -644.3191099311389);
            assert_eq!(
                eir_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );

            assert_eq!(tec_params[i].amount, -650.8993291092211);
            assert_eq!(
                tec_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );
        }

        let installments = 6;
        let calculation_basis_for_effective_interest_rate = 546.3383247758576;
        let customer_amount = 552.4322553512001;

        let (eir_params, tec_params) = prepare_xirr_params(
            installments,
            &due_dates,
            calculation_basis_for_effective_interest_rate,
            customer_amount,
        );

        for i in 0..6 {
            assert_eq!(eir_params[i].amount, -546.3383247758576);
            assert_eq!(
                eir_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );

            assert_eq!(tec_params[i].amount, -552.4322553512001);
            assert_eq!(
                tec_params[i].date,
                NaiveDate::from_ymd_opt(2022, base_month + i as u32, 30).unwrap()
            );
        }
    }
}
