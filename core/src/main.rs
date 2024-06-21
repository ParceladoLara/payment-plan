use core_payment_plan::{calc::calculate_payment_plan, Params};

fn main() {

    let first_payment_date = chrono::DateTime::from_timestamp_millis(1719025200000)
        .unwrap()
        .date_naive();

    let requested_date = chrono::DateTime::from_timestamp_millis(1718983261490)
        .unwrap()
        .date_naive();

    let params = Params {
        min_installment_amount: 100.0,
        requested_amount: 2770.71,
        first_payment_date,
        requested_date,
        installments: 48,
        debit_service_percentage: 0,
        mdr: 0.029999999329447746,
        tac_percentage: 0.0,
        iof_overall: 0.003800000064074993,
        iof_percentage: 0.029999999329447746,
        interest_rate: 0.029999999329447746,
    };

    let result = calculate_payment_plan(params).unwrap();
    for response in result {
        println!("installment {}", response.installment);
        println!("due_date {}", response.due_date);
        println!("accumulated_days {}", response.accumulated_days);
        println!("days_index {}", response.days_index);
        println!("accumulated_days_index {}", response.accumulated_days_index);
        println!("interest_rate {}", response.interest_rate);
        println!("installment_amount {}", response.installment_amount);
        println!(
            "installment_amount_without_tac {}",
            response.installment_amount_without_tac
        );
        println!("total_amount {}", response.total_amount);
        println!("debt_service {}", response.debit_service);
        println!(
            "customer_debt_service_amount {}",
            response.customer_debit_service_amount
        );
        println!("customer_amount {}", response.customer_amount);
        println!(
            "calculation_basis_for_effective_interest_rate {}",
            response.calculation_basis_for_effective_interest_rate
        );
        println!(
            "merchant_debt_service_amount {}",
            response.merchant_debit_service_amount
        );
        println!("merchant_total_amount {}", response.merchant_total_amount);
        println!("settled_to_merchant {}", response.settled_to_merchant);
        println!("mdr_amount {}", response.mdr_amount);
        println!(
            "effective_interest_rate {}",
            response.effective_interest_rate
        );
        println!("total_effective_cost {}", response.total_effective_cost);
        println!("eir_yearly {}", response.eir_yearly);
        println!("tec_yearly {}", response.tec_yearly);
        println!("eir_monthly {}", response.eir_monthly);
        println!("tec_monthly {}", response.tec_monthly);
        println!("total_iof {}", response.total_iof);
        println!("contract_amount {}", response.contract_amount);
        println!(
            "contract_amount_without_tac {}",
            response.contract_amount_without_tac
        );
        println!("tac_amount {}", response.tac_amount);
        println!("iof_percentage {}", response.iof_percentage);
        println!("overall_iof {}", response.overall_iof);
        println!("-------------------");
        break;
    }
}
