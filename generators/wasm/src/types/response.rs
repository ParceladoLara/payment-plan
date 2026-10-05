use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use super::date::Date;

#[allow(non_snake_case)]
#[derive(Tsify, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub accumulated_days: i64,
    pub factor: f64,
    pub accumulated_factor: f64,
    #[serde(rename = "mainIOFTAC")]
    pub main_iof_tac: f64,
    pub debit_service: f64,
    pub due_date: Date,
}

impl From<core_payment_plan::Invoice> for Invoice {
    fn from(value: core_payment_plan::Invoice) -> Self {
        Self {
            accumulated_days: value.accumulated_days,
            factor: value.factor,
            accumulated_factor: value.accumulated_factor,
            main_iof_tac: value.main_iof_tac,
            debit_service: value.debit_service,
            due_date: value.due_date.into(),
        }
    }
}

impl From<Invoice> for js_sys::Object {
    fn from(val: Invoice) -> Self {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &"accumulatedDays".into(),
            &val.accumulated_days.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"factor".into(), &val.factor.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"accumulatedFactor".into(),
            &val.accumulated_factor.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"mainIOFTAC".into(), &val.main_iof_tac.into());
        let _ = js_sys::Reflect::set(&obj, &"debitService".into(), &val.debit_service.into());
        let _ = js_sys::Reflect::set(&obj, &"dueDate".into(), &val.due_date.into());
        obj
    }
}

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
    pub invoices: Vec<Invoice>,
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
            invoices: value.invoices.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<PaymentPlanResponse> for js_sys::Object {
    fn from(val: PaymentPlanResponse) -> Self {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&obj, &"installment".into(), &val.installment.into());
        let _ = js_sys::Reflect::set(&obj, &"dueDate".into(), &val.due_date.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"accumulatedDays".into(),
            &val.accumulated_days.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"daysIndex".into(), &val.days_index.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"accumulatedDaysIndex".into(),
            &val.accumulated_days_index.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"interestRate".into(), &val.interest_rate.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentAmount".into(),
            &val.installment_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentAmountWithoutTAC".into(),
            &val.installment_amount_without_tac.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"totalAmount".into(), &val.total_amount.into());
        let _ = js_sys::Reflect::set(&obj, &"debitService".into(), &val.debit_service.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"customerDebitServiceAmount".into(),
            &val.customer_debit_service_amount.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"customerAmount".into(), &val.customer_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"calculationBasisForEffectiveInterestRate".into(),
            &val.calculation_basis_for_effective_interest_rate.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"merchantDebitServiceAmount".into(),
            &val.merchant_debit_service_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"merchantTotalAmount".into(),
            &val.merchant_total_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"settledToMerchant".into(),
            &val.settled_to_merchant.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"mdrAmount".into(), &val.mdr_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"effectiveInterestRate".into(),
            &val.effective_interest_rate.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"totalEffectiveCost".into(),
            &val.total_effective_cost.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"eirYearly".into(), &val.eir_yearly.into());
        let _ = js_sys::Reflect::set(&obj, &"tecYearly".into(), &val.tec_yearly.into());
        let _ = js_sys::Reflect::set(&obj, &"eirMonthly".into(), &val.eir_monthly.into());
        let _ = js_sys::Reflect::set(&obj, &"tecMonthly".into(), &val.tec_monthly.into());
        let _ = js_sys::Reflect::set(&obj, &"totalIOF".into(), &val.total_iof.into());
        let _ = js_sys::Reflect::set(&obj, &"contractAmount".into(), &val.contract_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"contractAmountWithoutTAC".into(),
            &val.contract_amount_without_tac.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"tacAmount".into(), &val.tac_amount.into());
        let _ = js_sys::Reflect::set(&obj, &"IOFPercentage".into(), &val.iof_percentage.into());
        let _ = js_sys::Reflect::set(&obj, &"overallIOF".into(), &val.overall_iof.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"disbursementDate".into(),
            &val.disbursement_date.into(),
        );

        let _ = js_sys::Reflect::set(&obj, &"paidTotalIOF".into(), &val.paid_total_iof.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"paidContractAmount".into(),
            &val.paid_contract_amount.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"preDisbursementAmount".into(),
            &val.pre_disbursement_amount.into(),
        );
        let array = js_sys::Array::new_with_length(val.invoices.len() as u32);
        for (i, invoice) in val.invoices.into_iter().enumerate() {
            let js_invoice: js_sys::Object = invoice.into();
            let _ = js_sys::Reflect::set(&array, &i.into(), &js_invoice.into());
        }
        let _ = js_sys::Reflect::set(&obj, &"invoices".into(), &array.into());
        obj
    }
}

impl From<PaymentPlanResponse> for JsValue {
    fn from(val: PaymentPlanResponse) -> Self {
        let obj: js_sys::Object = val.into();
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

impl From<DownPaymentResponse> for js_sys::Object {
    fn from(val: DownPaymentResponse) -> Self {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentAmount".into(),
            &val.installment_amount.into(),
        );
        let _ = js_sys::Reflect::set(&obj, &"totalAmount".into(), &val.total_amount.into());
        let _ = js_sys::Reflect::set(
            &obj,
            &"installmentQuantity".into(),
            &val.installment_quantity.into(),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &"firstPaymentDate".into(),
            &val.first_payment_date.into(),
        );

        let array = js_sys::Array::new_with_length(val.plans.len() as u32);
        for (i, plan) in val.plans.into_iter().enumerate() {
            let js_plan: js_sys::Object = plan.into();
            let _ = js_sys::Reflect::set(&array, &i.into(), &js_plan.into());
        }
        let _ = js_sys::Reflect::set(&obj, &"plans".into(), &array.into());

        obj
    }
}

impl From<DownPaymentResponse> for JsValue {
    fn from(val: DownPaymentResponse) -> Self {
        let obj: js_sys::Object = val.into();
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
