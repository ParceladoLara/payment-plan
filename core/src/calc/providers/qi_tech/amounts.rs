use crate::Params;

#[derive(Debug)]
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

pub fn calculate_amounts(
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
