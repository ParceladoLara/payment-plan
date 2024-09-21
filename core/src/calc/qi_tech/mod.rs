use crate::{err::PaymentPlanError, DownPaymentParams, DownPaymentResponse, Params, Response};

use super::PaymentPlan;

pub struct QiTech;

impl PaymentPlan for QiTech {
    fn new() -> Self {
        QiTech
    }

    fn calculate_payment_plan(&self, params: Params) -> Result<Vec<Response>, PaymentPlanError> {
        println!("Params: {:?}", params);
        todo!()
    }

    fn calculate_down_payment_plan(
        &self,
        params: DownPaymentParams,
    ) -> Result<Vec<DownPaymentResponse>, PaymentPlanError> {
        println!("Params: {:?}", params);
        todo!()
    }
}
