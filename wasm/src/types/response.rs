use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use super::date::Date;

#[allow(non_snake_case)]
#[derive(Tsify, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPlanResponse {
    pub installment: u32,
    pub due_date: Date,
    pub accumulated_days: i32,
    pub days_index: f64,
    pub accumulated_days_index: f64,
    pub interest_rate: f64,
    pub installment_amount: f64,
    #[serde(rename = "installmentAmountWithoutTAC")]
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
    #[serde(rename = "IOFPercentage")]
    pub iof_percentage: f64,
    #[serde(rename = "overallIOF")]
    pub overall_iof: f64,
    pub disbursement_date: Date,
    #[serde(rename = "paidTotalIOF")]
    pub paid_total_iof: f64,
    #[serde(rename = "paidContractAmount")]
    pub paid_contract_amount: f64,
    #[serde(rename = "preDisbursementAmount")]
    pub pre_disbursement_amount: f64,
}

impl From<core_payment_plan::Response> for PaymentPlanResponse {
    fn from(value: core_payment_plan::Response) -> Self {
        Self {
            installment: value.installment,
            due_date: value.due_date.into(),
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
            disbursement_date: value.disbursement_date.into(),
            paid_total_iof: value.paid_total_iof,
            paid_contract_amount: value.paid_contract_amount,
            pre_disbursement_amount: value.pre_disbursement_amount,
        }
    }
}

impl Into<js_sys::Object> for PaymentPlanResponse {
    fn into(self) -> js_sys::Object {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&obj, &"installment".into(), &self.installment.into());
        let _ = js_sys::Reflect::set(&obj, &"dueDate".into(), &self.due_date.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"accumulatedDays".into(),
            &self.accumulated_days.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"daysIndex".into(), &self.days_index.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"accumulatedDaysIndex".into(),
            &self.accumulated_days_index.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"interestRate".into(), &self.interest_rate.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentAmount".into(),
            &self.installment_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentAmountWithoutTAC".into(),
            &self.installment_amount_without_tac.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"totalAmount".into(), &self.total_amount.into());
        let _ = js_sys::Reflect::set(&obj, &"debitService".into(), &self.debit_service.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"customerDebitServiceAmount".into(),
            &self.customer_debit_service_amount.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"customerAmount".into(), &self.customer_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"calculationBasisForEffectiveInterestRate".into(),
            &self.calculation_basis_for_effective_interest_rate.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"merchantDebitServiceAmount".into(),
            &self.merchant_debit_service_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"merchantTotalAmount".into(),
            &self.merchant_total_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"settledToMerchant".into(),
            &self.settled_to_merchant.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"mdrAmount".into(), &self.mdr_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"effectiveInterestRate".into(),
            &self.effective_interest_rate.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"totalEffectiveCost".into(),
            &self.total_effective_cost.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"eirYearly".into(), &self.eir_yearly.into());
        let _ = js_sys::Reflect::set(&obj, &"tecYearly".into(), &self.tec_yearly.into());
        let _ = js_sys::Reflect::set(&obj, &"eirMonthly".into(), &self.eir_monthly.into());
        let _ = js_sys::Reflect::set(&obj, &"tecMonthly".into(), &self.tec_monthly.into());
        let _ = js_sys::Reflect::set(&obj, &"totalIOF".into(), &self.total_iof.into());
        let _ = js_sys::Reflect::set(&obj, &"contractAmount".into(), &self.contract_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"contractAmountWithoutTAC".into(),
            &self.contract_amount_without_tac.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"tacAmount".into(), &self.tac_amount.into());
        let _ = js_sys::Reflect::set(&obj, &"IOFPercentage".into(), &self.iof_percentage.into());
        let _ = js_sys::Reflect::set(&obj, &"overallIOF".into(), &self.overall_iof.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"disbursementDate".into(),
            &self.disbursement_date.into(),
        );

        let _ = js_sys::Reflect::set(&obj, &"paidTotalIOF".into(), &self.paid_total_iof.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"paidContractAmount".into(),
            &self.paid_contract_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"preDisbursementAmount".into(),
            &self.pre_disbursement_amount.into(),
        );
        obj
    }
}

impl Into<JsValue> for PaymentPlanResponse {
    fn into(self) -> JsValue {
        let obj: js_sys::Object = self.into();
        obj.into()
    }
}
#[allow(non_snake_case)]
#[derive(Debug, Clone, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct DownPaymentResponse {
    pub installment_amount: f64,
    pub total_amount: f64,
    pub installment_quantity: u32,
    pub first_payment_date: Date,
    pub plans: Vec<PaymentPlanResponse>,
}

impl Into<js_sys::Object> for DownPaymentResponse {
    fn into(self) -> js_sys::Object {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentAmount".into(),
            &self.installment_amount.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"totalAmount".into(), &self.total_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentQuantity".into(),
            &self.installment_quantity.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"firstPaymentDate".into(),
            &self.first_payment_date.into(),
        );

        let array = js_sys::Array::new_with_length(self.plans.len() as u32);
        for (i, plan) in self.plans.into_iter().enumerate() {
            let js_plan: js_sys::Object = plan.into();
            let _ = js_sys::Reflect::set(&array, &i.into(), &js_plan.into());
        }
        let _ = js_sys::Reflect::set(&obj, &"plans".into(), &array.into());

        obj
    }
}

impl Into<JsValue> for DownPaymentResponse {
    fn into(self) -> JsValue {
        let obj: js_sys::Object = self.into();
        obj.into()
    }
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
