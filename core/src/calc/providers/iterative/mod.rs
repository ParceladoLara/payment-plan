use installment::InstallmentData;
use rust_decimal::{dec, Decimal, MathematicalOps};

use crate::{
    calc::{
        inner_xirr::{eir::calculate_eir_monthly, prepare_xirr_params, tec::calculate_tec_monthly},
        PaymentPlan,
    },
    err::PaymentPlanError,
    util::get_next_business_day,
    Params, Response,
};

const POTENCY: Decimal = dec!(0.003968253968253968); // 1/252
const CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE: Decimal = dec!(0.08333333333333333); // 1/12

const NUM_OF_RUNS: u32 = 7;

mod amounts;
mod installment;
mod iof;

#[derive(Default, Debug, Clone, Copy)]
struct InnerParams {
    params: Params,
    main_value: Decimal,
    daily_interest_rate: Decimal,
    base_date: chrono::NaiveDate,
}

/**
 * This is the main implementation of the payment plan calculation.
 * It uses an iterative approach to calculate the payment plan based on the provided parameters.
 * It calculates the payment plan by iterating through the installments over and over again to better approximate the real iof value.
 * It's slower than the simple implementation, but it provides a more accurate result.
 * This is the recommended implementation for most cases.
 */
pub struct Iterative;

impl PaymentPlan for Iterative {
    fn calculate_payment_plan(
        &self,
        mut params: Params,
    ) -> Result<Vec<Response>, PaymentPlanError> {
        let base_date = params.first_payment_date;
        if params.requested_amount <= Decimal::ZERO {
            return Err(PaymentPlanError::InvalidRequestedAmount);
        }
        if params.installments == 0 {
            return Err(PaymentPlanError::InvalidNumberOfInstallments);
        }

        if params.disbursement_only_on_business_days {
            //Change the base date to the next business day
            params.disbursement_date = get_next_business_day(params.disbursement_date);
            params.first_payment_date = get_next_business_day(params.first_payment_date);
        }

        let mut response = Vec::with_capacity(params.installments as usize);

        let interest_rate = params.interest_rate;

        let min_installment_amount = params.min_installment_amount;
        let max_total_amount = params.max_total_amount;

        let annual_interest_rate = (Decimal::ONE + interest_rate).powf(12.0) - Decimal::ONE;
        let daily_interest_rate =
            (Decimal::ONE + annual_interest_rate).powd(POTENCY) - Decimal::ONE;

        let daily_interest_rate = daily_interest_rate.round_dp(10);
        let main_value = params.requested_amount;

        for i in 1..=params.installments {
            params.installments = i;
            let params = InnerParams {
                params,
                main_value,
                daily_interest_rate,
                base_date,
            };

            let resp = calc(params)?;
            if resp.installment_amount < min_installment_amount {
                break;
            }
            if resp.total_amount > max_total_amount {
                break;
            }
            response.push(resp);
        }

        Ok(response)
    }
}

fn calc(mut params: InnerParams) -> Result<Response, PaymentPlanError> {
    let debit_service_percentage = params.params.debit_service_percentage;
    let requested_amount = params.params.requested_amount;

    let mut data = installment::calc(&params);

    let mut iof = iof::calc(&params, &data);

    let debit_service = data.amount;

    params.main_value = requested_amount + iof;
    for _ in 1..NUM_OF_RUNS {
        data = installment::calc(&params);
        iof = iof::calc(&params, &data);
        params.main_value = requested_amount + iof;
    }

    let iof = iof.round_dp(2);
    let mut data = installment::calc(&params);

    let customer_amount = data.amount;

    let installment_amount = data.amount;
    let installments = params.params.installments;
    let total_amount = installment_amount * Decimal::from(installments);
    let total_amount = total_amount.round_dp(2);
    let contract_amount = params.params.requested_amount + iof;
    let accumulated_days = data.accumulated_days.pop().unwrap();
    let accumulated_days_index = data.accumulated_factor;
    let customer_debit_service_proportion =
        Decimal::ONE - Decimal::from(debit_service_percentage) / Decimal::ONE_HUNDRED;

    let params = params.params;

    let amounts = amounts::calc(
        params,
        installments.into(),
        customer_debit_service_proportion,
        iof,
        total_amount,
    );

    let (eir_params, tec_params) = prepare_xirr_params(
        installments,
        &data.due_dates,
        debit_service,
        customer_amount,
    );

    let eir_monthly = calculate_eir_monthly(
        params,
        eir_params,
        customer_debit_service_proportion,
        CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE,
    )?;

    let eir_yearly = (Decimal::ONE + eir_monthly).powf(12.0) - Decimal::ONE;

    let tec_monthly = calculate_tec_monthly(
        params,
        tec_params,
        CALCULATION_BASIS_FOR_EFFECTIVE_INTEREST_RATE,
    )?;

    let tec_yearly = (Decimal::ONE + tec_monthly).powf(12.0) - Decimal::ONE;

    let eir_monthly = eir_monthly.round_dp(4);
    let tec_monthly = tec_monthly.round_dp(4);

    let eir_yearly = eir_yearly.round_dp(6);
    let tec_yearly = tec_yearly.round_dp(6);

    let interest_rate = params.interest_rate;

    let present_value = present_value(installment_amount, &data, params.interest_rate);
    let present_value = present_value.round_dp(2);
    let pre_disbursement_amount = present_value - iof;
    let pre_disbursement_amount = pre_disbursement_amount.round_dp(2);
    let diff = pre_disbursement_amount - requested_amount;
    let diff = diff.round_dp(2);

    let paid_iof = iof + diff;
    let paid_iof = paid_iof.round_dp(2);

    let mut invoices = data.invoices;
    installment::insert_price_table_on_invoices(
        &mut invoices,
        contract_amount,
        installment_amount,
        interest_rate,
    );

    let resp = Response {
        contract_amount,
        total_amount,
        installment_amount,
        installment: installments,
        due_date: data.last_due_date,
        accumulated_days,
        interest_rate: params.interest_rate,
        iof_percentage: params.iof_percentage,
        overall_iof: params.iof_overall,
        total_iof: iof,
        days_index: data.factor,
        accumulated_days_index,
        calculation_basis_for_effective_interest_rate: amounts
            .calculation_basis_for_effective_interest_rate,
        customer_amount,
        customer_debit_service_amount: amounts.customer_debit_service_amount,
        debit_service: amounts.debit_service,
        mdr_amount: amounts.mdr_amount,
        settled_to_merchant: amounts.settled_to_merchant,
        merchant_debit_service_amount: amounts.merchant_debit_service_amount,
        merchant_total_amount: amounts.merchant_total_amount,
        eir_yearly,
        tec_yearly,
        eir_monthly,
        tec_monthly,
        effective_interest_rate: eir_monthly,
        total_effective_cost: tec_monthly,
        disbursement_date: params.disbursement_date,
        pre_disbursement_amount,
        paid_total_iof: paid_iof,
        paid_contract_amount: requested_amount + paid_iof,
        invoices,
        ..Default::default()
    };

    return Ok(resp);
}

