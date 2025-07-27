use crate::{
    params::{DownPaymentParams, Params},
    response::{DownPaymentResponse, Response},
};
use ::safer_ffi::prelude::*;

mod params;
mod response;

#[derive_ReprC]
#[repr(u8)]
pub enum PaymentPlanResult {
    Success = 0,
    InvalidParams = 1,
    CalculationError = 2,
}

#[ffi_export]
pub fn calculate_down_payment_plan(
    params: DownPaymentParams,
    out_responses: &mut repr_c::Vec<DownPaymentResponse>,
) -> PaymentPlanResult {
    let params: core_payment_plan::DownPaymentParams = params.into();
    let result = core_payment_plan::calculate_down_payment_plan(params).unwrap();
    let result: Vec<DownPaymentResponse> = result.into_iter().map(|x| x.into()).collect();
    *out_responses = result.into();
    PaymentPlanResult::Success
}

#[ffi_export]
pub fn calculate_payment_plan(
    params: Params,
    out_responses: &mut repr_c::Vec<Response>,
) -> PaymentPlanResult {
    let params: core_payment_plan::Params = params.into();
    let result = core_payment_plan::calculate_payment_plan(params).unwrap(); //remove unwrap
    let result: Vec<Response> = result.into_iter().map(|x| x.into()).collect();
    *out_responses = result.into();
    PaymentPlanResult::Success
}

/*
#[uniffi::export]
pub fn next_disbursement_date(base_date: SystemTime) -> SystemTime {
    let base_date: DateTime<Utc> = base_date.into();
    let base_date = base_date.date_naive();
    let result = core_payment_plan::next_disbursement_date(base_date);

    let date: NaiveDateTime = result.into();
    let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date, Utc);
    //add 10 hours to the date
    let date: DateTime<Utc> = date + chrono::Duration::hours(10);
    let date: SystemTime = date.into();
    date
}

#[uniffi::export]
pub fn disbursement_date_range(base_date: SystemTime, days: u32) -> Vec<SystemTime> {
    let base_date: DateTime<Utc> = base_date.into();
    let base_date = base_date.date_naive();
    let result = core_payment_plan::disbursement_date_range(base_date, days);

    let (start_date, end_date) = result;

    let start_date: NaiveDateTime = start_date.into();
    let start_date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(start_date, Utc);
    //add 10 hours to the date
    let start_date: DateTime<Utc> = start_date + chrono::Duration::hours(10);
    let start_date: SystemTime = start_date.into();

    let end_date: NaiveDateTime = end_date.into();
    let end_date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(end_date, Utc);
    //add 10 hours to the date
    let end_date: DateTime<Utc> = end_date + chrono::Duration::hours(10);
    let end_date: SystemTime = end_date.into();

    let mut result = Vec::with_capacity(2);
    result.push(start_date);
    result.push(end_date);
    result
}

#[uniffi::export]
pub fn get_non_business_days_between(
    start_date: SystemTime,
    end_date: SystemTime,
) -> Vec<SystemTime> {
    let start_date: DateTime<Utc> = start_date.into();
    let end_date: DateTime<Utc> = end_date.into();
    let start_date = start_date.date_naive();
    let end_date = end_date.date_naive();
    let result = core_payment_plan::get_non_business_days_between(start_date, end_date);

    let mut resp = Vec::with_capacity(result.len());
    for date in result {
        let date: NaiveDateTime = date.into();
        let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date, Utc);
        //add 10 hours to the date
        let date: DateTime<Utc> = date + chrono::Duration::hours(10);
        let date: SystemTime = date.into();
        resp.push(date);
    }
    resp
}
*/

#[ffi_export]
fn free_response_vec(value: repr_c::Vec<Response>) {
    drop(value);
}

#[ffi_export]
fn free_down_payment_response_vec(value: repr_c::Vec<DownPaymentResponse>) {
    drop(value);
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("payment_plan.h")?
        .generate()
}
