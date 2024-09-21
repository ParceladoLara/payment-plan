use crate::{err::PaymentPlanError, DownPaymentParams, DownPaymentResponse, Params, Response};

pub mod bmp;
pub mod qi_tech;

pub trait PaymentPlan {
    fn new() -> Self;

    fn calculate_payment_plan(&self, params: Params) -> Result<Vec<Response>, PaymentPlanError>;
    /*
        A down payment plan is a payment plan that is made before the actual payment plan.
        It is much simpler than the actual payment plan in terms of calculations.
        It is just the division of the requested amount by the number of installments with some rules.
        The rules are:
            - The down payment plan must have at least 1 installment
            - When the number of installments is 1, the minimum installment amount is not considered
            - for the 2nd installment and beyond, the minimum installment amount is considered

        The calculation is done by simply dividing the requested amount by the number of installments
            until the number of installments is reached or the minimum installment amount is reached.

        Because the customer will start the actual payment after the last installment of the down payment plan,
        in every iteration, we must update the "requested_date" and "first_payment_date" of the actual payment plan.

    */
    fn calculate_down_payment_plan(
        &self,
        params: DownPaymentParams,
    ) -> Result<Vec<DownPaymentResponse>, PaymentPlanError>;
}
