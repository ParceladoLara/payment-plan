use chrono::NaiveTime;
use core_payment_plan::types::reimbursement::{
    InvoiceParam, InvoiceResponse, InvoiceStatus, Params, Response,
};
use neon::{
    handle::Handle,
    object::Object,
    prelude::{Context, FunctionContext},
    result::NeonResult,
    types::{JsArray, JsDate, JsNumber, JsObject, JsString, JsValue},
};

use crate::parser::any_to_number;

pub fn cast_js_object_to_param(
    cx: &mut FunctionContext,
    obj: Handle<JsObject>,
) -> NeonResult<Params> {
    let fee: Handle<JsValue> = obj.get(cx, "fee")?;
    let fee = any_to_number(cx, fee)?;

    let mdr: Handle<JsValue> = obj.get(cx, "mdr")?;
    let mdr = any_to_number(cx, mdr)?;

    let invoice_cost: Handle<JsValue> = obj.get(cx, "invoiceCost")?;
    let invoice_cost = any_to_number(cx, invoice_cost)?;

    let interest_rate: Handle<JsValue> = obj.get(cx, "interestRate")?;
    let interest_rate = any_to_number(cx, interest_rate)?;

    let base_date: Handle<JsDate> = obj.get(cx, "baseDate")?;
    let base_date = base_date.value(cx);
    let base_date = chrono::DateTime::from_timestamp_millis(base_date as i64);
    let base_date = match base_date {
        Some(date) => date.date_naive(),
        None => {
            return cx.throw_error("Invalid date");
        }
    };

    let max_repurchase_payment_days: Handle<JsValue> = obj.get(cx, "maxRepurchasePaymentDays")?;
    let max_repurchase_payment_days = any_to_number(cx, max_repurchase_payment_days)? as i64;

    let max_reimbursement_payment_days: Handle<JsValue> =
        obj.get(cx, "maxReimbursementPaymentDays")?;
    let max_reimbursement_payment_days = any_to_number(cx, max_reimbursement_payment_days)? as i64;

    let invoices: Handle<JsArray> = obj.get(cx, "invoices")?;
    let invoices = cast_js_array_to_invoice_param(cx, invoices)?;

    Ok(Params {
        fee,
        mdr,
        invoice_cost,
        interest_rate,
        base_date,
        max_repurchase_payment_days,
        max_reimbursement_payment_days,
        invoices,
    })
}

