use calc::PaymentPlan;
use err::PaymentPlanError;
use serde::{Deserialize, Serialize};

#[cfg(feature = "bmp")]
use calc::providers::bmp::BMP;
#[cfg(feature = "qitech")]
use calc::providers::qi_tech::QiTech;

// Default to BMP if no feature is specified
#[cfg(not(any(feature = "bmp", feature = "qitech")))]
use calc::providers::qi_tech::QiTech;

mod calc;
mod err;
mod util;

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Params {
    pub requested_amount: f64,
    pub first_payment_date: chrono::NaiveDate,
    pub requested_date: chrono::NaiveDate,
    pub installments: u32,
    pub debit_service_percentage: u16, // 0-100
    pub mdr: f64,                      // 0.0-1.0
    pub tac_percentage: f64,           // 0.0-1.0
    pub iof_overall: f64,              // 0.0-1.0
    pub iof_percentage: f64,           // 0.0-1.0
    pub interest_rate: f64,            // 0.0-1.0
    pub min_installment_amount: f64,
    pub max_total_amount: f64,
}

#[derive(Debug, Serialize, Clone, Copy, Default)]
pub struct Response {
    pub installment: u32,
    pub due_date: chrono::NaiveDate,
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

#[cfg(feature = "bmp")]
const P: BMP = BMP {};
#[cfg(feature = "qitech")]
const P: QiTech = QiTech {};

// Default to BMP if no feature is specified
#[cfg(not(any(feature = "bmp", feature = "qitech")))]
const P: QiTech = QiTech {};

pub fn calculate_down_payment_plan(
    params: DownPaymentParams,
) -> Result<Vec<DownPaymentResponse>, PaymentPlanError> {
    return P.calculate_down_payment_plan(params);
}

pub fn calculate_payment_plan(params: Params) -> Result<Vec<Response>, PaymentPlanError> {
    return P.calculate_payment_plan(params);
}
