# How to navigate the codebase

Rust has a tree-like structure with modules and submodules. every project can have 1 or 2 entry points, the `main.rs` and the `lib.rs`. The `main.rs` is the entry point for binaries and the `lib.rs` is the entry point for libraries.

So the `cli` package has a [main.rs](../cli/src/main.rs) file that builds the CLI binary.

The same it's not true for the `node` package [lib.rs](../node/src/lib.rs) for the sake of the Neon library standard.

The `core` package has a [lib.rs](../core/src/lib.rs) file that builds the library that is used by the other packages, it also has a [main.rs](../core/src/main.rs) file that is used for testing the library.

So does entry point files are the root of the tree, on the top of this files you will see the `mod` keyword that imports the submodules of the package.
`mod`keyword with `pub` visibility means that the module is public and can be accessed from outside the package. if not, the module is private and can only be accessed from inside the module that it is declared our above it.


This is a basic explanation of rust modules, you can see more about it [here](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

# General Concepts

## Traits

Traits are a way to define a set of methods that a type must implement. This is similar to interfaces in other languages.
[traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

The `derive` attribute is used to automatically implement traits for a type. This is similar to annotations in other languages.
[derive](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)

```rust
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Params {
    pub requested_amount: f64,
    pub first_payment_date: chrono::NaiveDate,
    pub disbursement_date: chrono::NaiveDate,
    pub installments: u32,
    pub debit_service_percentage: u16,
    pub mdr: f64,
    pub tac_percentage: f64,
    pub iof_overall: f64,
    pub iof_percentage: f64,
    pub interest_rate: f64,
    pub min_installment_amount: f64,
    pub max_total_amount: f64,
}
```

This struct now implements following traits:
- Debug:
  - This allows the struct to be printed using the `{:?}` formatter.

- Clone:

  - This allows the struct to be cloned. `clone` creates a deep copy of the struct.
  When a struct only implements `Clone` generally it means that the size of the struct is not fixed and the values are stored on the heap,calling `clone` will create another struct with the same values on the heap.
  The `clone()` invocation is explicit, meaning that you have to call it to create a new instance of the struct.

- Copy:
  - This allows the struct to be copied. `copy` is a type of clone but for types and values that have a fixed size. this means that the values will be copied on the stack.(copy requires that the type implements the `Clone` trait)
  The `copy` invocation is implicit, meaning that the struct will be copied when passed to a function or assigned to a variable.

- Deserialize:
  - This allows the struct to be deserialized from a JSON string. This is useful when working with JSON data.

There are many other traits but these are the most common ones on this project.

