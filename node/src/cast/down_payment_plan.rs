use core_payment_plan::types::down_payment_plan;

use chrono::NaiveTime;
use neon::{
    context::{Context, FunctionContext},
    handle::Handle,
    object::Object,
    result::NeonResult,
    types::{JsArray, JsDate, JsNumber, JsObject, JsValue},
};

use crate::parser::any_to_number;

use super::plan;

pub fn cast_js_object_to_down_payment_param(
    cx: &mut FunctionContext,
    obj: Handle<JsObject>,
) -> NeonResult<down_payment_plan::Params> {
    let params: Handle<JsObject> = obj.get(cx, "params")?;
    let requested_amount: Handle<JsValue> = obj.get(cx, "requestedAmount")?;
    let min_installment_amount: Handle<JsValue> = obj.get(cx, "minInstallmentAmount")?;
    let first_payment_date_millis: Handle<JsDate> = obj.get(cx, "firstPaymentDate")?;
    let installments: Handle<JsValue> = obj.get(cx, "installments")?;

    let params = plan::cast_js_object_to_param(cx, params)?;
    let requested_amount = any_to_number(cx, requested_amount)?;
    let min_installment_amount = any_to_number(cx, min_installment_amount)?;
    let first_payment_date_millis = first_payment_date_millis.value(cx);
    let installments = any_to_number(cx, installments)? as u32;

    let first_payment_date =
        chrono::DateTime::from_timestamp_millis(first_payment_date_millis as i64);
    let first_payment_date = match first_payment_date {
        Some(date) => date.date_naive(),
        None => {
            return cx.throw_error("Invalid date");
        }
    };

    Ok(down_payment_plan::Params {
        params,
        requested_amount,
        min_installment_amount,
        first_payment_date,
        installments,
    })
}

fn cast_down_payment_response_to_js_object<'a, C: Context<'a>>(
    cx: &mut C,
    response: down_payment_plan::Response,
) -> NeonResult<Handle<'a, JsObject>> {
    let installment_amount = JsNumber::new(cx, response.installment_amount);
    let total_amount = JsNumber::new(cx, response.total_amount);
    let installment_quantity = JsNumber::new(cx, response.installment_quantity as f64);

    let first_payment_date = response
        .first_payment_date
        .and_time(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
        .and_utc();

    let first_payment_date = JsDate::new(cx, first_payment_date.timestamp_millis() as f64);
    let first_payment_date = match first_payment_date {
        Ok(date) => date,
        Err(_) => {
            return cx.throw_error("Invalid date");
        }
    };

    let plans = plan::cast_vec_response_to_js_array(cx, response.plans)?;

    let obj = JsObject::new(cx);
    obj.set(cx, "installmentAmount", installment_amount)?;
    obj.set(cx, "totalAmount", total_amount)?;
    obj.set(cx, "installmentQuantity", installment_quantity)?;
    obj.set(cx, "firstPaymentDate", first_payment_date)?;
    obj.set(cx, "plans", plans)?;

    Ok(obj)
}

pub fn cast_vec_down_payment_response_to_js_array<'a, C: Context<'a>>(
    cx: &mut C,
    responses: Vec<down_payment_plan::Response>,
) -> NeonResult<Handle<'a, JsArray>> {
    let array = JsArray::new(cx, responses.len() as usize);
    for (i, response) in responses.into_iter().enumerate() {
        let obj = cast_down_payment_response_to_js_object(cx, response)?;
        array.set(cx, i as u32, obj)?;
    }
    Ok(array)
}
