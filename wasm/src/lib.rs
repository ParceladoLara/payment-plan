use types::{
    param::{DownPaymentParams, Params},
    response::{DownPaymentResponse, Response},
};
use wasm_bindgen::prelude::*;

mod custom_serde;
mod types;

#[allow(non_snake_case)]
#[wasm_bindgen(js_name = "calculatePaymentPlan")]
pub fn calculate_payment_plan(p: Params) -> Result<Vec<Response>, JsError> {
    let core_params: core_payment_plan::Params = match p.try_into() {
        Ok(params) => params,
        Err(e) => return Err(e),
    };
    let result = match core_payment_plan::calc::calculate_payment_plan(core_params) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };
    let js_result: Vec<Response> = result.into_iter().map(|r| r.into()).collect();
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
    let result = match core_payment_plan::calc::calculate_down_payment_plan(core_params) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };
    let js_result: Vec<DownPaymentResponse> = result.into_iter().map(|r| r.into()).collect();
    Ok(js_result)
}
