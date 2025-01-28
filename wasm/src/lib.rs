use types::{
    param::{DownPaymentParams, Params},
    response::{DownPaymentResponse, InnerResponse},
};
use wasm_bindgen::prelude::*;

mod debug;
mod types;

#[allow(non_snake_case)]
#[wasm_bindgen(js_name = "calculatePaymentPlan")]
pub fn calculate_payment_plan(
    #[wasm_bindgen(unchecked_param_type = "Params")] p: Params,
) -> Result<Vec<InnerResponse>, JsError> {
    let core_params: core_payment_plan::Params = match p.try_into() {
        Ok(params) => params,
        Err(e) => return Err(e),
    };

    let result = match core_payment_plan::calculate_payment_plan(core_params) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };
    let js_result: Vec<InnerResponse> = result.into_iter().map(|r| r.into()).collect();
    Ok(js_result)
}

#[allow(non_snake_case)]
#[wasm_bindgen(js_name = "calculateDownPaymentPlan")]
pub fn calculate_down_payment_plan(
    p: DownPaymentParams,
) -> Result<Vec<DownPaymentResponse>, JsError> {
    let core_params: core_payment_plan::DownPaymentParams = match p.try_into() {
        Ok(params) => params,
        Err(e) => return Err(e),
    };
    let result = match core_payment_plan::calculate_down_payment_plan(core_params) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };
    let js_result: Vec<DownPaymentResponse> = result.into_iter().map(|r| r.into()).collect();
    Ok(js_result)
}

#[allow(non_snake_case)]
#[wasm_bindgen(js_name = "nextDisbursementDate")]
pub fn next_disbursement_date(base_date: js_sys::Date) -> Result<js_sys::Date, JsError> {
    let inner_date: types::date::InnerDate = base_date.into();
    let core_date: chrono::NaiveDate = match inner_date.try_into() {
        Ok(date) => date,
        Err(e) => return Err(e),
    };
    let result = match core_payment_plan::next_disbursement_date(core_date) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };
    let js_result: types::date::InnerDate = result.into();
    Ok(js_result.into())
}
