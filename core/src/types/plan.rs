use std::fmt::Display;

use serde::{Deserialize, Serialize};

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

impl Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Params {{ requested_amount: {}, first_payment_date: {}, requested_date: {}, installments: {}, debit_service_percentage: {}, mdr: {}, tac_percentage: {}, iof_overall: {}, iof_percentage: {}, interest_rate: {}, min_installment_amount: {}, max_total_amount: {} }}",
            self.requested_amount,
            self.first_payment_date,
            self.requested_date,
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

#[derive(Debug, Serialize, Clone, Copy, Default, PartialEq)]
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