pub fn cast_response_to_js_object<'a, C: Context<'a>>(
    cx: &mut C,
    response: Response,
) -> NeonResult<Handle<'a, JsObject>> {
    let obj = JsObject::new(cx);

    let total_present_value_repurchase = JsNumber::new(cx, response.total_present_value_repurchase);
    let reimbursement_value = JsNumber::new(cx, response.reimbursement_value);

    let reference_date_for_repurchase = response
        .reference_date_for_repurchase
        .and_time(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
        .and_utc();

    let reference_date_for_repurchase =
        JsDate::new(cx, reference_date_for_repurchase.timestamp_millis() as f64);

    let reference_date_for_repurchase = match reference_date_for_repurchase {
        Ok(date) => date,
        Err(_) => {
            return cx.throw_error("Invalid date");
        }
    };

    let interest_rate_daily = JsNumber::new(cx, response.interest_rate_daily);
    let subsidy_for_cancellation = JsNumber::new(cx, response.subsidy_for_cancellation);
    let customer_charge_back_amount = JsNumber::new(cx, response.customer_charge_back_amount);

    let reimbursement_invoice_due_date = response
        .reimbursement_invoice_due_date
        .and_time(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
        .and_utc();
    let reimbursement_invoice_due_date =
        JsDate::new(cx, reimbursement_invoice_due_date.timestamp_millis() as f64);
    let reimbursement_invoice_due_date = match reimbursement_invoice_due_date {
        Ok(date) => date,
        Err(_) => {
            return cx.throw_error("Invalid date");
        }
    };

    let invoices = cast_vec_invoice_response_to_js_array(cx, response.invoices)?;

    obj.set(
        cx,
        "totalPresentValueRepurchase",
        total_present_value_repurchase,
    )?;
    obj.set(cx, "reimbursementValue", reimbursement_value)?;
    obj.set(
        cx,
        "referenceDateForRepurchase",
        reference_date_for_repurchase,
    )?;
    obj.set(cx, "interestRateDaily", interest_rate_daily)?;
    obj.set(cx, "subsidyForCancellation", subsidy_for_cancellation)?;
    obj.set(cx, "customerChargeBackAmount", customer_charge_back_amount)?;
    obj.set(
        cx,
        "reimbursementInvoiceDueDate",
        reimbursement_invoice_due_date,
    )?;
    obj.set(cx, "invoices", invoices)?;

    Ok(obj)
}

fn cast_vec_invoice_response_to_js_array<'a, C: Context<'a>>(
    cx: &mut C,
    invoices: Vec<InvoiceResponse>,
) -> NeonResult<Handle<'a, JsArray>> {
    let array = JsArray::new(cx, invoices.len() as usize);
    for (i, response) in invoices.into_iter().enumerate() {
        let obj = JsObject::new(cx);
        let id = JsNumber::new(cx, response.id);
        let days_difference_between_repurchase_date_and_due_at = JsNumber::new(
            cx,
            response.days_difference_between_repurchase_date_and_due_at as f64,
        );
        let present_value_repurchase = JsNumber::new(cx, response.present_value_repurchase);

        obj.set(cx, "id", id)?;
        obj.set(
            cx,
            "daysDifferenceBetweenRepurchaseDateAndDueAt",
            days_difference_between_repurchase_date_and_due_at,
        )?;
        obj.set(cx, "presentValueRepurchase", present_value_repurchase)?;

        array.set(cx, i as u32, obj)?;
    }
    Ok(array)
}

fn cast_js_string_to_invoice_status(
    cx: &mut FunctionContext,
    string: Handle<JsString>,
) -> NeonResult<InvoiceStatus> {
    let string = string.value(cx);
    let status = InvoiceStatus::from(string);
    Ok(status)
}

fn cast_js_object_to_invoice_param(
    cx: &mut FunctionContext,
    obj: Handle<JsObject>,
) -> NeonResult<InvoiceParam> {
    let status = obj.get(cx, "status")?;
    let status = cast_js_string_to_invoice_status(cx, status)?;

    let id = obj.get(cx, "id")?;
    let id = any_to_number(cx, id)? as u32;

    let original_amount = obj.get(cx, "originalAmount")?;
    let original_amount = any_to_number(cx, original_amount)?;

    let due_at: Handle<JsDate> = obj.get(cx, "dueAt")?;
    let due_at = due_at.value(cx);
    let due_at = chrono::DateTime::from_timestamp_millis(due_at as i64);
    let due_at = match due_at {
        Some(date) => date.date_naive(),
        None => {
            return cx.throw_error("Invalid date");
        }
    };

    let main_iof_tac = obj.get(cx, "mainIofTac")?;
    let main_iof_tac = any_to_number(cx, main_iof_tac)?;

    Ok(InvoiceParam {
        id,
        status,
        original_amount,
        due_at,
        main_iof_tac,
    })
}

fn cast_js_array_to_invoice_param(
    cx: &mut FunctionContext,
    array: Handle<JsArray>,
) -> NeonResult<Vec<InvoiceParam>> {
    let array = array.to_vec(cx)?;
    let mut invoices = Vec::with_capacity(array.len());
    for item in array {
        let item = item.downcast_or_throw::<JsObject, _>(cx)?;
        let item = cast_js_object_to_invoice_param(cx, item)?;
        invoices.push(item);
    }
    Ok(invoices)
}
