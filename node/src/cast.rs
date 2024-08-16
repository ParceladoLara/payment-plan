use core_payment_plan::{Params, Response};

use chrono::NaiveTime;
use neon::{
    context::{Context, FunctionContext},
    handle::Handle,
    object::Object,
    result::NeonResult,
    types::{JsArray, JsDate, JsNumber, JsObject, JsValue},
};

use crate::parser::any_to_number;

pub fn cast_js_object_to_param(
    cx: &mut FunctionContext,
    obj: Handle<JsObject>,
) -> NeonResult<Params> {
    let max_total_amount: Handle<JsValue> = obj.get(cx, "maxTotalAmount")?;
    let requested_amount: Handle<JsValue> = obj.get(cx, "requestedAmount")?;
    let first_payment_date: Handle<JsDate> = obj.get(cx, "firstPaymentDate")?;
    let requested_date: Handle<JsDate> = obj.get(cx, "requestedDate")?;
    let installments: Handle<JsValue> = obj.get(cx, "installments")?;
    let debt_service_percentage: Handle<JsValue> = obj.get(cx, "debitServicePercentage")?;
    let mdr: Handle<JsValue> = obj.get(cx, "mdr")?;
    let tac_percentage: Handle<JsValue> = obj.get(cx, "tacPercentage")?;
    let iof_overall: Handle<JsValue> = obj.get(cx, "iofOverall")?;
    let iof_percentage: Handle<JsValue> = obj.get(cx, "iofPercentage")?;
    let interest_rate: Handle<JsValue> = obj.get(cx, "interestRate")?;
    let min_installment_amount: Option<Handle<JsValue>> =
        obj.get_opt(cx, "minInstallmentAmount")?;

    let max_total_amount = any_to_number(cx, max_total_amount)?;
    let requested_amount = any_to_number(cx, requested_amount)?;
    let first_payment_date = first_payment_date.value(cx);
    let requested_date = requested_date.value(cx);
    let installments = any_to_number(cx, installments)? as u32;
    let debt_service_percentage = any_to_number(cx, debt_service_percentage)? as u16;
    let mdr = any_to_number(cx, mdr)?;
    let tac_percentage = any_to_number(cx, tac_percentage)?;
    let iof_overall = any_to_number(cx, iof_overall)?;
    let iof_percentage = any_to_number(cx, iof_percentage)?;
    let interest_rate = any_to_number(cx, interest_rate)?;
    let min_installment_amount = match min_installment_amount {
        Some(value) => any_to_number(cx, value)?,
        None => 0.0,
    };

    let first_payment_date = chrono::DateTime::from_timestamp_millis(first_payment_date as i64);
    let first_payment_date = match first_payment_date {
        Some(date) => date.date_naive(),
        None => {
            return cx.throw_error("Invalid date");
        }
    };

    let requested_date = chrono::DateTime::from_timestamp_millis(requested_date as i64);
    let requested_date = match requested_date {
        Some(date) => date.date_naive(),
        None => {
            return cx.throw_error("Invalid date");
        }
    };

    Ok(Params {
        max_total_amount,
        min_installment_amount,
        requested_amount,
        first_payment_date,
        requested_date,
        installments,
        debit_service_percentage: debt_service_percentage,
        mdr,
        tac_percentage,
        iof_overall,
        iof_percentage,
        interest_rate,
    })
}

