use crate::Params;

#[derive(Debug)]
pub struct AmountsResponse {
    pub contract_amount: f64,
    pub contract_amount_without_tac: f64,
    pub installment_amount_without_tac: f64,
    pub installment_amount: f64,
    pub total_amount: f64,
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
    accumulated_days_index: f64,
    installments: f64,
    customer_debit_service_proportion: f64,
    total_iof: f64,
) -> AmountsResponse {
    let debit_service_percentage = params.debit_service_percentage;
    // TOTAL FINANCIADO NA PLANILHA BPM
    let requested_amount = params.requested_amount;
    let tac_amount = params.tac_percentage;

    let contract_amount = requested_amount + tac_amount + total_iof;
    let contract_amount_without_tac = requested_amount + total_iof;

    let installment_amount = contract_amount * (1.0 / accumulated_days_index);
    let installment_amount_without_tac =
        contract_amount_without_tac * (1.0 / accumulated_days_index);

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
        (debit_service + tac_amount) * debit_service_percentage as f64;

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
    use crate::{calc::bmp::amounts::calculate_amounts, Params};

    #[test]
    fn test_calculate_amounts_test_7() {
        let params = Params {
            max_total_amount: f64::MAX,
            min_installment_amount: 0.0,
            requested_amount: 2900.0,
            first_payment_date: chrono::NaiveDate::from_ymd_opt(2022, 04, 30).unwrap(),
            requested_date: chrono::NaiveDate::from_ymd_opt(2022, 03, 30).unwrap(),
            installments: 6,
            debit_service_percentage: 0,
            mdr: 0.029900000000000003,
            tac_percentage: 0.0,
            iof_overall: 0.0038,
            iof_percentage: 0.03,
            interest_rate: 0.035,
        };

        let accumulated_days_index = 0.9650762734315015;
        let installments = 1.0;
        let customer_debit_service_proportion = 1.0;
        let total_iof = 18.40904109589041;

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, 2918.40904109589);
        assert_eq!(amounts.contract_amount_without_tac, 2918.40904109589);
        assert_eq!(amounts.installment_amount_without_tac, 3004.943836914082);
        assert_eq!(amounts.installment_amount, 3024.0190557363558);
        assert_eq!(amounts.total_amount, 3024.0190557363558);
        assert_eq!(amounts.debit_service, 105.61001464046535);
        assert_eq!(amounts.customer_debit_service_amount, 105.61001464046535);
        assert_eq!(amounts.customer_amount, 3024.0190557363553);
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            3005.610014640465
        );
        assert_eq!(amounts.mdr_amount, 86.71000000000001);
        assert_eq!(amounts.merchant_debit_service_amount, 0.0);
        assert_eq!(amounts.merchant_total_amount, 86.71000000000001);
        assert_eq!(amounts.settled_to_merchant, 2813.29);

        let accumulated_days_index = 1.897517117326672;
        let installments = 2.0;
        let customer_debit_service_proportion = 1.0;
        let total_iof = 21.984383561643835;

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, 2921.984383561644);
        assert_eq!(amounts.contract_amount_without_tac, 2921.984383561644);
        assert_eq!(amounts.installment_amount_without_tac, 1528.312958823624);
        assert_eq!(amounts.installment_amount, 1539.8988271991445);
        assert_eq!(amounts.total_amount, 3079.797654398289);
        assert_eq!(amounts.debit_service, 157.81327083664524);
        assert_eq!(amounts.customer_debit_service_amount, 157.81327083664524);
        assert_eq!(amounts.customer_amount, 1539.8988271991445);
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            1528.9066354183226
        );
        assert_eq!(amounts.mdr_amount, 86.71000000000001);
        assert_eq!(amounts.merchant_debit_service_amount, 0.0);
        assert_eq!(amounts.merchant_total_amount, 86.71000000000001);
        assert_eq!(amounts.settled_to_merchant, 2813.29);

        let accumulated_days_index = 2.7973936521483473;
        let installments = 3.0;
        let customer_debit_service_proportion = 1.0;
        let total_iof = 25.639266493150686;

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, 2925.639266493151);
        assert_eq!(amounts.contract_amount_without_tac, 2925.639266493151);
        assert_eq!(amounts.installment_amount_without_tac, 1036.6792667070124);
        assert_eq!(amounts.installment_amount, 1045.8446791163315);
        assert_eq!(amounts.total_amount, 3137.5340373489944);
        assert_eq!(amounts.debit_service, 211.89477085584372);
        assert_eq!(amounts.customer_debit_service_amount, 211.89477085584372);
        assert_eq!(amounts.customer_amount, 1045.8446791163315);
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            1037.298256951948
        );
        assert_eq!(amounts.mdr_amount, 86.71000000000001);
        assert_eq!(amounts.merchant_debit_service_amount, 0.0);
        assert_eq!(amounts.merchant_total_amount, 86.71000000000001);
        assert_eq!(amounts.settled_to_merchant, 2813.29);

        let accumulated_days_index = 3.6668395795122857;
        let installments = 4.0;
        let customer_debit_service_proportion = 1.0;
        let total_iof = 29.254246575342467;

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, 2929.2542465753427);
        assert_eq!(amounts.contract_amount_without_tac, 2929.2542465753427);
        assert_eq!(amounts.installment_amount_without_tac, 790.8717949383865);
        assert_eq!(amounts.installment_amount, 798.8498495930802);
        assert_eq!(amounts.total_amount, 3195.399398372321);
        assert_eq!(amounts.debit_service, 266.1451517969783);
        assert_eq!(amounts.customer_debit_service_amount, 266.1451517969783);
        assert_eq!(amounts.customer_amount, 798.8498495930802);
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            791.5362879492445
        );
        assert_eq!(amounts.mdr_amount, 86.71000000000001);
        assert_eq!(amounts.merchant_debit_service_amount, 0.0);
        assert_eq!(amounts.merchant_total_amount, 86.71000000000001);
        assert_eq!(amounts.settled_to_merchant, 2813.29);

        let accumulated_days_index = 4.505921215042871;
        let installments = 5.0;
        let customer_debit_service_proportion = 1.0;
        let total_iof = 32.90109589041096;

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, 2932.901095890411);
        assert_eq!(amounts.contract_amount_without_tac, 2932.901095890411);
        assert_eq!(amounts.installment_amount_without_tac, 643.5975822920393);
        assert_eq!(amounts.installment_amount, 650.8993291092211);
        assert_eq!(amounts.total_amount, 3254.4966455461054);
        assert_eq!(amounts.debit_service, 321.5955496556944);
        assert_eq!(amounts.customer_debit_service_amount, 321.5955496556944);
        assert_eq!(amounts.customer_amount, 650.8993291092211);
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            644.3191099311389
        );
        assert_eq!(amounts.mdr_amount, 86.71000000000001);
        assert_eq!(amounts.merchant_debit_service_amount, 0.0);
        assert_eq!(amounts.merchant_total_amount, 86.71000000000001);
        assert_eq!(amounts.settled_to_merchant, 2813.29);

        let accumulated_days_index = 5.315698992965537;
        let installments = 6.0;
        let customer_debit_service_proportion = 1.0;
        let total_iof = 36.56358345205479;

        let amounts = calculate_amounts(
            params,
            accumulated_days_index,
            installments,
            customer_debit_service_proportion,
            total_iof,
        );

        assert_eq!(amounts.contract_amount, 2936.563583452055);
        assert_eq!(amounts.contract_amount_without_tac, 2936.563583452055);
        assert_eq!(amounts.installment_amount_without_tac, 545.5538403957181);
        assert_eq!(amounts.installment_amount, 552.4322553512001);
        assert_eq!(amounts.total_amount, 3314.5935321072);
        assert_eq!(amounts.debit_service, 378.0299486551454);
        assert_eq!(amounts.customer_debit_service_amount, 378.0299486551454);
        assert_eq!(amounts.customer_amount, 552.4322553512001);
        assert_eq!(
            amounts.calculation_basis_for_effective_interest_rate,
            546.3383247758576
        );
        assert_eq!(amounts.mdr_amount, 86.71000000000001);
        assert_eq!(amounts.merchant_debit_service_amount, 0.0);
        assert_eq!(amounts.merchant_total_amount, 86.71000000000001);
        assert_eq!(amounts.settled_to_merchant, 2813.29);
    }
}
