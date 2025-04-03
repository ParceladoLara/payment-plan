use std::time::SystemTime;

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(uniffi::Record)]
pub struct Response {
    pub installment: u32,
    pub due_date: SystemTime,
    pub disbursement_date: SystemTime,
    pub accumulated_days: i64,
    pub days_index: f64,
    pub accumulated_days_index: f64,
    pub interest_rate: f64,
    pub installment_amount: f64,
    pub installment_amount_without_tac: f64,
    pub total_amount: f64,
    pub debit_service: f64,
    pub customer_debit_service_amount: f64,
    pub customer_amount: f64,
    pub calculation_basis_for_effective_interest_rate: f64,
    pub merchant_debit_service_amount: f64,
    pub merchant_total_amount: f64,
    pub settled_to_merchant: f64,
    pub mdr_amount: f64,
    pub effective_interest_rate: f64,
    pub total_effective_cost: f64,
    pub eir_yearly: f64,
    pub tec_yearly: f64,
    pub eir_monthly: f64,
    pub tec_monthly: f64,
    pub total_iof: f64,
    pub contract_amount: f64,
    pub contract_amount_without_tac: f64,
    pub tac_amount: f64,
    pub iof_percentage: f64,
    pub overall_iof: f64,
    pub pre_disbursement_amount: f64,
    pub paid_total_iof: f64,
    pub paid_contract_amount: f64,
}

impl From<core_payment_plan::Response> for Response {
    fn from(value: core_payment_plan::Response) -> Self {
        let disbursement_date: NaiveDateTime = value.disbursement_date.into();
        let due_date: NaiveDateTime = value.due_date.into();

        let disbursement_date: DateTime<Utc> =
            DateTime::from_naive_utc_and_offset(disbursement_date, Utc);
        let due_date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(due_date, Utc);

        let disbursement_date: SystemTime = disbursement_date.into();
        let due_date: SystemTime = due_date.into();

        Self {
            installment: value.installment,
            due_date,
            disbursement_date,
            accumulated_days: value.accumulated_days,
            days_index: value.days_index,
            accumulated_days_index: value.accumulated_days_index,
            interest_rate: value.interest_rate,
            installment_amount: value.installment_amount,
            installment_amount_without_tac: value.installment_amount_without_tac,
            total_amount: value.total_amount,
            debit_service: value.debit_service,
            customer_debit_service_amount: value.customer_debit_service_amount,
            customer_amount: value.customer_amount,
            calculation_basis_for_effective_interest_rate: value
                .calculation_basis_for_effective_interest_rate,
            merchant_debit_service_amount: value.merchant_debit_service_amount,
            merchant_total_amount: value.merchant_total_amount,
            settled_to_merchant: value.settled_to_merchant,
            mdr_amount: value.mdr_amount,
            effective_interest_rate: value.effective_interest_rate,
            total_effective_cost: value.total_effective_cost,
            eir_yearly: value.eir_yearly,
            tec_yearly: value.tec_yearly,
            eir_monthly: value.eir_monthly,
            tec_monthly: value.tec_monthly,
            total_iof: value.total_iof,
            contract_amount: value.contract_amount,
            contract_amount_without_tac: value.contract_amount_without_tac,
            tac_amount: value.tac_amount,
            iof_percentage: value.iof_percentage,
            overall_iof: value.overall_iof,
            pre_disbursement_amount: value.pre_disbursement_amount,
            paid_total_iof: value.paid_total_iof,
            paid_contract_amount: value.paid_contract_amount,
        }
    }
}

#[derive(uniffi::Record)]
pub struct DownPaymentResponse {
    pub installment_amount: f64, // The installment amount for the down payment
    pub total_amount: f64,       // The total amount for the down payment
    pub installment_quantity: u32, // The number of installments for the down payment
    pub first_payment_date: SystemTime, // The first payment date for the down payment
    pub plans: Vec<Response>,    // The payment plans available for the down payment
}

impl From<core_payment_plan::DownPaymentResponse> for DownPaymentResponse {
    fn from(value: core_payment_plan::DownPaymentResponse) -> Self {
        let first_payment_date: NaiveDateTime = value.first_payment_date.into();
        let first_payment_date: DateTime<Utc> =
            DateTime::from_naive_utc_and_offset(first_payment_date, Utc);
        let first_payment_date: SystemTime = first_payment_date.into();

        Self {
            installment_amount: value.installment_amount,
            total_amount: value.total_amount,
            installment_quantity: value.installment_quantity,
            first_payment_date,
            plans: value.plans.into_iter().map(Into::into).collect(),
        }
    }
}
