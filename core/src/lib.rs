use std::fmt::Display;

use calc::PaymentPlan;
use err::PaymentPlanError;
use serde::{Deserialize, Serialize};

#[cfg(feature = "iterative")]
use calc::providers::iterative::Iterative;
#[cfg(feature = "simple")]
use calc::providers::simple::Simple;

// Default to BMP if no feature is specified
#[cfg(not(any(feature = "simple", feature = "iterative")))]
use calc::providers::iterative::Iterative;

mod calc;
mod err;
mod util;

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq, Serialize)]
pub struct Installment {
    pub accumulated_days: i64,
    pub factor: f64,
    pub accumulated_factor: f64,
    pub due_date: chrono::NaiveDate,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Params {
    pub requested_amount: f64,
    pub first_payment_date: chrono::NaiveDate,
    pub disbursement_date: chrono::NaiveDate,
    pub installments: u32,
    pub debit_service_percentage: u16, // 0-100
    pub mdr: f64,                      // 0.0-1.0
    pub tac_percentage: f64,           // 0.0-1.0
    pub iof_overall: f64,              // 0.0-1.0
    pub iof_percentage: f64,           // 0.0-1.0
    pub interest_rate: f64,            // 0.0-1.0
    pub min_installment_amount: f64,
    pub max_total_amount: f64,
    pub disbursement_only_on_business_days: bool,
}

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Params {{ requested_amount: {}, first_payment_date: {}, disbursement_date: {}, installments: {}, debit_service_percentage: {}, mdr: {}, tac_percentage: {}, iof_overall: {}, iof_percentage: {}, interest_rate: {}, min_installment_amount: {}, max_total_amount: {} }}",
            self.requested_amount,
            self.first_payment_date,
            self.disbursement_date,
            self.installments,
            self.debit_service_percentage,
            self.mdr,
            self.tac_percentage,
            self.iof_overall,
            self.iof_percentage,
            self.interest_rate,
            self.min_installment_amount,
            self.max_total_amount
        )
    }
}

#[derive(Debug, Serialize, Clone, Default, PartialEq)]
pub struct Response {
    pub installment: u32,
    pub due_date: chrono::NaiveDate,
    pub disbursement_date: chrono::NaiveDate,
    pub accumulated_days: i64,
    pub days_index: f64,
    pub accumulated_days_index: f64,
    pub interest_rate: f64,
    pub installment_amount: f64,
    pub installment_amount_without_tac: f64,
    pub total_amount: f64,
    pub debit_service: f64,
    pub customer_debit_service_amount: f64,
    pub customer_amount: f64,
    pub calculation_basis_for_effective_interest_rate: f64,
    pub merchant_debit_service_amount: f64,
    pub merchant_total_amount: f64,
    pub settled_to_merchant: f64,
    pub mdr_amount: f64,
    pub effective_interest_rate: f64,
    pub total_effective_cost: f64,
    pub eir_yearly: f64,
    pub tec_yearly: f64,
    pub eir_monthly: f64,
    pub tec_monthly: f64,
    pub total_iof: f64,
    pub contract_amount: f64,
    pub contract_amount_without_tac: f64,
    pub tac_amount: f64,
    pub iof_percentage: f64,
    pub overall_iof: f64,
    pub pre_disbursement_amount: f64,
    pub paid_total_iof: f64,
    pub paid_contract_amount: f64,
    pub installments: Vec<Installment>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct DownPaymentParams {
    pub params: Params,              // The params for the actual payment plan
    pub requested_amount: f64,       // The requested amount for the down payment(ex: 1000.0)
    pub min_installment_amount: f64, // The minium installment value for the down payment (ex: 100.0)
    pub first_payment_date: chrono::NaiveDate, // The first payment date for the down payment
    pub installments: u32,           // The max number of installments for the down payment (ex: 12)
}

//This struct can't derive Copy because it contains a Vec that is not known at compile time
#[derive(Debug, Serialize, Clone)]
pub struct DownPaymentResponse {
    pub installment_amount: f64, // The installment amount for the down payment
    pub total_amount: f64,       // The total amount for the down payment
    pub installment_quantity: u32, // The number of installments for the down payment
    pub first_payment_date: chrono::NaiveDate, // The first payment date for the down payment
    pub plans: Vec<Response>,    // The payment plans available for the down payment
}

#[cfg(feature = "simple")]
const P: Simple = Simple {};
#[cfg(feature = "iterative")]
const P: Iterative = Iterative {};

// Default to BMP if no feature is specified
#[cfg(not(any(feature = "simple", feature = "iterative")))]
const P: Iterative = Iterative {};

pub fn calculate_down_payment_plan(
    params: DownPaymentParams,
) -> Result<Vec<DownPaymentResponse>, PaymentPlanError> {
    return P.calculate_down_payment_plan(params);
}

pub fn calculate_payment_plan(params: Params) -> Result<Vec<Response>, PaymentPlanError> {
    return P.calculate_payment_plan(params);
}

pub fn next_disbursement_date(mut base_date: chrono::NaiveDate) -> chrono::NaiveDate {
    let today = chrono::Local::now().date_naive();
    if base_date == today {
        base_date = util::add_days(base_date, 1);
    }

    return util::get_next_business_day(base_date);
}

pub fn disbursement_date_range(
    base_date: chrono::NaiveDate,
    days: u32,
) -> (chrono::NaiveDate, chrono::NaiveDate) {
    let start_date = next_disbursement_date(base_date);

    let mut end_date = start_date;
    let mut i = 1;
    while i < days {
        end_date = util::add_days(end_date, 1);
        if util::is_business_day(end_date) {
            i += 1;
        }
    }

    return (start_date, end_date);
}

pub fn get_non_business_days_between(
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> Vec<chrono::NaiveDate> {
    return util::get_non_business_days_between(start_date, end_date);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_next_disbursement_date() {
        let base_date = chrono::NaiveDate::from_ymd_opt(2078, 02, 12).unwrap();
        let result = super::next_disbursement_date(base_date);
        let expected = chrono::NaiveDate::from_ymd_opt(2078, 02, 16).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_disbursement_data_range() {
        let base_date = chrono::NaiveDate::from_ymd_opt(2078, 02, 12).unwrap();
        let days = 5;
        let result = super::disbursement_date_range(base_date, days);
        let expected = (
            chrono::NaiveDate::from_ymd_opt(2078, 02, 16).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2078, 02, 22).unwrap(),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_non_business_days_between() {
        let start_date = chrono::NaiveDate::from_ymd_opt(2078, 11, 12).unwrap();
        let end_date = chrono::NaiveDate::from_ymd_opt(2078, 11, 22).unwrap();
        let result = super::get_non_business_days_between(start_date, end_date);
        let expected = vec![
            chrono::NaiveDate::from_ymd_opt(2078, 11, 12).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2078, 11, 13).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2078, 11, 15).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2078, 11, 19).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2078, 11, 20).unwrap(),
        ];
        assert_eq!(result, expected);
    }
}
