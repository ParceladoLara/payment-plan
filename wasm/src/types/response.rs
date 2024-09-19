use serde::Serialize;
use tsify_next::Tsify;

use super::date::Date;

#[allow(non_snake_case)]
#[derive(Tsify, Debug, Serialize, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub installment: u32,
    pub due_date: Date,
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
    #[serde(rename = "totalIOF")]
    pub total_iof: f64,
    pub contract_amount: f64,
    #[serde(rename = "contractAmountWithoutTAC")]
    pub contract_amount_without_tac: f64,
    pub tac_amount: f64,
    pub iof_percentage: f64,
    pub overall_iof: f64,
}

impl From<core_payment_plan::Response> for Response {
    fn from(value: core_payment_plan::Response) -> Self {
        Self {
            installment: value.installment,
            due_date: value.due_date.into(),
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
        }
    }
}

#[allow(non_snake_case)]
#[derive(Tsify, Debug, Serialize, Clone)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct DownPaymentResponse {
    pub installment_amount: f64,
    pub total_amount: f64,
    pub installment_quantity: u32,
    pub first_payment_date: Date,
    pub plans: Vec<Response>,
}

impl From<core_payment_plan::DownPaymentResponse> for DownPaymentResponse {
    fn from(value: core_payment_plan::DownPaymentResponse) -> Self {
        Self {
            installment_amount: value.installment_amount,
            total_amount: value.total_amount,
            installment_quantity: value.installment_quantity,
            first_payment_date: value.first_payment_date.into(),
            plans: value.plans.into_iter().map(|r| r.into()).collect(),
        }
    }
}
