use crate::{
    params::{DownPaymentParams, Params},
    response::{DownPaymentResponse, Response},
};
use ::safer_ffi::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};

mod params;
mod response;

#[derive_ReprC]
#[repr(u8)]
pub enum PaymentPlanResult {
    Success = 0,
    InvalidParams = 1,
    CalculationError = 2,
}

impl From<core_payment_plan::err::PaymentPlanError> for PaymentPlanResult {
    fn from(value: core_payment_plan::err::PaymentPlanError) -> Self {
        match value {
            core_payment_plan::err::PaymentPlanError::CalculationError(_) => {
                PaymentPlanResult::CalculationError
            }
            core_payment_plan::err::PaymentPlanError::XirCalculationError(_) => {
                PaymentPlanResult::CalculationError
            }
            _ => PaymentPlanResult::InvalidParams,
        }
    }
}

/// Calculate the down payment plan.
/// the pointer on `Vec_DownPaymentResponse_t` must be null because the function will allocate the vector
///
/// # Safety: The caller must free the vector using `free_down_payment_response_vec`.
#[ffi_export]
pub fn calculate_down_payment_plan(
    params: DownPaymentParams,
    out_responses: &mut repr_c::Vec<DownPaymentResponse>,
) -> PaymentPlanResult {
    let params: core_payment_plan::DownPaymentParams = params.into();
    let result = match core_payment_plan::calculate_down_payment_plan(params) {
        Ok(res) => res,
        Err(err) => return err.into(),
    };
    let result: Vec<DownPaymentResponse> = result.into_iter().map(|x| x.into()).collect();
    *out_responses = result.into();
    PaymentPlanResult::Success
}

/// Calculate the payment plan.
/// the pointer on `Vec_Response_t` must be null because the function will allocate the vector
///
/// # Safety: The caller must free the vector using `free_response_vec`.
#[ffi_export]
pub fn calculate_payment_plan(
    params: Params,
    out_responses: &mut repr_c::Vec<Response>,
) -> PaymentPlanResult {
    let params: core_payment_plan::Params = params.into();
    let result = match core_payment_plan::calculate_payment_plan(params) {
        Ok(res) => res,
        Err(err) => return err.into(),
    };
    let result: Vec<Response> = result.into_iter().map(|x| x.into()).collect();
    *out_responses = result.into();
    PaymentPlanResult::Success
}

// Calculate the next disbursement date.
#[ffi_export]
pub fn next_disbursement_date(base_date: i64, result: &mut i64) -> PaymentPlanResult {
    let base_date: DateTime<Utc> = match chrono::DateTime::from_timestamp_millis(base_date) {
        Some(date) => date,
        None => return PaymentPlanResult::InvalidParams,
    };
    let base_date = base_date.date_naive();
    let call_result = core_payment_plan::next_disbursement_date(base_date);

    let date: NaiveDateTime = call_result.into();
    let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date, Utc);
    //add 10 hours to the date
    let date: DateTime<Utc> = date + chrono::Duration::hours(10);

    *result = date.timestamp_millis();
    PaymentPlanResult::Success
}

/// Calculate the disbursement date range.
#[ffi_export]
pub fn disbursement_date_range(
    base_date: i64,
    days: u32,
    result: &mut [i64; 2],
) -> PaymentPlanResult {
    let base_date: DateTime<Utc> = match chrono::DateTime::from_timestamp_millis(base_date) {
        Some(date) => date,
        None => return PaymentPlanResult::InvalidParams,
    };
    let base_date = base_date.date_naive();
    let call_result = core_payment_plan::disbursement_date_range(base_date, days);

    let (start_date, end_date) = call_result;

    let start_date: NaiveDateTime = start_date.into();
    let start_date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(start_date, Utc);
    //add 10 hours to the date
    let start_date: DateTime<Utc> = start_date + chrono::Duration::hours(10);

    let end_date: NaiveDateTime = end_date.into();
    let end_date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(end_date, Utc);
    //add 10 hours to the date
    let end_date: DateTime<Utc> = end_date + chrono::Duration::hours(10);

    *result = [start_date.timestamp_millis(), end_date.timestamp_millis()];
    PaymentPlanResult::Success
}

/// Get non-business days between two dates.
/// the pointer on `Vec<i64>` must be null because the function will allocate the vector
///
/// # Safety: The caller must free the vector using `free_i64_vec`.
#[ffi_export]
pub fn get_non_business_days_between(
    start_date: i64,
    end_date: i64,
    result: &mut repr_c::Vec<i64>,
) -> PaymentPlanResult {
    let start_date: DateTime<Utc> = match chrono::DateTime::from_timestamp_millis(start_date) {
        Some(date) => date,
        None => return PaymentPlanResult::InvalidParams,
    };
    let end_date: DateTime<Utc> = match chrono::DateTime::from_timestamp_millis(end_date) {
        Some(date) => date,
        None => return PaymentPlanResult::InvalidParams,
    };
    let start_date = start_date.date_naive();
    let end_date = end_date.date_naive();
    let call_result = core_payment_plan::get_non_business_days_between(start_date, end_date);

    let mut resp = Vec::with_capacity(result.len());
    for date in call_result {
        let date: NaiveDateTime = date.into();
        let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date, Utc);
        //add 10 hours to the date
        let date: DateTime<Utc> = date + chrono::Duration::hours(10);
        resp.push(date.timestamp_millis());
    }
    *result = resp.into();
    PaymentPlanResult::Success
}

/// Free the response vector allocated by the FFI functions.
#[ffi_export]
fn free_response_vec(value: repr_c::Vec<Response>) {
    drop(value);
}

/// Free the down payment response vector allocated by the FFI functions.
#[ffi_export]
fn free_down_payment_response_vec(value: repr_c::Vec<DownPaymentResponse>) {
    drop(value);
}

/// Free the i64 vector allocated by the FFI functions.
#[ffi_export]
fn free_i64_vec(value: repr_c::Vec<i64>) {
    drop(value);
}
#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("payment_plan.h")?
        .generate()
}
