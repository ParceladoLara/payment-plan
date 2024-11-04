use calc::PaymentPlan;
use err::PaymentPlanError;

#[cfg(feature = "bmp")]
use calc::providers::bmp::BMP;
#[cfg(feature = "qitech")]
use calc::providers::qi_tech::QiTech;

// Default to BMP if no feature is specified
#[cfg(not(any(feature = "bmp", feature = "qitech")))]
use calc::providers::qi_tech::QiTech;

use types::{down_payment_plan, plan, reimbursement};

mod calc;
mod err;
mod util;

pub mod types;

#[cfg(feature = "bmp")]
const P: BMP = BMP {};
#[cfg(feature = "qitech")]
const P: QiTech = QiTech {};

// Default to BMP if no feature is specified
#[cfg(not(any(feature = "bmp", feature = "qitech")))]
const P: QiTech = QiTech {};

pub fn calculate_down_payment_plan(
    params: down_payment_plan::Params,
) -> Result<Vec<down_payment_plan::Response>, PaymentPlanError> {
    return P.calculate_down_payment_plan(params);
}

pub fn calculate_payment_plan(
    params: plan::Params,
) -> Result<Vec<plan::Response>, PaymentPlanError> {
    return P.calculate_payment_plan(params);
}

pub fn calculate_reimbursement(
    params: reimbursement::Params,
) -> Result<reimbursement::Response, PaymentPlanError> {
    return P.calculate_reimbursement(params);
}
