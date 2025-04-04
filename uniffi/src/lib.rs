mod params;
mod response;

use std::fmt::{Display, Formatter};

use params::{DownPaymentParams, Params};
use response::{DownPaymentResponse, Response};

#[derive(uniffi::Error, Debug)]
pub enum Error {
    InvalidParams,
    CalculationError,
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
    params: DownPaymentParams,
) -> Result<Vec<DownPaymentResponse>, Error> {
    let params: core_payment_plan::DownPaymentParams = params.into();
    let result = core_payment_plan::calculate_down_payment_plan(params).unwrap();
    let result: Vec<DownPaymentResponse> = result.into_iter().map(|x| x.into()).collect();
    Ok(result)
}

#[uniffi::export]
pub fn calculate_payment_plan(params: Params) -> Result<Vec<Response>, Error> {
    let params: core_payment_plan::Params = params.into();
    let result = core_payment_plan::calculate_payment_plan(params).unwrap(); //remove unwrap
    let result: Vec<Response> = result.into_iter().map(|x| x.into()).collect();
    Ok(result)
}

uniffi::setup_scaffolding!();
