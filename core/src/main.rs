use core_payment_plan::{
    calculate_reimbursement,
    types::reimbursement::{InvoiceParam, InvoiceStatus, Params},
};

// let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();
fn main() {
    let due_date1 = chrono::NaiveDate::from_ymd_opt(2024, 09, 19).unwrap();
    let due_date2 = chrono::NaiveDate::from_ymd_opt(2024, 10, 19).unwrap();

    let mut invoices = Vec::new();

    invoices.push(InvoiceParam {
        due_at: due_date1,
        id: 1,
        main_iof_tac: 1448.8733387743182,
        original_amount: 1569.3233494592498,
        status: InvoiceStatus::PAID,
    });
    invoices.push(InvoiceParam {
        due_at: due_date2,
        id: 2,
        main_iof_tac: 1506.6833849914135,
        original_amount: 1569.3233494592498,
        status: InvoiceStatus::READJUSTED,
    });

    let params = Params {
        base_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 04).unwrap(),
        fee: 0.3,
        interest_rate: 0.039900000000000005,
        invoice_cost: 2.0,
        invoices,
        max_reimbursement_payment_days: 7,
        max_repurchase_payment_days: 3,
        mdr: 90.0,
    };

    let result = calculate_reimbursement(params).unwrap();

    println!("{:#?}", result);
}
