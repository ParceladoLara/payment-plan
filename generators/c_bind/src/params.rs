use ::safer_ffi::prelude::*;

use chrono::{DateTime, Utc};

#[derive_ReprC]
#[repr(C)]
pub struct Params {
    pub requested_amount: f64,
    pub first_payment_date_ms: i64,
    pub disbursement_date_ms: i64,
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
    /*
    I will not be using Option<u32> because is not part of the "safer-ffi" crate.
    The create was a TaggedOption<T> with is the version of Option<T> for "safer-ffi", this works fine on the Rust side,
    but on the C side it generates the following struct:
        typedef struct Tuple2_bool_uint32
        {
            bool _0;
            uint32_t _1;
        } Tuple2_bool_uint32_t;
    Where bool is if the value is Some or None and uint32_t is the value itself, this makes sense for pointers of complex types, but for simple types like u32 it becomes a bit cumbersome to use.
    So instead of using Option<u32> I will use a u32 where 0 means None and any other value means Some(value), this way on the C side it will be just a uint32_t which is much easier to use.
    And of course anyone using the ABI can make the parameter optional on their side
     */
    pub min_installments: u32,
}

impl Into<core_payment_plan::Params> for Params {
    fn into(self) -> core_payment_plan::Params {
        let disbursement_date: DateTime<Utc> =
            chrono::DateTime::from_timestamp_millis(self.disbursement_date_ms)
                .expect("Invalid disbursement date timestamp");
        let first_payment_date: DateTime<Utc> =
            chrono::DateTime::from_timestamp_millis(self.first_payment_date_ms)
                .expect("Invalid first payment date timestamp");

        let disbursement_date = disbursement_date.date_naive();
        let first_payment_date = first_payment_date.date_naive();

        let min_installments = if self.min_installments == 0 {
            None
        } else {
            Some(self.min_installments)
        };

        core_payment_plan::Params {
            requested_amount: self.requested_amount,
            first_payment_date,
            disbursement_date: disbursement_date,
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
            min_installments,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
pub struct DownPaymentParams {
    pub params: Params,              // The params for the actual payment plan
    pub requested_amount: f64,       // The requested amount for the down payment(ex: 1000.0)
    pub min_installment_amount: f64, // The minium installment value for the down payment (ex: 100.0)
    pub first_payment_date_ms: i64,  // The first payment date for the down payment
    pub installments: u32,           // The max number of installments for the down payment (ex: 12)
}

impl Into<core_payment_plan::DownPaymentParams> for DownPaymentParams {
    fn into(self) -> core_payment_plan::DownPaymentParams {
        let first_payment_date: DateTime<Utc> =
            chrono::DateTime::from_timestamp_millis(self.first_payment_date_ms)
                .expect("Invalid first payment date timestamp");
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
