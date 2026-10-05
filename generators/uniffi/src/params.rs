use std::time::SystemTime;

use chrono::{DateTime, Utc};

#[derive(uniffi::Record)]
pub struct InternalParams {
    pub requested_amount: f64,
    pub first_payment_date: SystemTime,
    pub disbursement_date: SystemTime,
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
    pub min_installments: Option<u32>,
}

impl From<InternalParams> for core_payment_plan::Params {
    fn from(val: InternalParams) -> Self {
        let disbursement_date: DateTime<Utc> = val.disbursement_date.into();
        let first_payment_date: DateTime<Utc> = val.first_payment_date.into();

        let disbursement_date = disbursement_date.date_naive();
        let first_payment_date = first_payment_date.date_naive();

        core_payment_plan::Params {
            requested_amount: val.requested_amount,
            first_payment_date,
            disbursement_date,
            installments: val.installments,
            debit_service_percentage: val.debit_service_percentage,
            mdr: val.mdr,
            tac_percentage: val.tac_percentage,
            iof_overall: val.iof_overall,
            iof_percentage: val.iof_percentage,
            interest_rate: val.interest_rate,
            min_installment_amount: val.min_installment_amount,
            max_total_amount: val.max_total_amount,
            disbursement_only_on_business_days: val.disbursement_only_on_business_days,
            min_installments: val.min_installments,
        }
    }
}

#[derive(uniffi::Record)]
pub struct InternalDownPaymentParams {
    pub params: InternalParams,      // The params for the actual payment plan
    pub requested_amount: f64,       // The requested amount for the down payment(ex: 1000.0)
    pub min_installment_amount: f64, // The minium installment value for the down payment (ex: 100.0)
    pub first_payment_date: SystemTime, // The first payment date for the down payment
    pub installments: u32,           // The max number of installments for the down payment (ex: 12)
}

impl From<InternalDownPaymentParams> for core_payment_plan::DownPaymentParams {
    fn from(val: InternalDownPaymentParams) -> Self {
        let first_payment_date: DateTime<Utc> = val.first_payment_date.into();
        let first_payment_date = first_payment_date.date_naive();

        core_payment_plan::DownPaymentParams {
            params: val.params.into(),
            requested_amount: val.requested_amount,
            min_installment_amount: val.min_installment_amount,
            first_payment_date,
            installments: val.installments,
        }
    }
}