fn cast_response_to_js_object<'a, C: Context<'a>>(
    cx: &mut C,
    response: Response,
) -> NeonResult<Handle<'a, JsObject>> {
    let installment = JsNumber::new(cx, response.installment);
    let due_date = response
        .due_date
        .and_time(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
        .and_utc();
    let due_date = JsDate::new(cx, due_date.timestamp_millis() as f64);
    let due_date = match due_date {
        Ok(date) => date,
        Err(_) => {
            return cx.throw_error("Invalid date");
        }
    };
    let accumulated_days = JsNumber::new(cx, response.accumulated_days as f64);
    let days_index = JsNumber::new(cx, response.days_index);
    let accumulated_days_index = JsNumber::new(cx, response.accumulated_days_index);
    let interest_rate = JsNumber::new(cx, response.interest_rate);
    let installment_amount = JsNumber::new(cx, response.installment_amount);
    let installment_amount_without_tac = JsNumber::new(cx, response.installment_amount_without_tac);
    let total_amount = JsNumber::new(cx, response.total_amount);
    let debit_service = JsNumber::new(cx, response.debit_service);
    let customer_debit_service_amount = JsNumber::new(cx, response.customer_debit_service_amount);
    let customer_amount = JsNumber::new(cx, response.customer_amount);
    let calculation_basis_for_effective_interest_rate =
        JsNumber::new(cx, response.calculation_basis_for_effective_interest_rate);
    let merchant_debit_service_amount = JsNumber::new(cx, response.merchant_debit_service_amount);
    let merchant_total_amount = JsNumber::new(cx, response.merchant_total_amount);
    let settled_to_merchant = JsNumber::new(cx, response.settled_to_merchant);
    let mdr_amount = JsNumber::new(cx, response.mdr_amount);
    let effective_interest_rate = JsNumber::new(cx, response.effective_interest_rate);
    let total_effective_cost = JsNumber::new(cx, response.total_effective_cost);
    let eir_yearly = JsNumber::new(cx, response.eir_yearly);
    let tec_yearly = JsNumber::new(cx, response.tec_yearly);
    let eir_monthly = JsNumber::new(cx, response.eir_monthly);
    let tec_monthly = JsNumber::new(cx, response.tec_monthly);
    let total_iof = JsNumber::new(cx, response.total_iof);
    let contract_amount = JsNumber::new(cx, response.contract_amount);
    let contract_amount_without_tac = JsNumber::new(cx, response.contract_amount_without_tac);
    let tac_amount = JsNumber::new(cx, response.tac_amount);
    let iof_percentage = JsNumber::new(cx, response.iof_percentage);
    let overall_iof = JsNumber::new(cx, response.overall_iof);

    let obj = JsObject::new(cx);
    obj.set(cx, "installment", installment)?;
    obj.set(cx, "dueDate", due_date)?;
    obj.set(cx, "accumulatedDays", accumulated_days)?;
    obj.set(cx, "daysIndex", days_index)?;
    obj.set(cx, "accumulatedDaysIndex", accumulated_days_index)?;
    obj.set(cx, "interestRate", interest_rate)?;
    obj.set(cx, "installmentAmount", installment_amount)?;
    obj.set(
        cx,
        "installmentAmountWithoutTAC",
        installment_amount_without_tac,
    )?;
    obj.set(cx, "totalAmount", total_amount)?;
    obj.set(cx, "debitService", debit_service)?;
    obj.set(
        cx,
        "customerDebitServiceAmount",
        customer_debit_service_amount,
    )?;
    obj.set(cx, "customerAmount", customer_amount)?;
    obj.set(
        cx,
        "calculationBasisForEffectiveInterestRate",
        calculation_basis_for_effective_interest_rate,
    )?;
    obj.set(
        cx,
        "merchantDebitServiceAmount",
        merchant_debit_service_amount,
    )?;
    obj.set(cx, "merchantTotalAmount", merchant_total_amount)?;
    obj.set(cx, "settledToMerchant", settled_to_merchant)?;
    obj.set(cx, "mdrAmount", mdr_amount)?;
    obj.set(cx, "effectiveInterestRate", effective_interest_rate)?;
    obj.set(cx, "totalEffectiveCost", total_effective_cost)?;
    obj.set(cx, "eirYearly", eir_yearly)?;
    obj.set(cx, "tecYearly", tec_yearly)?;
    obj.set(cx, "eirMonthly", eir_monthly)?;
    obj.set(cx, "tecMonthly", tec_monthly)?;
    obj.set(cx, "totalIOF", total_iof)?;
    obj.set(cx, "contractAmount", contract_amount)?;
    obj.set(cx, "contractAmountWithoutTAC", contract_amount_without_tac)?;
    obj.set(cx, "tacAmount", tac_amount)?;
    obj.set(cx, "iofPercentage", iof_percentage)?;
    obj.set(cx, "overallIOF", overall_iof)?;

    Ok(obj)
}

pub fn cast_vec_response_to_js_array<'a, C: Context<'a>>(
    cx: &mut C,
    responses: Vec<Response>,
) -> NeonResult<Handle<'a, JsArray>> {
    let array = JsArray::new(cx, responses.len() as usize);
    for (i, response) in responses.into_iter().enumerate() {
        let obj = cast_response_to_js_object(cx, response)?;
        array.set(cx, i as u32, obj)?;
    }
    Ok(array)
}
