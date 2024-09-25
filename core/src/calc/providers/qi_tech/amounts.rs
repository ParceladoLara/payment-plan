use crate::Params;

#[derive(Debug, PartialEq)]
pub struct AmountsResponse {
    pub debit_service: f64,
    pub customer_debit_service_amount: f64,
    pub customer_amount: f64,
    pub calculation_basis_for_effective_interest_rate: f64,
    pub mdr_amount: f64,
    pub merchant_debit_service_amount: f64,
    pub merchant_total_amount: f64,
    pub settled_to_merchant: f64,
}

pub fn calc(
    params: Params,
    installments: f64,
    customer_debit_service_proportion: f64,
    total_iof: f64,
    total_amount: f64,
) -> AmountsResponse {
    let debit_service_percentage = params.debit_service_percentage;
    // TOTAL FINANCIADO NA PLANILHA BPM
    let requested_amount = params.requested_amount;
    let tac_amount = params.tac_percentage;

    let debit_service = total_amount - requested_amount - tac_amount - total_iof;
    let customer_debit_service_amount = debit_service * customer_debit_service_proportion;

    // CALCULATION BASIS FOR totalEffectiveCost
    let customer_amount = (requested_amount
        + (debit_service + tac_amount) * customer_debit_service_proportion
        + total_iof)
        / installments;

    let calculation_basis_for_effective_interest_rate =
        (requested_amount + debit_service * customer_debit_service_proportion) / installments;

    let mdr_amount = requested_amount * params.mdr;

    let merchant_debit_service_amount =
        (debit_service + tac_amount) * debit_service_percentage as f64;

    let merchant_total_amount = merchant_debit_service_amount + mdr_amount;

    let settled_to_merchant = requested_amount - merchant_total_amount;

    return AmountsResponse {
        debit_service,
        customer_debit_service_amount,
        customer_amount,
        calculation_basis_for_effective_interest_rate,
        mdr_amount,
        merchant_debit_service_amount,
        merchant_total_amount,
        settled_to_merchant,
    };
}

#[cfg(test)]
mod test {
    use crate::{calc::providers::qi_tech::amounts::AmountsResponse, Params};

    #[test]
    fn test_calc() {
        let expected = AmountsResponse {
            debit_service: 3264.9940111656333,
            customer_debit_service_amount: 3264.9940111656333,
            customer_amount: 605.4000559686463,
            calculation_basis_for_effective_interest_rate: 594.2218895092018,
            mdr_amount: 371.55,
            merchant_debit_service_amount: 0.0,
            merchant_total_amount: 371.55,
            settled_to_merchant: 7059.45,
        };

        let requested_date = chrono::NaiveDate::from_ymd_opt(2024, 09, 24).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap();

        let params = Params {
            requested_amount: 7431.0,
            first_payment_date,
            requested_date,
            installments: 18,
            debit_service_percentage: 0,
            mdr: 0.05,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.04,
            min_installment_amount: 100.0,
            max_total_amount: f64::MAX,
        };
        let installments = 18;
        let debit_service_proportion = 1.0;
        let iof = 201.20699627;
        let total_amount = 10897.201007435633;

        let amounts = super::calc(
            params,
            installments as f64,
            debit_service_proportion,
            iof,
            total_amount,
        );

        assert_eq!(amounts, expected);
    }
}
