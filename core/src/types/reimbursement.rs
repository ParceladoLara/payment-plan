use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Params {
    pub fee: f64,
    pub mdr: f64,
    pub invoice_cost: f64,
    pub interest_rate: f64,
    pub base_date: NaiveDate,
    pub max_repurchase_payment_days: i64,
    pub max_reimbursement_payment_days: i64,
    pub invoices: Vec<InvoiceParam>,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct InvoiceParam {
    pub id: u32,
    pub status: InvoiceStatus,
    pub original_amount: f64,
    pub due_at: NaiveDate,
    pub main_iof_tac: f64,
}

#[derive(Debug, Serialize, Clone, Default, PartialEq)]
pub struct Response {
    pub total_present_value_repurchase: f64,
    pub reimbursement_value: f64,
    pub reference_date_for_repurchase: NaiveDate,
    pub interest_rate_daily: f64,
    pub subsidy_for_cancellation: f64,
    pub customer_charge_back_amount: f64,
    pub invoices: Vec<InvoiceResponse>,
    pub reimbursement_invoice_due_date: NaiveDate,
}

#[derive(Debug, Serialize, Clone, Copy, Default, PartialEq)]
pub struct InvoiceResponse {
    pub id: u32,
    pub days_difference_between_repurchase_date_and_due_at: i64,
    pub present_value_repurchase: f64,
}

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq)]
pub enum InvoiceStatus {
    OVERDUE,
    #[default]
    CREATED,
    READJUSTED,
    PAID,
    IRRELEVANT,
}

impl From<&str> for InvoiceStatus {
    fn from(s: &str) -> Self {
        match s {
            "OVERDUE" => InvoiceStatus::OVERDUE,
            "CREATED" => InvoiceStatus::CREATED,
            "READJUSTED" => InvoiceStatus::READJUSTED,
            _ => InvoiceStatus::IRRELEVANT,
        }
    }
}

impl From<String> for InvoiceStatus {
    fn from(s: String) -> Self {
        return InvoiceStatus::from(s.as_str());
    }
}
