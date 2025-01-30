use cast::{
    cast_js_object_to_down_payment_param, cast_js_object_to_param,
    cast_vec_down_payment_response_to_js_array, cast_vec_response_to_js_array,
};

use neon::{prelude::*, types::JsDate};

mod cast;
mod parser;

fn calculate_plan(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_obj: Handle<JsObject> = cx.argument(0)?;
    let params = cast_js_object_to_param(&mut cx, js_obj)?;
    let result = core_payment_plan::calculate_payment_plan(params);
    let result = match result {
        Ok(plan) => plan,
        Err(e) => {
            return cx.throw_error(e.to_string());
        }
    };
    let result = cast_vec_response_to_js_array(&mut cx, result)?;
    Ok(result)
}

fn calculate_down_payment_plan(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_obj: Handle<JsObject> = cx.argument(0)?;
    let params = cast_js_object_to_down_payment_param(&mut cx, js_obj)?;
    let result = core_payment_plan::calculate_down_payment_plan(params);
    let result = match result {
        Ok(plan) => plan,
        Err(e) => {
            return cx.throw_error(e.to_string());
        }
    };
    let result = cast_vec_down_payment_response_to_js_array(&mut cx, result)?;
    Ok(result)
}

fn next_disbursement_date(mut cx: FunctionContext) -> JsResult<JsDate> {
    let js_date: Handle<JsDate> = cx.argument(0)?;
    let date = parser::js_date_to_naive(&mut cx, js_date)?;
    let result = core_payment_plan::next_disbursement_date(date);
    let result = match result {
        Ok(date) => date,
        Err(e) => {
            return cx.throw_error(e.to_string());
        }
    };
    let result = parser::naive_to_js_date(&mut cx, result)?;
    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("calculatePlan", calculate_plan)?;
    cx.export_function("calculateDownPaymentPlan", calculate_down_payment_plan)?;
    cx.export_function("nextDisbursementDate", next_disbursement_date)?;
    Ok(())
}
