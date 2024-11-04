use serde::{Deserialize, Serialize};

use super::plan;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Params {
    pub params: plan::Params,        // The params for the actual payment plan
    pub requested_amount: f64,       // The requested amount for the down payment(ex: 1000.0)
    pub min_installment_amount: f64, // The minium installment value for the down payment (ex: 100.0)
    pub first_payment_date: chrono::NaiveDate, // The first payment date for the down payment
    pub installments: u32,           // The max number of installments for the down payment (ex: 12)
}

//This struct can't derive Copy because it contains a Vec that is not known at compile time
#[derive(Debug, Serialize, Clone)]
pub struct Response {
    pub installment_amount: f64, // The installment amount for the down payment
    pub total_amount: f64,       // The total amount for the down payment
    pub installment_quantity: u32, // The number of installments for the down payment
    pub first_payment_date: chrono::NaiveDate, // The first payment date for the down payment
    pub plans: Vec<plan::Response>, // The payment plans available for the down payment
}
