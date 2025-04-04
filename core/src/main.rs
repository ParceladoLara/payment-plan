use core_payment_plan::{calculate_payment_plan, Params};

fn main() {
    let requested_date = chrono::NaiveDate::from_ymd_opt(2025, 04, 05).unwrap();
    let first_payment_date = chrono::NaiveDate::from_ymd_opt(2025, 05, 3).unwrap();

    let params = Params {
        requested_amount: 7800.0,
        first_payment_date,
        requested_date,
        installments: 4,
        debit_service_percentage: 0,
        mdr: 0.05,
        tac_percentage: 0.0,
        iof_overall: 0.0038,
        iof_percentage: 0.000082,
        interest_rate: 0.0235,
        min_installment_amount: 100.0,
        max_total_amount: 1000000.0,
        disbursement_only_on_business_days: true,
    };

    let mut result = calculate_payment_plan(params).unwrap();

    for i in &result {
        println!("Installment: {:#?}", i);
    }

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
