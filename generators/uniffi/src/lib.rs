mod params;
mod response;

use std::{
    fmt::{Display, Formatter},
    time::SystemTime,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use params::{InternalDownPaymentParams, InternalParams};
use response::{InternalDownPaymentResponse, InternalResponse};

#[derive(uniffi::Error, Debug)]
pub enum Error {
    InvalidParams,
    CalculationError,
}

impl From<core_payment_plan::err::PaymentPlanError> for Error {
    fn from(value: core_payment_plan::err::PaymentPlanError) -> Self {
        match value {
            core_payment_plan::err::PaymentPlanError::CalculationError(_) => {
                Error::CalculationError
            }
            core_payment_plan::err::PaymentPlanError::XirCalculationError(_) => {
                Error::CalculationError
            }
            _ => Error::InvalidParams,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidParams => write!(f, "Invalid parameters provided"),
            Error::CalculationError => write!(f, "Error during calculation"),
        }
    }
}

impl std::error::Error for Error {}

#[uniffi::export]
pub fn calculate_down_payment_plan(
    params: InternalDownPaymentParams,
) -> Result<Vec<InternalDownPaymentResponse>, Error> {
    let params: core_payment_plan::DownPaymentParams = params.into();
    let result = core_payment_plan::calculate_down_payment_plan(params)?;
    let result: Vec<InternalDownPaymentResponse> = result.into_iter().map(|x| x.into()).collect();
    Ok(result)
}

#[uniffi::export]
pub fn calculate_payment_plan(params: InternalParams) -> Result<Vec<InternalResponse>, Error> {
    let params: core_payment_plan::Params = params.into();
    let result = core_payment_plan::calculate_payment_plan(params)?;
    let result: Vec<InternalResponse> = result.into_iter().map(|x| x.into()).collect();
    Ok(result)
}

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

uniffi::setup_scaffolding!();
