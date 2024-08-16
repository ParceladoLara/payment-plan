use serde::{Deserialize, Serialize};

pub mod calc;
mod util;

#[derive(Debug, Clone, Copy, Deserialize)]
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

#[derive(Debug, Serialize)]
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
