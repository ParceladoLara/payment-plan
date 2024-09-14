# Node

This is the package responsible for generating the `.node` file used on the node server.
The idea is to have a more specific implementation for the node server by the fact that so much of the Lara code is written in Node.

It uses [neon](https://neon-rs.dev) to achieve this.

## Important libs

- [neon](https://neon-rs.dev)
  - A Rust library for writing safe and fast native Node.js add-ons.

## Maintenance

This project is basically a interface to serialize/deserialize the data so the `core` package can calculate the payment plan and return the result, with means that the maintenance of this package is minimal and only necessary if the `core` package changes.

This project in terms of syntax is the most complex of all, there are [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) appearing, and[ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) start to play a role in the code.

To really maintain this project you need to have a good understanding of Rust, Node and Neon.

But for just adding basic functionalities you can use the utilities functions that are already implemented, for casting the data from Rust to Node and vice versa.

example:

```typescript
export type DownPaymentPlanParams = {
  params: PaymentPlanParams;
  requestedAmount: number;
  firstPaymentDate: Date;
  installments: number;
  minInstallmentAmount: number;
};

const params: DownPaymentPlanParams = {
    params: planParam,
    requestedAmount: downPayment,
    minInstallmentAmount,
    installments,
    firstPaymentDate: new Date('2022-06-20'),
  };
```

This object will be casted to Rust using the `cast_js_object_to_down_payment_param` function.

```rust
pub fn cast_js_object_to_down_payment_param(
    cx: &mut FunctionContext,
    obj: Handle<JsObject>,
) -> NeonResult<DownPaymentParams> {
    let params: Handle<JsObject> = obj.get(cx, "params")?;
    let request_amount: Handle<JsValue> = obj.get(cx, "requestedAmount")?;
    let min_installment_amount: Handle<JsValue> = obj.get(cx, "minInstallmentAmount")?;
    let first_payment_date_millis: Handle<JsDate> = obj.get(cx, "firstPaymentDate")?;
    let installments: Handle<JsValue> = obj.get(cx, "installments")?;

    let params = cast_js_object_to_param(cx, params)?;
    let request_amount = any_to_number(cx, request_amount)?;
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

    Ok(DownPaymentParams {
        params,
        request_amount,
        min_installment_amount,
        first_payment_date,
        installments,
    })
}
```	

So you can see that the function receives a Context that represents the Node context, and a Handle of the JsObject that represents a safe way to access the garbage collected object in Node.

The obj.get(cx, "params")? is a way to get the value of the key "params" from the object.

From there you can use the utility functions to cast the primitive types to Rust types.

To create a JsObject from a Rust object is just 

```rust
  let obj = JsObject::new(cx);
  obj.set(cx, "installmentAmount", installment_amount)?;
  obj.set(cx, "totalAmount", total_amount)?;
  obj.set(cx, "installmentQuantity", installment_quantity)?;
  obj.set(cx, "firstPaymentDate", first_payment_date)?;
  obj.set(cx, "plans", plans)?;
```
JsObject::new(cx) is the same doing `const obj = {}` in Node.
And obj.set(cx,"foo",bar)? is the same as doing `obj.foo = bar` in Node.

## Testing

Different from the other packages where you test the code using the `cargo test` command, in this package you need to test the code using the Node test files.

Just run `npm run test` to run the tests.