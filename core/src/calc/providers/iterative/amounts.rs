use rust_decimal::Decimal;

use crate::Params;

#[derive(Debug, PartialEq)]
pub struct AmountsResponse {
    pub debit_service: Decimal,
    pub customer_debit_service_amount: Decimal,
    pub customer_amount: Decimal,
    pub calculation_basis_for_effective_interest_rate: Decimal,
    pub mdr_amount: Decimal,
    pub merchant_debit_service_amount: Decimal,
    pub merchant_total_amount: Decimal,
    pub settled_to_merchant: Decimal,
}

pub fn calc(
    params: Params,
    installments: Decimal,
    customer_debit_service_proportion: Decimal,
    total_iof: Decimal,
    total_amount: Decimal,
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
        (debit_service + tac_amount) * Decimal::from(debit_service_percentage);

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
    use rust_decimal::{dec, Decimal};

    use crate::{calc::providers::iterative::amounts::AmountsResponse, Params};

    #[test]
    fn test_calc() {
        let expected = AmountsResponse {
            debit_service: dec!(3264.9940111656333),
            customer_debit_service_amount: dec!(3264.9940111656333),
            customer_amount: dec!(605.4000559686463),
            calculation_basis_for_effective_interest_rate: dec!(594.2218895092018),
            mdr_amount: dec!(371.55),
            merchant_debit_service_amount: Decimal::ZERO,
            merchant_total_amount: dec!(371.55),
            settled_to_merchant: dec!(7059.45),
        };

        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 09, 24).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 24).unwrap();

        let params = Params {
            disbursement_only_on_business_days: false,
            requested_amount: dec!(7431.0),
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 18,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.04),
            min_installment_amount: dec!(100.0),
            max_total_amount: Decimal::MAX,
        };
        let installments = 18;
        let debit_service_proportion = Decimal::ONE;
        let iof = dec!(201.20699627);
        let total_amount = dec!(10897.201007435633);

        let amounts = super::calc(
            params,
            installments.into(),
            debit_service_proportion,
            iof,
            total_amount,
        );

        assert_eq!(amounts, expected);
    }
}
