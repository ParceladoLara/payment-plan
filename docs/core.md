# CORE

The core package contains the core logic of the Lara Payment Plan. 

It contains 2 exposed functions [here](../core/src/calc/mod.rs):

- `calculate`: This function calculates the payment plan for a given credit proposal.
- `calculate_payment_plan`: This function calculates a payment plan for a given credit proposal with a down payment.

And 4 exposed structs [here](../core/src/lib.rs):
- `Params`: This represents the input parameters for the credit proposal.
- `Response`: This represents the response of the payment plan calculation.
- `DownPaymentParams`: This represents the input parameters for the credit proposal with a down payment.
- `DownPaymentResponse`: This represents the response of the payment plan calculation with a down payment.

Start by looking at the [lib.rs](../core/src/lib.rs) file to understand the structure of the package.

## Maintenance

This package is the core of the Lara Payment Plan, it contains the business logic of the payment plan calculation.

So the maintenance of this package should be done with care, only if necessary, and with a lot of tests.

This is the least complicated package in terms of dependencies, your Rust knowledge doesn't need to be huge cause it's basically just math.

Start by looking at the [lib.rs](../core/src/lib.rs) and go from there.
