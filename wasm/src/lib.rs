use js_sys::Array;
use types::{
    date::Date,
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
    let result: Date = match core_payment_plan::next_disbursement_date(core_date) {
        Ok(r) => r.into(),
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

#[allow(non_snake_case)]
#[wasm_bindgen(
    js_name = "disbursementDateRange",
    unchecked_return_type = "Array<Date>"
)]
pub fn disbursement_date_range(base_date: js_sys::Date, days: u32) -> Result<Array, JsError> {
    let inner_date: types::date::Date = base_date.into();
    let core_date: chrono::NaiveDate = match inner_date.try_into() {
        Ok(date) => date,
        Err(e) => return Err(e),
    };

    let result = match core_payment_plan::disbursement_data_range(core_date, days) {
        Ok(r) => r,
        Err(e) => return Err(JsError::new(&e.to_string())),
    };

    let array = Array::new_with_length(2);

    let start_date: Date = result.0.into();
    let end_date: Date = result.1.into();

    array.set(0, start_date.into());
    array.set(1, end_date.into());

    return Ok(array);
}

#[allow(non_snake_case)]
#[wasm_bindgen(
    js_name = "getNonBusinessDaysBetween",
    unchecked_return_type = "Array<Date>"
)]
pub fn get_non_business_days_between(
    start_date: js_sys::Date,
    end_date: js_sys::Date,
) -> Result<Array, JsError> {
    let start_date: types::date::Date = start_date.into();
    let end_date: types::date::Date = end_date.into();

    let core_start_date: chrono::NaiveDate = match start_date.try_into() {
        Ok(date) => date,
        Err(e) => return Err(e),
    };

    let core_end_date: chrono::NaiveDate = match end_date.try_into() {
        Ok(date) => date,
        Err(e) => return Err(e),
    };

    let result = core_payment_plan::get_non_business_days_between(core_start_date, core_end_date);

    let array = Array::new_with_length(result.len() as u32);

    for (i, date) in result.into_iter().enumerate() {
        let js_date: types::date::Date = date.into();
        array.set(i as u32, js_date.into());
    }

    return Ok(array);
}