fn present_value(
    installment_amount: Decimal,
    installments: &InstallmentData,
    interest_rate: Decimal,
) -> Decimal {
    let mut present_value = Decimal::ZERO;
    let annual_interest_rate = (Decimal::ONE + interest_rate).powf(12.0);
    for days in &installments.accumulated_business_days {
        let days = Decimal::from(*days);

        let days_diff = days / dec!(252);
        let potency = annual_interest_rate.powd(days_diff);
        let installment_value = installment_amount / potency;
        present_value += installment_value;
    }
    return present_value;
}

#[cfg(test)]
mod test {
    use chrono::Datelike;

    use super::*;
    use crate::{Invoice, Params};

    fn print_expected(resp: &Response) {
        // Print the actual response values for copying
        println!("let expected = Response {{");
        println!("    installment: {},", resp.installment);
        println!("    disbursement_date: disbursement_date,");
        println!("    due_date: expected_due_date,");
        println!("    accumulated_days: {},", resp.accumulated_days);
        println!("    days_index: {},", resp.days_index);
        println!(
            "    accumulated_days_index: {},",
            resp.accumulated_days_index
        );
        println!("    interest_rate: {},", resp.interest_rate);
        println!("    installment_amount: {},", resp.installment_amount);
        println!(
            "    installment_amount_without_tac: {},",
            resp.installment_amount_without_tac
        );
        println!("    total_amount: {},", resp.total_amount);
        println!("    debit_service: {},", resp.debit_service);
        println!(
            "    customer_debit_service_amount: {},",
            resp.customer_debit_service_amount
        );
        println!("    customer_amount: {},", resp.customer_amount);
        println!(
            "    calculation_basis_for_effective_interest_rate: {},",
            resp.calculation_basis_for_effective_interest_rate
        );
        println!(
            "    merchant_debit_service_amount: {},",
            resp.merchant_debit_service_amount
        );
        println!("    merchant_total_amount: {},", resp.merchant_total_amount);
        println!("    settled_to_merchant: {},", resp.settled_to_merchant);
        println!("    mdr_amount: {},", resp.mdr_amount);
        println!(
            "    effective_interest_rate: {},",
            resp.effective_interest_rate
        );
        println!("    total_effective_cost: {},", resp.total_effective_cost);
        println!("    eir_yearly: {},", resp.eir_yearly);
        println!("    tec_yearly: {},", resp.tec_yearly);
        println!("    eir_monthly: {},", resp.eir_monthly);
        println!("    tec_monthly: {},", resp.tec_monthly);
        println!("    total_iof: {},", resp.total_iof);
        println!("    contract_amount: {},", resp.contract_amount);
        println!(
            "    contract_amount_without_tac: {},",
            resp.contract_amount_without_tac
        );
        println!("    tac_amount: {},", resp.tac_amount);
        println!("    iof_percentage: {:e},", resp.iof_percentage);
        println!("    overall_iof: {},", resp.overall_iof);
        println!(
            "    pre_disbursement_amount: {},",
            resp.pre_disbursement_amount
        );
        println!("    paid_total_iof: {},", resp.paid_total_iof);
        println!("    paid_contract_amount: {},", resp.paid_contract_amount);
        println!("    invoices: vec![");
        for invoice in &resp.invoices {
            println!("        Invoice {{");
            println!(
                "            accumulated_days: {},",
                invoice.accumulated_days
            );
            println!("            factor: {},", invoice.factor);
            println!(
                "            accumulated_factor: {},",
                invoice.accumulated_factor
            );
            println!("            main_iof_tac: {},", invoice.main_iof_tac);
            println!("            debit_service: {},", invoice.debit_service);
            println!(
                "            due_date: chrono::NaiveDate::from_ymd_opt({}, {}, {}).unwrap(),",
                invoice.due_date.year(),
                invoice.due_date.month(),
                invoice.due_date.day()
            );
            println!("        }},");
        }
        println!("    ],");
        println!("}};");
    }

