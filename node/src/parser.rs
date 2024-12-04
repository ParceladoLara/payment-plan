use neon::{
    context::{Context, FunctionContext},
    handle::Handle,
    result::NeonResult,
    types::{JsBoolean, JsNull, JsNumber, JsString, JsUndefined, JsValue},
};

pub fn any_to_number<'a>(cx: &mut FunctionContext<'a>, js_any: Handle<JsValue>) -> NeonResult<f64> {
    if let Ok(num) = js_any.downcast::<JsNumber, _>(cx) {
        Ok(num.value(cx))
    } else if let Ok(str) = js_any.downcast::<JsString, _>(cx) {
        let str = str.value(cx);
        match str.parse::<f64>() {
            Ok(num) => Ok(num),
            Err(_) => {
                let error = format!("Value cannot be converted to number: {:?}", js_any);
                let js_error = cx.string(error);
                cx.throw(js_error)
            }
        }
    } else if let Ok(_) = js_any.downcast::<JsNull, _>(cx) {
        Ok(0.0)
    } else if let Ok(_) = js_any.downcast::<JsUndefined, _>(cx) {
        Ok(0.0)
    } else {
        let error = format!("Value cannot be converted to number: {:?}", js_any);
        let js_error = cx.string(error);
        cx.throw(js_error)
    }
}

pub fn any_to_bool<'a>(cx: &mut FunctionContext<'a>, js_any: Handle<JsValue>) -> NeonResult<bool> {
    if let Ok(bool) = js_any.downcast::<JsBoolean, _>(cx) {
        Ok(bool.value(cx))
    } else if let Ok(str) = js_any.downcast::<JsString, _>(cx) {
        let str = str.value(cx);
        match str.parse::<bool>() {
            Ok(bool) => Ok(bool),
            Err(_) => {
                let error = format!("Value cannot be converted to boolean: {:?}", js_any);
                let js_error = cx.string(error);
                cx.throw(js_error)
            }
        }
    } else if let Ok(_) = js_any.downcast::<JsNull, _>(cx) {
        Ok(false)
    } else if let Ok(_) = js_any.downcast::<JsUndefined, _>(cx) {
        Ok(false)
    } else {
        let error = format!("Value cannot be converted to boolean: {:?}", js_any);
        let js_error = cx.string(error);
        cx.throw(js_error)
    }
}
