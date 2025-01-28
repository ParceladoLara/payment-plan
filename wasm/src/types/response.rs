use js_sys::Date;
use wasm_bindgen::prelude::*;

use super::date::InnerDate;

#[allow(non_snake_case)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct InnerResponse {
    pub installment: u32,
    #[wasm_bindgen(getter_with_clone, js_name = "dueDate")]
    pub due_date: Date,
    #[wasm_bindgen(js_name = "accumulatedDays")]
    pub accumulated_days: i32,
    #[wasm_bindgen(js_name = "daysIndex")]
    pub days_index: f64,
    #[wasm_bindgen(js_name = "accumulatedDaysIndex")]
    pub accumulated_days_index: f64,
    #[wasm_bindgen(js_name = "interestRate")]
    pub interest_rate: f64,
    #[wasm_bindgen(js_name = "installmentAmount")]
    pub installment_amount: f64,
    #[wasm_bindgen(js_name = "installmentAmountWithoutTAC")]
    pub installment_amount_without_tac: f64,
    #[wasm_bindgen(js_name = "totalAmount")]
    pub total_amount: f64,
    #[wasm_bindgen(js_name = "debitService")]
    pub debit_service: f64,
    #[wasm_bindgen(js_name = "customerDebitServiceAmount")]
    pub customer_debit_service_amount: f64,
    #[wasm_bindgen(js_name = "customerAmount")]
    pub customer_amount: f64,
    #[wasm_bindgen(js_name = "calculationBasisForEffectiveInterestRate")]
    pub calculation_basis_for_effective_interest_rate: f64,
    #[wasm_bindgen(js_name = "merchantDebitServiceAmount")]
    pub merchant_debit_service_amount: f64,
    #[wasm_bindgen(js_name = "merchantTotalAmount")]
    pub merchant_total_amount: f64,
    #[wasm_bindgen(js_name = "settledToMerchant")]
    pub settled_to_merchant: f64,
    #[wasm_bindgen(js_name = "mdrAmount")]
    pub mdr_amount: f64,
    #[wasm_bindgen(js_name = "effectiveInterestRate")]
    pub effective_interest_rate: f64,
    #[wasm_bindgen(js_name = "totalEffectiveCost")]
    pub total_effective_cost: f64,
    #[wasm_bindgen(js_name = "eirYearly")]
    pub eir_yearly: f64,
    #[wasm_bindgen(js_name = "tecYearly")]
    pub tec_yearly: f64,
    #[wasm_bindgen(js_name = "eirMonthly")]
    pub eir_monthly: f64,
    #[wasm_bindgen(js_name = "tecMonthly")]
    pub tec_monthly: f64,
    #[wasm_bindgen(js_name = "totalIOF")]
    pub total_iof: f64,
    #[wasm_bindgen(js_name = "contractAmount")]
    pub contract_amount: f64,
    #[wasm_bindgen(js_name = "contractAmountWithoutTAC")]
    pub contract_amount_without_tac: f64,
    #[wasm_bindgen(js_name = "tacAmount")]
    pub tac_amount: f64,
    #[wasm_bindgen(js_name = "IOFPercentage")]
    pub iof_percentage: f64,
    #[wasm_bindgen(js_name = "overallIOF")]
    pub overall_iof: f64,
    #[wasm_bindgen(getter_with_clone, js_name = "disbursementDate")]
    pub disbursement_date: Date,
}

impl From<core_payment_plan::Response> for InnerResponse {
    fn from(value: core_payment_plan::Response) -> Self {
        let due_date: InnerDate = value.due_date.into();
        let disbursement_date: InnerDate = value.disbursement_date.into();
        Self {
            installment: value.installment,
            due_date: due_date.into(),
            accumulated_days: value.accumulated_days as i32,
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
            disbursement_date: disbursement_date.into(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct DownPaymentResponse {
    #[wasm_bindgen(js_name = "installmentAmount")]
    pub installment_amount: f64,
    #[wasm_bindgen(js_name = "totalAmount")]
    pub total_amount: f64,
    #[wasm_bindgen(js_name = "installmentQuantity")]
    pub installment_quantity: u32,
    #[wasm_bindgen(getter_with_clone, js_name = "firstPaymentDate")]
    pub first_payment_date: Date,
    #[wasm_bindgen(getter_with_clone)]
    pub plans: Vec<InnerResponse>,
}

impl From<core_payment_plan::DownPaymentResponse> for DownPaymentResponse {
    fn from(value: core_payment_plan::DownPaymentResponse) -> Self {
        let first_payment_date: InnerDate = value.first_payment_date.into();
        Self {
            installment_amount: value.installment_amount,
            total_amount: value.total_amount,
            installment_quantity: value.installment_quantity,
            first_payment_date: first_payment_date.into(),
            plans: value.plans.into_iter().map(|r| r.into()).collect(),
        }
    }
}
