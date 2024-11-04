use std::fmt::Display;

use xirr::InvalidPaymentsError;

use crate::types::plan::Params;

#[derive(Debug)]
pub enum PaymentPlanError {
    CalculationError(InvalidPaymentsError),
    InvalidNumberOfInstallments,
    InvalidRequestedAmount,
    XirCalculationError(Params),
}

impl PartialEq for PaymentPlanError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PaymentPlanError::CalculationError(_), PaymentPlanError::CalculationError(_)) => true,
            (
                PaymentPlanError::InvalidNumberOfInstallments,
                PaymentPlanError::InvalidNumberOfInstallments,
            ) => true,
            (
                PaymentPlanError::InvalidRequestedAmount,
                PaymentPlanError::InvalidRequestedAmount,
            ) => true,
            _ => false,
        }
    }
}

impl From<InvalidPaymentsError> for PaymentPlanError {
    fn from(error: InvalidPaymentsError) -> Self {
        PaymentPlanError::CalculationError(error)
    }
}

impl Display for PaymentPlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentPlanError::CalculationError(error) => write!(f, "Calculation error: {}", error),
            PaymentPlanError::InvalidNumberOfInstallments => {
                write!(f, "Number of installments must be greater than 0")
            }
            PaymentPlanError::InvalidRequestedAmount => {
                write!(f, "Requested amount must be greater than 0")
            }
            PaymentPlanError::XirCalculationError(params) => {
                write!(f, "XIR calculation error: {}", params)
            }
        }
    }
}