    #[test]
    fn test_iterative() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: true,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ONE_HUNDRED,
            requested_amount: dec!(12853.43),
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),      // %0.38
            iof_percentage: dec!(0.000082), // 0.0082%
            interest_rate: dec!(0.035),
        };

        let iterative = Iterative;
        let mut resp = iterative.calculate_payment_plan(params).unwrap();
        assert_eq!(resp.len(), 48);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2028, 10, 23).unwrap();

        let expected = Response {
            installment: 48,
            disbursement_date: disbursement_date,
            due_date: expected_due_date,
            accumulated_days: 1461,
            days_index: dec!(0.19275140186402),
            accumulated_days_index: dec!(23.079195526791356),
            interest_rate: dec!(0.035),
            installment_amount: dec!(575.5),
            installment_amount_without_tac: Decimal::ZERO,
            total_amount: dec!(27624.0),
            debit_service: dec!(14342.02),
            customer_debit_service_amount: dec!(14342.02),
            customer_amount: dec!(575.5),
            calculation_basis_for_effective_interest_rate: dec!(566.571875),
            merchant_debit_service_amount: Decimal::ZERO,
            merchant_total_amount: dec!(642.6715),
            settled_to_merchant: dec!(12210.7585),
            mdr_amount: dec!(642.6715),
            effective_interest_rate: dec!(0.035),
            total_effective_cost: dec!(0.0369),
            eir_yearly: dec!(0.511034),
            tec_yearly: dec!(0.544357),
            eir_monthly: dec!(0.035),
            tec_monthly: dec!(0.0369),
            total_iof: dec!(428.55),
            contract_amount: dec!(13281.98),
            contract_amount_without_tac: Decimal::ZERO,
            tac_amount: Decimal::ZERO,
            iof_percentage: dec!(8.2e-5),
            overall_iof: dec!(0.0038),
            pre_disbursement_amount: dec!(12853.53),
            paid_total_iof: dec!(428.65),
            paid_contract_amount: dec!(13282.08),
            invoices: vec![
                Invoice {
                    accumulated_days: 33,
                    factor: dec!(0.96302322215506),
                    accumulated_factor: dec!(0.96302322215506),
                    main_iof_tac: dec!(110.63069999999999),
                    debit_service: dec!(464.8693),
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 61,
                    factor: dec!(0.931982709374806),
                    accumulated_factor: dec!(1.895005931529866),
                    main_iof_tac: dec!(114.50277449999999),
                    debit_service: dec!(460.9972255),
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 92,
                    factor: dec!(0.897520991774929),
                    accumulated_factor: dec!(2.792526923304795),
                    main_iof_tac: dec!(118.51037160749996),
                    debit_service: dec!(456.98962839250004),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 1, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 124,
                    factor: dec!(0.865750637245039),
                    accumulated_factor: dec!(3.658277560549834),
                    main_iof_tac: dec!(122.65823461376243),
                    debit_service: dec!(452.84176538623757),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 2, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 152,
                    factor: dec!(0.840595006492485),
                    accumulated_factor: dec!(4.498872567042319),
                    main_iof_tac: dec!(126.95127282524413),
                    debit_service: dec!(448.54872717475587),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 3, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 182,
                    factor: dec!(0.813500644236445),
                    accumulated_factor: dec!(5.312373211278764),
                    main_iof_tac: dec!(131.3945673741277),
                    debit_service: dec!(444.1054326258723),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 4, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 212,
                    factor: dec!(0.78599096060344),
                    accumulated_factor: dec!(6.098364171882205),
                    main_iof_tac: dec!(135.99337723222214),
                    debit_service: dec!(439.50662276777786),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 5, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 243,
                    factor: dec!(0.760656615702412),
                    accumulated_factor: dec!(6.859020787584616),
                    main_iof_tac: dec!(140.7531454353499),
                    debit_service: dec!(434.7468545646501),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 273,
                    factor: dec!(0.733730972093141),
                    accumulated_factor: dec!(7.592751759677757),
                    main_iof_tac: dec!(145.67950552558716),
                    debit_service: dec!(429.82049447441284),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 7, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 306,
                    factor: dec!(0.706599964940101),
                    accumulated_factor: dec!(8.299351724617859),
                    main_iof_tac: dec!(150.77828821898277),
                    debit_service: dec!(424.72171178101723),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 8, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 335,
                    factor: dec!(0.68270528012539),
                    accumulated_factor: dec!(8.982057004743249),
                    main_iof_tac: dec!(156.05552830664715),
                    debit_service: dec!(419.44447169335285),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 9, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 365,
                    factor: dec!(0.658538949769018),
                    accumulated_factor: dec!(9.640595954512266),
                    main_iof_tac: dec!(161.5174717973798),
                    debit_service: dec!(413.9825282026202),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 397,
                    factor: dec!(0.636269516675746),
                    accumulated_factor: dec!(10.276865471188012),
                    main_iof_tac: dec!(167.17058331028812),
                    debit_service: dec!(408.3294166897119),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 426,
                    factor: dec!(0.61475315619947),
                    accumulated_factor: dec!(10.891618627387482),
                    main_iof_tac: dec!(173.02155372614823),
                    debit_service: dec!(402.4784462738518),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 12, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 457,
                    factor: dec!(0.593964402116414),
                    accumulated_factor: dec!(11.485583029503896),
                    main_iof_tac: dec!(179.0773081065634),
                    debit_service: dec!(396.4226918934366),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 488,
                    factor: dec!(0.575761946586711),
                    accumulated_factor: dec!(12.061344976090608),
                    main_iof_tac: dec!(185.3450138902931),
                    debit_service: dec!(390.1549861097069),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 2, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 516,
                    factor: dec!(0.557203779296191),
                    accumulated_factor: dec!(12.618548755386799),
                    main_iof_tac: dec!(191.83208937645338),
                    debit_service: dec!(383.6679106235466),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 547,
                    factor: dec!(0.538361139408745),
                    accumulated_factor: dec!(13.156909894795543),
                    main_iof_tac: dec!(198.5462125046293),
                    debit_service: dec!(376.9537874953707),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 4, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 579,
                    factor: dec!(0.520155690242396),
                    accumulated_factor: dec!(13.677065585037939),
                    main_iof_tac: dec!(205.49532994229128),
                    debit_service: dec!(370.0046700577087),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 608,
                    factor: dec!(0.503389843916739),
                    accumulated_factor: dec!(14.180455428954678),
                    main_iof_tac: dec!(212.68766649027145),
                    debit_service: dec!(362.81233350972855),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 6, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 638,
                    factor: dec!(0.485570902683561),
                    accumulated_factor: dec!(14.666026331638239),
                    main_iof_tac: dec!(220.13173481743098),
                    debit_service: dec!(355.368265182569),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 7, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 670,
                    factor: dec!(0.468382714475119),
                    accumulated_factor: dec!(15.134409046113358),
                    main_iof_tac: dec!(227.83634553604105),
                    debit_service: dec!(347.66365446395895),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 8, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 700,
                    factor: dec!(0.452543685476596),
                    accumulated_factor: dec!(15.586952731589955),
                    main_iof_tac: dec!(235.8106176298025),
                    debit_service: dec!(339.6893823701975),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 730,
                    factor: dec!(0.43724027581641),
                    accumulated_factor: dec!(16.024193007406364),
                    main_iof_tac: dec!(244.0639892468456),
                    debit_service: dec!(331.4360107531544),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 10, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 761,
                    factor: dec!(0.423840741016035),
                    accumulated_factor: dec!(16.4480337484224),
                    main_iof_tac: dec!(252.6062288704852),
                    debit_service: dec!(322.8937711295148),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 11, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 791,
                    factor: dec!(0.408837670636966),
                    accumulated_factor: dec!(16.856871419059367),
                    main_iof_tac: dec!(261.4474468809522),
                    debit_service: dec!(314.0525531190478),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 12, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 824,
                    factor: dec!(0.395012242155549),
                    accumulated_factor: dec!(17.251883661214915),
                    main_iof_tac: dec!(270.59810752178555),
                    debit_service: dec!(304.90189247821445),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 1, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 853,
                    factor: dec!(0.382906815052672),
                    accumulated_factor: dec!(17.634790476267586),
                    main_iof_tac: dec!(280.06904128504806),
                    debit_service: dec!(295.43095871495194),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 2, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 881,
                    factor: dec!(0.370564824109098),
                    accumulated_factor: dec!(18.005355300376685),
                    main_iof_tac: dec!(289.87145773002476),
                    debit_service: dec!(285.62854226997524),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 3, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 912,
                    factor: dec!(0.35803364647699),
                    accumulated_factor: dec!(18.363388946853675),
                    main_iof_tac: dec!(300.0169587505756),
                    debit_service: dec!(275.4830412494244),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 4, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 943,
                    factor: dec!(0.345926228475129),
                    accumulated_factor: dec!(18.709315175328804),
                    main_iof_tac: dec!(310.51755230684574),
                    debit_service: dec!(264.98244769315426),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 5, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 973,
                    factor: dec!(0.334228240067706),
                    accumulated_factor: dec!(19.04354341539651),
                    main_iof_tac: dec!(321.38566663758536),
                    debit_service: dec!(254.11433336241467),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 6, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1003,
                    factor: dec!(0.322397263658059),
                    accumulated_factor: dec!(19.365940679054567),
                    main_iof_tac: dec!(332.6341649699008),
                    debit_service: dec!(242.8658350300992),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 7, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1034,
                    factor: dec!(0.311494940727874),
                    accumulated_factor: dec!(19.677435619782443),
                    main_iof_tac: dec!(344.27636074384736),
                    debit_service: dec!(231.22363925615267),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 8, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1065,
                    factor: dec!(0.300468675279062),
                    accumulated_factor: dec!(19.977904295061506),
                    main_iof_tac: dec!(356.326033369882),
                    debit_service: dec!(219.173966630118),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 9, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1097,
                    factor: dec!(0.290307898816108),
                    accumulated_factor: dec!(20.268212193877613),
                    main_iof_tac: dec!(368.79744453782786),
                    debit_service: dec!(206.70255546217214),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 10, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 1126,
                    factor: dec!(0.281411209722801),
                    accumulated_factor: dec!(20.549623403600414),
                    main_iof_tac: dec!(381.70535509665183),
                    debit_service: dec!(193.79464490334817),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 11, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1156,
                    factor: dec!(0.27144984504887),
                    accumulated_factor: dec!(20.821073248649284),
                    main_iof_tac: dec!(395.06504252503464),
                    debit_service: dec!(180.43495747496536),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 12, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1188,
                    factor: dec!(0.261841091723523),
                    accumulated_factor: dec!(21.082914340372806),
                    main_iof_tac: dec!(408.8923190134109),
                    debit_service: dec!(166.60768098658912),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 1, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 1218,
                    factor: dec!(0.252572468046991),
                    accumulated_factor: dec!(21.335486808419798),
                    main_iof_tac: dec!(423.2035501788803),
                    debit_service: dec!(152.29644982111975),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 2, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1247,
                    factor: dec!(0.244832207685813),
                    accumulated_factor: dec!(21.580319016105612),
                    main_iof_tac: dec!(438.0156744351411),
                    debit_service: dec!(137.48432556485892),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 3, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1279,
                    factor: dec!(0.236940687422488),
                    accumulated_factor: dec!(21.8172597035281),
                    main_iof_tac: dec!(453.346223040371),
                    debit_service: dec!(122.15377695962898),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 4, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 1308,
                    factor: dec!(0.229303529494311),
                    accumulated_factor: dec!(22.04656323302241),
                    main_iof_tac: dec!(469.21334084678404),
                    debit_service: dec!(106.286659153216),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 5, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1339,
                    factor: dec!(0.221186667054601),
                    accumulated_factor: dec!(22.26774990007701),
                    main_iof_tac: dec!(485.6358077764214),
                    debit_service: dec!(89.86419222357856),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 6, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1370,
                    factor: dec!(0.213706924687313),
                    accumulated_factor: dec!(22.481456824764322),
                    main_iof_tac: dec!(502.63306104859623),
                    debit_service: dec!(72.8669389514038),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 7, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 1400,
                    factor: dec!(0.20614214923913),
                    accumulated_factor: dec!(22.687598974003453),
                    main_iof_tac: dec!(520.2252181852971),
                    debit_service: dec!(55.274781814702926),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 8, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 1433,
                    factor: dec!(0.198845150923883),
                    accumulated_factor: dec!(22.886444124927337),
                    main_iof_tac: dec!(538.4331008217824),
                    debit_service: dec!(37.066899178217525),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 9, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 1461,
                    factor: dec!(0.19275140186402),
                    accumulated_factor: dec!(23.079195526791356),
                    main_iof_tac: dec!(557.2782593505449),
                    debit_service: dec!(18.22174064945514),
                    due_date: chrono::NaiveDate::from_ymd_opt(2028, 10, 23).unwrap(),
                },
            ],
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_iterative_wrong_amount() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: true,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ONE_HUNDRED,
            requested_amount: Decimal::ZERO,
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),      // %0.38
            iof_percentage: dec!(0.000082), // 0.0082%
            interest_rate: dec!(0.035),
        };

        let iterative = Iterative;
        let resp = iterative.calculate_payment_plan(params);
        assert_eq!(resp.is_err(), true);

        assert_eq!(resp.unwrap_err(), PaymentPlanError::InvalidRequestedAmount);
    }

    #[test]
    fn test_iterative_wrong_installments() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: true,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ONE_HUNDRED,
            requested_amount: dec!(12853.43),
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 0,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),      // %0.38
            iof_percentage: dec!(0.000082), // 0.0082%
            interest_rate: dec!(0.035),
        };

        let iterative = Iterative;
        let resp = iterative.calculate_payment_plan(params);
        assert_eq!(resp.is_err(), true);

        assert_eq!(
            resp.unwrap_err(),
            PaymentPlanError::InvalidNumberOfInstallments
        );
    }

    #[test]
    fn test_iterative_min_installment_amount_reached() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: true,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ONE_HUNDRED,
            requested_amount: dec!(200.43),
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),      // %0.38
            iof_percentage: dec!(0.000082), // 0.0082%
            interest_rate: dec!(0.035),
        };

        let iterative = Iterative;

        let mut resp = iterative.calculate_payment_plan(params).unwrap();

        assert_eq!(resp.len(), 2);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap();

        let expected = Response {
            installment: 2,
            disbursement_date: disbursement_date,
            due_date: expected_due_date,
            accumulated_days: 61,
            days_index: dec!(0.931982709374806),
            accumulated_days_index: dec!(1.895005931529866),
            interest_rate: dec!(0.035),
            installment_amount: dec!(106.59),
            installment_amount_without_tac: Decimal::ZERO,
            total_amount: dec!(213.18),
            debit_service: dec!(11.2),
            customer_debit_service_amount: dec!(11.2),
            customer_amount: dec!(106.59),
            calculation_basis_for_effective_interest_rate: dec!(105.815),
            merchant_debit_service_amount: Decimal::ZERO,
            merchant_total_amount: dec!(10.021500000000001),
            settled_to_merchant: dec!(190.4085),
            mdr_amount: dec!(10.021500000000001),
            effective_interest_rate: dec!(0.0356),
            total_effective_cost: dec!(0.0408),
            eir_yearly: dec!(0.521921),
            tec_yearly: dec!(0.616492),
            eir_monthly: dec!(0.0356),
            tec_monthly: dec!(0.0408),
            total_iof: dec!(1.55),
            contract_amount: dec!(201.98000000000002),
            contract_amount_without_tac: Decimal::ZERO,
            tac_amount: Decimal::ZERO,
            iof_percentage: dec!(8.2e-5),
            overall_iof: dec!(0.0038),
            pre_disbursement_amount: dec!(200.44),
            paid_total_iof: dec!(1.56),
            paid_contract_amount: dec!(201.99),
            invoices: vec![
                Invoice {
                    accumulated_days: 33,
                    factor: dec!(0.96302322215506),
                    accumulated_factor: dec!(0.96302322215506),
                    main_iof_tac: dec!(99.5207),
                    debit_service: dec!(7.069300000000001),
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 61,
                    factor: dec!(0.931982709374806),
                    accumulated_factor: dec!(1.895005931529866),
                    main_iof_tac: dec!(103.0039245),
                    debit_service: dec!(3.5860755000000006),
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
                },
            ],
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_iterative_max_amount_reached() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 23).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2024, 11, 23).unwrap();

        let params = Params {
            disbursement_only_on_business_days: true,
            max_total_amount: dec!(2400.43),
            min_installment_amount: Decimal::ONE_HUNDRED,
            requested_amount: dec!(2000.43),
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 48,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),      // %0.38
            iof_percentage: dec!(0.000082), // 0.0082%
            interest_rate: dec!(0.035),
        };

        let iterative = Iterative;

        let mut resp = iterative.calculate_payment_plan(params).unwrap();

        assert_eq!(resp.len(), 8);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2025, 6, 23).unwrap();

        let expected = Response {
            installment: 8,
            disbursement_date: disbursement_date,
            due_date: expected_due_date,
            accumulated_days: 243,
            days_index: dec!(0.760656615702412),
            accumulated_days_index: dec!(6.859020787584616),
            interest_rate: dec!(0.035),
            installment_amount: dec!(296.26),
            installment_amount_without_tac: Decimal::ZERO,
            total_amount: dec!(2370.08),
            debit_service: dec!(338.04999999999984),
            customer_debit_service_amount: dec!(338.04999999999984),
            customer_amount: dec!(296.26),
            calculation_basis_for_effective_interest_rate: dec!(292.31),
            merchant_debit_service_amount: Decimal::ZERO,
            merchant_total_amount: dec!(100.0215),
            settled_to_merchant: dec!(1900.4085),
            mdr_amount: dec!(100.0215),
            effective_interest_rate: dec!(0.0354),
            total_effective_cost: dec!(0.0391),
            eir_yearly: dec!(0.517493),
            tec_yearly: dec!(0.58491),
            eir_monthly: dec!(0.0354),
            tec_monthly: dec!(0.0391),
            total_iof: dec!(31.6),
            contract_amount: dec!(2032.03),
            contract_amount_without_tac: Decimal::ZERO,
            tac_amount: Decimal::ZERO,
            iof_percentage: dec!(8.2e-5),
            overall_iof: dec!(0.0038),
            pre_disbursement_amount: dec!(2000.45),
            paid_total_iof: dec!(31.62),
            paid_contract_amount: dec!(2032.05),
            invoices: vec![
                Invoice {
                    accumulated_days: 33,
                    factor: dec!(0.96302322215506),
                    accumulated_factor: dec!(0.96302322215506),
                    main_iof_tac: dec!(225.13894999999997),
                    debit_service: dec!(71.12105000000001),
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 11, 25).unwrap(),
                },
                Invoice {
                    accumulated_days: 61,
                    factor: dec!(0.931982709374806),
                    accumulated_factor: dec!(1.895005931529866),
                    main_iof_tac: dec!(233.01881325),
                    debit_service: dec!(63.241186750000004),
                    due_date: chrono::NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 92,
                    factor: dec!(0.897520991774929),
                    accumulated_factor: dec!(2.792526923304795),
                    main_iof_tac: dec!(241.17447171375),
                    debit_service: dec!(55.08552828625),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 1, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 124,
                    factor: dec!(0.865750637245039),
                    accumulated_factor: dec!(3.658277560549834),
                    main_iof_tac: dec!(249.61557822373123),
                    debit_service: dec!(46.64442177626876),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 2, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 152,
                    factor: dec!(0.840595006492485),
                    accumulated_factor: dec!(4.498872567042319),
                    main_iof_tac: dec!(258.35212346156186),
                    debit_service: dec!(37.90787653843816),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 3, 24).unwrap(),
                },
                Invoice {
                    accumulated_days: 182,
                    factor: dec!(0.813500644236445),
                    accumulated_factor: dec!(5.312373211278764),
                    main_iof_tac: dec!(267.3944477827165),
                    debit_service: dec!(28.865552217283494),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 4, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 212,
                    factor: dec!(0.78599096060344),
                    accumulated_factor: dec!(6.098364171882205),
                    main_iof_tac: dec!(276.7532534551116),
                    debit_service: dec!(19.50674654488842),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 5, 23).unwrap(),
                },
                Invoice {
                    accumulated_days: 243,
                    factor: dec!(0.760656615702412),
                    accumulated_factor: dec!(6.859020787584616),
                    main_iof_tac: dec!(286.4396173260405),
                    debit_service: dec!(9.82038267395951),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 23).unwrap(),
                },
            ],
        };

        assert_eq!(resp, expected);
    }

    #[test]
    fn test_system_proposal() {
        let disbursement_date = chrono::NaiveDate::from_ymd_opt(2025, 08, 21).unwrap();

        let first_payment_date = chrono::NaiveDate::from_ymd_opt(2025, 09, 18).unwrap();

        let params = Params {
            disbursement_only_on_business_days: true,
            max_total_amount: Decimal::MAX,
            min_installment_amount: Decimal::ONE_HUNDRED,
            requested_amount: dec!(3883.48),
            first_payment_date,
            disbursement_date: disbursement_date,
            installments: 24,
            debit_service_percentage: 0,
            mdr: dec!(0.05),
            tac_percentage: Decimal::ZERO,
            iof_overall: dec!(0.0038),      // %0.38
            iof_percentage: dec!(0.000082), // 0.0082%
            interest_rate: dec!(0.0449),
        };

        let iterative = Iterative;

        let mut resp = iterative.calculate_payment_plan(params).unwrap();

        assert_eq!(resp.len(), 24);

        let resp = resp.pop().unwrap();

        let expected_due_date = chrono::NaiveDate::from_ymd_opt(2027, 8, 18).unwrap();

        let expected = Response {
            installment: 24,
            disbursement_date: disbursement_date,
            due_date: expected_due_date,
            accumulated_days: 727,
            days_index: dec!(0.352166545526241),
            accumulated_days_index: dec!(14.596086465727566),
            interest_rate: dec!(0.0449),
            installment_amount: dec!(274.01),
            installment_amount_without_tac: Decimal::ZERO,
            total_amount: dec!(6576.24),
            debit_service: dec!(2576.72),
            customer_debit_service_amount: dec!(2576.72),
            customer_amount: dec!(274.01),
            calculation_basis_for_effective_interest_rate: dec!(269.175),
            merchant_debit_service_amount: Decimal::ZERO,
            merchant_total_amount: dec!(194.174),
            settled_to_merchant: dec!(3689.306),
            mdr_amount: dec!(194.174),
            effective_interest_rate: dec!(0.0446),
            total_effective_cost: dec!(0.0476),
            eir_yearly: dec!(0.689008),
            tec_yearly: dec!(0.74792),
            eir_monthly: dec!(0.0446),
            tec_monthly: dec!(0.0476),
            total_iof: dec!(116.04),
            contract_amount: dec!(3999.52),
            contract_amount_without_tac: Decimal::ZERO,
            tac_amount: Decimal::ZERO,
            iof_percentage: dec!(8.2e-5),
            overall_iof: dec!(0.0038),
            pre_disbursement_amount: dec!(3883.43),
            paid_total_iof: dec!(115.99),
            paid_contract_amount: dec!(3999.47),
            invoices: vec![
                Invoice {
                    accumulated_days: 28,
                    factor: dec!(0.959033087711361),
                    accumulated_factor: dec!(0.959033087711361),
                    main_iof_tac: dec!(94.43155199999998),
                    debit_service: dec!(179.578448),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 9, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 60,
                    factor: dec!(0.915905234894921),
                    accumulated_factor: dec!(1.874938322606282),
                    main_iof_tac: dec!(98.6715286848),
                    debit_service: dec!(175.3384713152),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 10, 20).unwrap(),
                },
                Invoice {
                    accumulated_days: 89,
                    factor: dec!(0.876548219430817),
                    accumulated_factor: dec!(2.7514865420370986),
                    main_iof_tac: dec!(103.10188032274749),
                    debit_service: dec!(170.9081196772525),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 11, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 119,
                    factor: dec!(0.83888239930792),
                    accumulated_factor: dec!(3.5903689413450186),
                    main_iof_tac: dec!(107.73115474923887),
                    debit_service: dec!(166.27884525076112),
                    due_date: chrono::NaiveDate::from_ymd_opt(2025, 12, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 151,
                    factor: dec!(0.80451597763499),
                    accumulated_factor: dec!(4.394884918980009),
                    main_iof_tac: dec!(112.5682835974797),
                    debit_service: dec!(161.44171640252029),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 19).unwrap(),
                },
                Invoice {
                    accumulated_days: 181,
                    factor: dec!(0.771557442144409),
                    accumulated_factor: dec!(5.1664423611244175),
                    main_iof_tac: dec!(117.62259953100653),
                    debit_service: dec!(156.38740046899346),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 2, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 209,
                    factor: dec!(0.739949116086432),
                    accumulated_factor: dec!(5.90639147721085),
                    main_iof_tac: dec!(122.90385424994872),
                    debit_service: dec!(151.10614575005127),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 242,
                    factor: dec!(0.706673500282199),
                    accumulated_factor: dec!(6.613064977493049),
                    main_iof_tac: dec!(128.42223730577143),
                    debit_service: dec!(145.58776269422856),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 4, 20).unwrap(),
                },
                Invoice {
                    accumulated_days: 270,
                    factor: dec!(0.680564102662863),
                    accumulated_factor: dec!(7.293629080155911),
                    main_iof_tac: dec!(134.18839576080057),
                    debit_service: dec!(139.82160423919942),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 301,
                    factor: dec!(0.649959039263183),
                    accumulated_factor: dec!(7.943588119419094),
                    main_iof_tac: dec!(140.2134547304605),
                    debit_service: dec!(133.79654526953948),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 6, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 333,
                    factor: dec!(0.620730289868361),
                    accumulated_factor: dec!(8.564318409287456),
                    main_iof_tac: dec!(146.5090388478582),
                    debit_service: dec!(127.5009611521418),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 7, 20).unwrap(),
                },
                Invoice {
                    accumulated_days: 362,
                    factor: dec!(0.594057124690754),
                    accumulated_factor: dec!(9.158375533978209),
                    main_iof_tac: dec!(153.08729469212705),
                    debit_service: dec!(120.92270530787296),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 8, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 393,
                    factor: dec!(0.567342292255346),
                    accumulated_factor: dec!(9.725717826233554),
                    main_iof_tac: dec!(159.96091422380354),
                    debit_service: dec!(114.04908577619645),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 424,
                    factor: dec!(0.544100030330886),
                    accumulated_factor: dec!(10.26981785656444),
                    main_iof_tac: dec!(167.14315927245235),
                    debit_service: dec!(106.86684072754765),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 10, 19).unwrap(),
                },
                Invoice {
                    accumulated_days: 454,
                    factor: dec!(0.520719714887871),
                    accumulated_factor: dec!(10.790537571452312),
                    main_iof_tac: dec!(174.64788712378544),
                    debit_service: dec!(99.36211287621454),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 11, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 484,
                    factor: dec!(0.498344066086544),
                    accumulated_factor: dec!(11.288881637538855),
                    main_iof_tac: dec!(182.48957725564344),
                    debit_service: dec!(91.52042274435657),
                    due_date: chrono::NaiveDate::from_ymd_opt(2026, 12, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 515,
                    factor: dec!(0.47892907471239),
                    accumulated_factor: dec!(11.767810712251245),
                    main_iof_tac: dec!(190.6833592744218),
                    debit_service: dec!(83.32664072557819),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 1, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 546,
                    factor: dec!(0.45834919561406),
                    accumulated_factor: dec!(12.226159907865306),
                    main_iof_tac: dec!(199.24504210584334),
                    debit_service: dec!(74.76495789415665),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 2, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 574,
                    factor: dec!(0.439572044319771),
                    accumulated_factor: dec!(12.665731952185077),
                    main_iof_tac: dec!(208.1911444963957),
                    debit_service: dec!(65.81885550360428),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 3, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 606,
                    factor: dec!(0.420683361204136),
                    accumulated_factor: dec!(13.086415313389212),
                    main_iof_tac: dec!(217.53892688428388),
                    debit_service: dec!(56.47107311571611),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 4, 19).unwrap(),
                },
                Invoice {
                    accumulated_days: 635,
                    factor: dec!(0.403449262844397),
                    accumulated_factor: dec!(13.48986457623361),
                    main_iof_tac: dec!(227.3064247013882),
                    debit_service: dec!(46.70357529861177),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 5, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 666,
                    factor: dec!(0.385306092760059),
                    accumulated_factor: dec!(13.875170668993668),
                    main_iof_tac: dec!(237.51248317048055),
                    debit_service: dec!(36.497516829519434),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 6, 18).unwrap(),
                },
                Invoice {
                    accumulated_days: 697,
                    factor: dec!(0.368749251207657),
                    accumulated_factor: dec!(14.243919920201325),
                    main_iof_tac: dec!(248.17679366483515),
                    debit_service: dec!(25.833206335164856),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 7, 19).unwrap(),
                },
                Invoice {
                    accumulated_days: 727,
                    factor: dec!(0.352166545526241),
                    accumulated_factor: dec!(14.596086465727566),
                    main_iof_tac: dec!(259.31993170038623),
                    debit_service: dec!(14.690068299613758),
                    due_date: chrono::NaiveDate::from_ymd_opt(2027, 8, 18).unwrap(),
                },
            ],
        };

        assert_eq!(resp, expected);
    }
}
