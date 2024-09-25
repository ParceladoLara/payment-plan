use std::fs::File;
use std::io::Write;

use core_payment_plan::{calculate_payment_plan, Params};

fn main() {
    let i = vec![6, 12, 18, 24];

    for i in i {
        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 09, 24).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap();

        let requested_amount = 7431.00;
        let installments = i;
        let interest_rate = 0.04;

        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 100.0,
            requested_amount,
            first_payment_date,
            requested_date,
            installments,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,  // %0.38
            iof_percentage: 0.03, // 0.0082%
            interest_rate,
        };

        let file_name = format!("./csv/output_{}_{}.csv", requested_amount, i);

        let mut result = calculate_payment_plan(params).unwrap();

        let mut file = File::create(file_name).unwrap();

        // Write the headers
        writeln!(file, "valor solicitado;qtd parcelas;taxa de juros;data de requisicao;data de vencimento primeira parcela;data de vencimento ultima parcela;dias acumulados;fator acumulado;valor da parcela;iof;valor total;taxa de juros efetiva;valor total efetivo").unwrap() ;

        let response = result.pop().unwrap();

        writeln!(
            file,
            "{:.2};{:.0};{:.15};{};{};{};{:.0};{};{:.2};{};{:.2};{:.15};{:.15}",
            format!("{:.2}", requested_amount).replace('.', ","),
            response.installment,
            format!("{:.15}", interest_rate).replace('.', ","),
            requested_date,
            first_payment_date,
            response.due_date,
            response.accumulated_days,
            format!("{:.15}", response.accumulated_days_index).replace('.', ","),
            response.installment_amount,
            format!("{:.15}", response.total_iof).replace('.', ","),
            response.total_amount,
            format!("{:.15}", response.effective_interest_rate).replace('.', ","),
            format!("{:.15}", response.total_effective_cost).replace('.', ",")
        )
        .unwrap();
    }
}
