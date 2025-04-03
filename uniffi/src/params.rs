use std::time::SystemTime;

use chrono::{DateTime, Utc};

#[derive(uniffi::Record)]
pub struct Params {
    pub requested_amount: f64,
    pub first_payment_date: SystemTime,
    pub requested_date: SystemTime,
    pub installments: u32,
    pub debit_service_percentage: u16,
    pub mdr: f64,
    pub tac_percentage: f64,
    pub iof_overall: f64,
    pub iof_percentage: f64,
    pub interest_rate: f64,
    pub min_installment_amount: f64,
    pub max_total_amount: f64,
    pub disbursement_only_on_business_days: bool,
}

impl Into<core_payment_plan::Params> for Params {
    fn into(self) -> core_payment_plan::Params {
        let requested_date: DateTime<Utc> = self.requested_date.into();
        let first_payment_date: DateTime<Utc> = self.first_payment_date.into();

        let requested_date = requested_date.date_naive();
        let first_payment_date = first_payment_date.date_naive();

        core_payment_plan::Params {
            requested_amount: self.requested_amount,
            first_payment_date,
            requested_date,
            installments: self.installments,
            debit_service_percentage: self.debit_service_percentage,
            mdr: self.mdr,
            tac_percentage: self.tac_percentage,
            iof_overall: self.iof_overall,
            iof_percentage: self.iof_percentage,
            interest_rate: self.interest_rate,
            min_installment_amount: self.min_installment_amount,
            max_total_amount: self.max_total_amount,
            disbursement_only_on_business_days: self.disbursement_only_on_business_days,
        }
    }
}

#[derive(uniffi::Record)]
pub struct DownPaymentParams {
    pub params: Params,                 // The params for the actual payment plan
    pub requested_amount: f64,          // The requested amount for the down payment(ex: 1000.0)
    pub min_installment_amount: f64, // The minium installment value for the down payment (ex: 100.0)
    pub first_payment_date: SystemTime, // The first payment date for the down payment
    pub installments: u32,           // The max number of installments for the down payment (ex: 12)
}

impl Into<core_payment_plan::DownPaymentParams> for DownPaymentParams {
    fn into(self) -> core_payment_plan::DownPaymentParams {
        let first_payment_date: DateTime<Utc> = self.first_payment_date.into();
        let first_payment_date = first_payment_date.date_naive();

        core_payment_plan::DownPaymentParams {
            params: self.params.into(),
            requested_amount: self.requested_amount,
            min_installment_amount: self.min_installment_amount,
            first_payment_date,
            installments: self.installments,
        }
    }
}
