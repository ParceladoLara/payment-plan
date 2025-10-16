use rust_decimal::Decimal;

use crate::Params;

#[derive(Debug)]
pub struct AmountsResponse {
    pub contract_amount: Decimal,
    pub contract_amount_without_tac: Decimal,
    pub installment_amount_without_tac: Decimal,
    pub installment_amount: Decimal,
    pub total_amount: Decimal,
    pub debit_service: Decimal,
    pub customer_debit_service_amount: Decimal,
    pub customer_amount: Decimal,
    pub calculation_basis_for_effective_interest_rate: Decimal,
    pub mdr_amount: Decimal,
    pub merchant_debit_service_amount: Decimal,
    pub merchant_total_amount: Decimal,
    pub settled_to_merchant: Decimal,
}

pub fn calculate_amounts(
    params: Params,
    accumulated_days_index: Decimal,
    installments: Decimal,
    customer_debit_service_proportion: Decimal,
    total_iof: Decimal,
) -> AmountsResponse {
    let debit_service_percentage = params.debit_service_percentage;
    // TOTAL FINANCIADO NA PLANILHA BPM
    let requested_amount = params.requested_amount;
    let tac_amount = params.tac_percentage;

    let contract_amount = requested_amount + tac_amount + total_iof;
    let contract_amount_without_tac = requested_amount + total_iof;

    let installment_amount = contract_amount * (Decimal::ONE / accumulated_days_index);
    let installment_amount_without_tac =
        contract_amount_without_tac * (Decimal::ONE / accumulated_days_index);

    let total_amount = installment_amount * installments;
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
        contract_amount,
        contract_amount_without_tac,
        installment_amount_without_tac,
        installment_amount,
        total_amount,
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

    use crate::{calc::providers::simple::amounts::calculate_amounts, Params};

    #[test]
    fn test_calculate_amounts_test_7() {
        let params = Params {
            disbursement_only_on_business_days: false,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ZERO,
            requested_amount: dec!(2900.0),
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            disbursement_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 30).unwrap(),
            installments: 6,
            debit_service_percentage: 0,
            mdr: dec!(0.029900000000000003),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),
            iof_percentage: dec!(0.03),
            interest_rate: dec!(0.035),
        };

        let accumulated_days_index = dec!(0.9650762734315015);
        let installments = Decimal::ONE;
        let customer_debit_service_proportion = Decimal::ONE;
        let total_iof = dec!(18.40904109589041);

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, dec!(2918.40904109589));
        assert_eq!(amounts.contract_amount_without_tac, dec!(2918.40904109589));
        assert_eq!(
            amounts.installment_amount_without_tac,
            dec!(3024.0190557363558)
        );
        assert_eq!(amounts.installment_amount, dec!(3024.0190557363558));
        assert_eq!(amounts.total_amount, dec!(3024.0190557363558));
        assert_eq!(amounts.debit_service, dec!(105.61001464046535));
        assert_eq!(
            amounts.customer_debit_service_amount,
            dec!(105.61001464046535)
        );
        assert_eq!(amounts.customer_amount, dec!(3024.0190557363553));
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            dec!(3005.610014640465)
        );
        assert_eq!(amounts.mdr_amount, dec!(86.71000000000001));
        assert_eq!(amounts.merchant_debit_service_amount, Decimal::ZERO);
        assert_eq!(amounts.merchant_total_amount, dec!(86.71000000000001));
        assert_eq!(amounts.settled_to_merchant, dec!(2813.29));

        let accumulated_days_index = dec!(1.897517117326672);
        let installments = dec!(2.0);
        let customer_debit_service_proportion = Decimal::ONE;
        let total_iof = dec!(21.984383561643835);

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, dec!(2921.984383561644));
        assert_eq!(amounts.contract_amount_without_tac, dec!(2921.984383561644));
        assert_eq!(
            amounts.installment_amount_without_tac,
            dec!(1539.8988271991445)
        );
        assert_eq!(amounts.installment_amount, dec!(1539.8988271991445));
        assert_eq!(amounts.total_amount, dec!(3079.797654398289));
        assert_eq!(amounts.debit_service, dec!(157.81327083664524));
        assert_eq!(
            amounts.customer_debit_service_amount,
            dec!(157.81327083664524)
        );
        assert_eq!(amounts.customer_amount, dec!(1539.8988271991445));
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            dec!(1528.9066354183226)
        );
        assert_eq!(amounts.mdr_amount, dec!(86.71000000000001));
        assert_eq!(amounts.merchant_debit_service_amount, Decimal::ZERO);
        assert_eq!(amounts.merchant_total_amount, dec!(86.71000000000001));
        assert_eq!(amounts.settled_to_merchant, dec!(2813.29));

        let accumulated_days_index = dec!(2.7973936521483473);
        let installments = dec!(3.0);
        let customer_debit_service_proportion = Decimal::ONE;
        let total_iof = dec!(25.639266493150686);

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, dec!(2925.639266493151));
        assert_eq!(amounts.contract_amount_without_tac, dec!(2925.639266493151));
        assert_eq!(
            amounts.installment_amount_without_tac,
            dec!(1045.8446791163315)
        );
        assert_eq!(amounts.installment_amount, dec!(1045.8446791163315));
        assert_eq!(amounts.total_amount, dec!(3137.5340373489944));
        assert_eq!(amounts.debit_service, dec!(211.89477085584372));
        assert_eq!(
            amounts.customer_debit_service_amount,
            dec!(211.89477085584372)
        );
        assert_eq!(amounts.customer_amount, dec!(1045.8446791163315));
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            dec!(1037.298256951948)
        );
        assert_eq!(amounts.mdr_amount, dec!(86.71000000000001));
        assert_eq!(amounts.merchant_debit_service_amount, Decimal::ZERO);
        assert_eq!(amounts.merchant_total_amount, dec!(86.71000000000001));
        assert_eq!(amounts.settled_to_merchant, dec!(2813.29));

        let accumulated_days_index = dec!(3.6668395795122857);
        let installments = dec!(4.0);
        let customer_debit_service_proportion = Decimal::ONE;
        let total_iof = dec!(29.254246575342467);

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, dec!(2929.2542465753427));
        assert_eq!(
            amounts.contract_amount_without_tac,
            dec!(2929.2542465753427)
        );
        assert_eq!(
            amounts.installment_amount_without_tac,
            dec!(798.8498495930802)
        );
        assert_eq!(amounts.installment_amount, dec!(798.8498495930802));
        assert_eq!(amounts.total_amount, dec!(3195.399398372321));
        assert_eq!(amounts.debit_service, dec!(266.1451517969783));
        assert_eq!(
            amounts.customer_debit_service_amount,
            dec!(266.1451517969783)
        );
        assert_eq!(amounts.customer_amount, dec!(798.8498495930802));
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            dec!(791.5362879492445)
        );
        assert_eq!(amounts.mdr_amount, dec!(86.71000000000001));
        assert_eq!(amounts.merchant_debit_service_amount, Decimal::ZERO);
        assert_eq!(amounts.merchant_total_amount, dec!(86.71000000000001));
        assert_eq!(amounts.settled_to_merchant, dec!(2813.29));

        let accumulated_days_index = dec!(4.505921215042871);
        let installments = dec!(5.0);
        let customer_debit_service_proportion = Decimal::ONE;
        let total_iof = dec!(32.90109589041096);

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, dec!(2932.901095890411));
        assert_eq!(amounts.contract_amount_without_tac, dec!(2932.901095890411));
        assert_eq!(
            amounts.installment_amount_without_tac,
            dec!(650.8993291092211)
        );
        assert_eq!(amounts.installment_amount, dec!(650.8993291092211));
        assert_eq!(amounts.total_amount, dec!(3254.4966455461054));
        assert_eq!(amounts.debit_service, dec!(321.5955496556944));
        assert_eq!(
            amounts.customer_debit_service_amount,
            dec!(321.5955496556944)
        );
        assert_eq!(amounts.customer_amount, dec!(650.8993291092211));
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            dec!(644.3191099311389)
        );
        assert_eq!(amounts.mdr_amount, dec!(86.71000000000001));
        assert_eq!(amounts.merchant_debit_service_amount, Decimal::ZERO);
        assert_eq!(amounts.merchant_total_amount, dec!(86.71000000000001));
        assert_eq!(amounts.settled_to_merchant, dec!(2813.29));

        let accumulated_days_index = dec!(5.315698992965537);
        let installments = dec!(6.0);
        let customer_debit_service_proportion = Decimal::ONE;
        let total_iof = dec!(36.56358345205479);

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, dec!(2936.563583452055));
        assert_eq!(amounts.contract_amount_without_tac, dec!(2936.563583452055));
        assert_eq!(
            amounts.installment_amount_without_tac,
            dec!(552.4322553512001)
        );
        assert_eq!(amounts.installment_amount, dec!(552.4322553512001));
        assert_eq!(amounts.total_amount, dec!(3314.5935321072));
        assert_eq!(amounts.debit_service, dec!(378.0299486551454));
        assert_eq!(
            amounts.customer_debit_service_amount,
            dec!(378.0299486551454)
        );
        assert_eq!(amounts.customer_amount, dec!(552.4322553512001));
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            dec!(546.3383247758576)
        );
        assert_eq!(amounts.mdr_amount, dec!(86.71000000000001));
        assert_eq!(amounts.merchant_debit_service_amount, Decimal::ZERO);
        assert_eq!(amounts.merchant_total_amount, dec!(86.71000000000001));
        assert_eq!(amounts.settled_to_merchant, dec!(2813.29));
    }
}
