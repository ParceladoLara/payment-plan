use core_payment_plan::{calculate_payment_plan, Params};

fn main() {
    let requested_date = chrono::NaiveDate::from_ymd_opt(2025, 02, 26).unwrap();
    let first_payment_date = chrono::NaiveDate::from_ymd_opt(2025, 03, 2).unwrap();

    let requested_amount = 4000.00;
    let installments = 18;
    let interest_rate = 0.0436; //Interest rate do caiao da massa

    let params = Params {
        max_total_amount: f64::MAX,
        min_installment_amount: 0.0,
        requested_amount,
        first_payment_date,
        requested_date,
        installments,
        debit_service_percentage: 0,
        mdr: 0.1,
        tac_percentage: 0.0,
        iof_overall: 0.0038,      // %0.38
        iof_percentage: 0.000082, // 0.0082%
        interest_rate,
        disbursement_only_on_business_days: true,
    };

    let mut result = calculate_payment_plan(params).unwrap();

    println!("Length: {}", result.len());

    let result = result.pop().unwrap();

    println!("Installment: {:#?}", result);

    println!("CET: {}", result.total_effective_cost);
    println!("annual_cet: {}", result.tec_yearly);
    println!("monthly_cet: {}", result.tec_monthly);
    println!("installment_amount: {}", result.installment_amount);
    println!("IOF: {}", result.total_iof);
    println!("Contract Amount: {}", result.contract_amount);
}
