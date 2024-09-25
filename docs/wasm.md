# WASM

This is the package responsible for generating the `wasm` files that will be used in the browser.

It uses [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) to achieve this.

## Important libs

- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
  - Facilitates high-level interactions between wasm modules and JavaScript.
- [tsify-next](https://github.com/AmbientRun/tsify-next)
  - For generating the typescript types on top of the rust structs for the wasm module.
- [js-sys](https://docs.rs/js-sys/0.3.70/js_sys/)
  - For better interaction with JavaScript types.

## Maintenance

This project is basically a interface to serialize/deserialize the data so the `core` package can calculate the payment plan and return the result, with means that the maintenance of this package is minimal and only necessary if the `core` package changes.

This project in terms of syntax is pretty complex on the details of the serialization/deserialization of the data, and pretty heavy on the macro usage.

But if you simple want to add a new functionality you can just "accept" things that are already implemented, like the serialization/deserialization of the data,copy and paste the code and change the necessary parts.

## Running the tests

To run the tests you can use the following command:

```bash
npm run test
```