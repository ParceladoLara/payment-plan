use crate::{
    down_payment_plan,
    err::PaymentPlanError,
    plan,
    types::reimbursement,
    util::{add_days, add_months, round_decimal_cases},
};

mod inner_xirr;
pub mod providers;

pub trait PaymentPlan {
    fn calculate_payment_plan(
        &self,
        params: plan::Params,
    ) -> Result<Vec<plan::Response>, PaymentPlanError>;
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
        params: down_payment_plan::Params,
    ) -> Result<Vec<down_payment_plan::Response>, PaymentPlanError> {
        if params.requested_amount <= 0.0 {
            return Err(PaymentPlanError::InvalidRequestedAmount);
        }
        if params.installments == 0 {
            return Err(PaymentPlanError::InvalidNumberOfInstallments);
        }
        let mut resp = Vec::new();

        let mut base_params = params.params;
        let min_installment_amount = params.min_installment_amount;
        let down_payment_amount = params.requested_amount;
        let down_payment_first_payment_date = params.first_payment_date;

        // The start of the actual payment plan for 1 installment (we will update this in every iteration)
        let mut contract_start_date = add_days(down_payment_first_payment_date, 1);
        // The first payment date of the actual payment plan for 1 installment (we will update this in every iteration)
        let mut contract_first_payment_date = add_months(down_payment_first_payment_date, 1);

        for i in 1..=params.installments {
            base_params.first_payment_date = contract_first_payment_date;
            base_params.requested_date = contract_start_date;
            let installment_amount = down_payment_amount / i as f64;

            if installment_amount < min_installment_amount && i != 1 {
                break;
            }

            let plans = self.calculate_payment_plan(base_params)?;

            resp.push(down_payment_plan::Response {
                first_payment_date: down_payment_first_payment_date,
                installment_amount,
                installment_quantity: i,
                plans,
                total_amount: down_payment_amount,
            });

            // Update the start date and first payment date by a month for the next iteration
            contract_start_date = add_months(contract_start_date, 1);
            contract_first_payment_date = add_months(contract_first_payment_date, 1);
        }

        return Ok(resp);
    }

    fn calculate_reimbursement(
        &self,
        params: reimbursement::Params,
    ) -> Result<reimbursement::Response, PaymentPlanError> {
        let mut invoices_response = Vec::new();
        let mut customer_charge_back_amount = 0.0;
        let mut total_present_value_repurchase = 0.0;

        let reference_day_for_repurchase =
            add_days(params.base_date, params.max_repurchase_payment_days);

        let interest_rate_daily = (1.0 + params.interest_rate).powf(1.0 / 30.0) - 1.0;
        let interest_rate_daily = round_decimal_cases(interest_rate_daily, 7);

        let reimbursement_invoice_due_date =
            add_days(params.base_date, params.max_reimbursement_payment_days);

        let invoices = params.invoices;
        for invoice in invoices {
            let due_at = invoice.due_at;
            let diff = due_at
                .signed_duration_since(reference_day_for_repurchase)
                .num_days();
            let present_value_repurchase;

            match invoice.status {
                reimbursement::InvoiceStatus::PAID => {
                    customer_charge_back_amount += invoice.main_iof_tac;
                    present_value_repurchase = 0.0;
                }
                reimbursement::InvoiceStatus::CREATED
                | reimbursement::InvoiceStatus::READJUSTED => {
                    present_value_repurchase =
                        invoice.original_amount / (1.0 + interest_rate_daily).powf(diff as f64);
                }
                reimbursement::InvoiceStatus::OVERDUE => {
                    present_value_repurchase = invoice.original_amount;
                }
                reimbursement::InvoiceStatus::IRRELEVANT => {
                    present_value_repurchase = 0.0;
                }
            }

            total_present_value_repurchase += present_value_repurchase;
            invoices_response.push(reimbursement::InvoiceResponse {
                id: invoice.id,
                days_difference_between_repurchase_date_and_due_at: diff,
                present_value_repurchase,
            });
        }

        let subsidy_for_cancellation = (1.0 - params.fee) * params.mdr;

        let reimbursement_value =
            total_present_value_repurchase - subsidy_for_cancellation + params.invoice_cost;

        let customer_charge_back_amount = round_decimal_cases(customer_charge_back_amount, 2);

        return Ok(reimbursement::Response {
            total_present_value_repurchase,
            reimbursement_value,
            reference_date_for_repurchase: reference_day_for_repurchase,
            interest_rate_daily,
            subsidy_for_cancellation,
            invoices: invoices_response,
            customer_charge_back_amount,
            reimbursement_invoice_due_date,
        });
    }
}
