use serde::Deserialize;
use tsify_next::Tsify;
use wasm_bindgen::JsError;

use super::date::Date;

#[allow(non_snake_case)]
#[derive(Tsify, Debug, Deserialize, Clone)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub requested_amount: f64,
    pub first_payment_date: Date,
    pub requested_date: Date,
    pub installments: u32,
    pub debit_service_percentage: u16,
    pub mdr: f64,
    pub tac_percentage: f64,
    pub iof_overall: f64,
    pub iof_percentage: f64,
    pub interest_rate: f64,
    pub min_installment_amount: f64,
    pub max_total_amount: f64,
    pub disbursement_only_on_business_days: bool,
}

impl TryInto<core_payment_plan::Params> for Params {
    type Error = JsError;

    fn try_into(self) -> Result<core_payment_plan::Params, Self::Error> {
        Ok(core_payment_plan::Params {
            requested_amount: self.requested_amount,
            first_payment_date: self.first_payment_date.try_into()?,
            requested_date: self.requested_date.try_into()?,
            installments: self.installments,
            debit_service_percentage: self.debit_service_percentage,
            mdr: self.mdr,
            tac_percentage: self.tac_percentage,
            iof_overall: self.iof_overall,
            iof_percentage: self.iof_percentage,
            interest_rate: self.interest_rate,
            min_installment_amount: self.min_installment_amount,
            max_total_amount: self.max_total_amount,
            disbursement_only_on_business_days: self.disbursement_only_on_business_days,
        })
    }
}

#[allow(non_snake_case)]
#[derive(Tsify, Debug, Deserialize, Clone)]
#[tsify(from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct DownPaymentParams {
    pub params: Params,              // The params for the actual payment plan
    pub requested_amount: f64,       // The requested amount for the down payment(ex: 1000.0)
    pub min_installment_amount: f64, // The minium installment value for the down payment (ex: 100.0)
    pub first_payment_date: Date,    // The first payment date for the down payment
    pub installments: u32,           // The max number of installments for the down payment (ex: 12)
}

impl TryInto<core_payment_plan::DownPaymentParams> for DownPaymentParams {
    type Error = JsError;

    fn try_into(self) -> Result<core_payment_plan::DownPaymentParams, Self::Error> {
        Ok(core_payment_plan::DownPaymentParams {
            params: self.params.try_into()?,
            requested_amount: self.requested_amount,
            min_installment_amount: self.min_installment_amount,
            first_payment_date: self.first_payment_date.try_into()?,
            installments: self.installments,
        })
    }
}
