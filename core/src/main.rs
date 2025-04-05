use chrono::Datelike;
use core_payment_plan::{calculate_down_payment_plan, DownPaymentParams, Params}; // Import the Datelike trait to access the `year` method

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

    let down_payment_params = DownPaymentParams {
        params,
        first_payment_date,
        installments: 4,
        min_installment_amount: 100.0,
        requested_amount: 1000.0,
    };

    let result = calculate_down_payment_plan(down_payment_params).unwrap();

    let mut buff = String::new();
    buff.push_str(r#"        expected = ["#);

    for i in &result {
        buff.push_str(&format!(
            r#"
            DownPaymentResponse(
                installment_amount={},
                total_amount={},
                installment_quantity={},
                first_payment_date=datetime({}, {}, {}, tzinfo=timezone(timedelta(hours=-3))),
                plans=["#,
            i.installment_amount,
            i.total_amount,
            i.installment_quantity,
            i.first_payment_date.year(),
            i.first_payment_date.month(),
            i.first_payment_date.day(),
        ));

        for j in &i.plans {
            buff.push_str(&format!(
                r#"
                    Response(
                        installment={},
                        due_date=datetime({}, {}, {}, tzinfo=timezone(timedelta(hours=-3))),
                        disbursement_date=datetime({}, {}, {}, tzinfo=timezone(timedelta(hours=-3))),
                        accumulated_days={},
                        days_index={},
                        accumulated_days_index={},
                        interest_rate={},
                        installment_amount={},
                        installment_amount_without_tac={},
                        total_amount={},
                        debit_service={},
                        customer_debit_service_amount={},
                        customer_amount={},
                        calculation_basis_for_effective_interest_rate={},
                        merchant_debit_service_amount={},
                        merchant_total_amount={},
                        settled_to_merchant={},
                        mdr_amount={},
                        effective_interest_rate={},
                        total_effective_cost={},
                        eir_yearly={},
                        tec_yearly={},
                        eir_monthly={},
                        tec_monthly={},
                        total_iof={},
                        contract_amount={},
                        contract_amount_without_tac={},
                        tac_amount={},
                        iof_percentage={},
                        overall_iof={},
                        pre_disbursement_amount={},
                        paid_total_iof={},
                        paid_contract_amount={}
                    ),"#,
                j.installment,
                j.due_date.year(),
                j.due_date.month(),
                j.due_date.day(),
                j.disbursement_date.year(),
                j.disbursement_date.month(),
                j.disbursement_date.day(),
                j.accumulated_days,
                j.days_index,
                j.accumulated_days_index,
                j.interest_rate,
                j.installment_amount,
                j.installment_amount_without_tac,
                j.total_amount,
                j.debit_service,
                j.customer_debit_service_amount,
                j.customer_amount,
                j.calculation_basis_for_effective_interest_rate,
                j.merchant_debit_service_amount,
                j.merchant_total_amount,
                j.settled_to_merchant,
                j.mdr_amount,
                j.effective_interest_rate,
                j.total_effective_cost,
                j.eir_yearly,
                j.tec_yearly,
                j.eir_monthly,
                j.tec_monthly,
                j.total_iof,
                j.contract_amount,
                j.contract_amount_without_tac,
                j.tac_amount,
                j.iof_percentage,
                j.overall_iof,
                j.pre_disbursement_amount,
                j.paid_total_iof,
                j.paid_contract_amount,
            ));
        }
        buff.push_str("                ]\n");
        buff.push_str("            ),\n");
    }
    buff.push_str("        ]\n");
    println!("{}", buff);
}
