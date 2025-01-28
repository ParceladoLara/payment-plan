use js_sys::Array;
use types::{
    param::{DownPaymentParams, Params},
    response::{DownPaymentResponse, PaymentPlanResponse},
};
use wasm_bindgen::prelude::*;

mod debug;
mod types;

#[allow(non_snake_case)]
#[wasm_bindgen(
    js_name = "calculateDownPaymentPlan",
    unchecked_return_type = "Array<DownPaymentResponse>"
)]
pub fn calculate_down_payment_plan(p: DownPaymentParams) -> Result<Array, JsError> {
    let core_params: core_payment_plan::DownPaymentParams = match p.try_into() {
        Ok(params) => params,
        Err(e) => return Err(e),
    };
    let results = match core_payment_plan::calculate_down_payment_plan(core_params) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };

    let array = Array::new_with_length(results.len() as u32);

    for (i, result) in results.into_iter().enumerate() {
        let inner_result: DownPaymentResponse = result.into();
        array.set(i as u32, inner_result.into());
    }

    return Ok(array);
}

#[allow(non_snake_case)]
#[wasm_bindgen(js_name = "nextDisbursementDate")]
pub fn next_disbursement_date(base_date: js_sys::Date) -> Result<js_sys::Date, JsError> {
    let inner_date: types::date::Date = base_date.into();
    let core_date: chrono::NaiveDate = match inner_date.try_into() {
        Ok(date) => date,
        Err(e) => return Err(e),
    };
    let result = match core_payment_plan::next_disbursement_date(core_date) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };
    let js_result: types::date::Date = result.into();
    Ok(js_result.into())
}

#[allow(non_snake_case)]
#[wasm_bindgen(
    js_name = "calculatePaymentPlan",
    unchecked_return_type = "Array<PaymentPlanResponse>"
)]
pub fn calculate_payment_plan(p: Params) -> Result<Array, JsError> {
    let core_params: core_payment_plan::Params = match p.try_into() {
        Ok(params) => params,
        Err(e) => return Err(e),
    };

    let results = match core_payment_plan::calculate_payment_plan(core_params) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };

    let array = Array::new_with_length(results.len() as u32);

    for (i, result) in results.into_iter().enumerate() {
        let inner_result: PaymentPlanResponse = result.into();
        array.set(i as u32, inner_result.into());
    }

    return Ok(array);
}
