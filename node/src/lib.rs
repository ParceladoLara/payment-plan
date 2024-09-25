use cast::{
    cast_js_object_to_down_payment_param, cast_js_object_to_param,
    cast_vec_down_payment_response_to_js_array, cast_vec_response_to_js_array,
};
use neon::prelude::*;

mod cast;
mod parser;

fn calculate_plan(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_obj: Handle<JsObject> = cx.argument(0)?;
    let params = cast_js_object_to_param(&mut cx, js_obj)?;
    let result = core_payment_plan::calc::calculate_payment_plan(params);
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
    let result = core_payment_plan::calc::calculate_down_payment_plan(params);
    let result = match result {
        Ok(plan) => plan,
        Err(e) => {
            return cx.throw_error(e.to_string());
        }
    };
    let result = cast_vec_down_payment_response_to_js_array(&mut cx, result)?;
    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("calculatePlan", calculate_plan)?;
    cx.export_function("calculateDownPaymentPlan", calculate_down_payment_plan)?;
    Ok(())
}
