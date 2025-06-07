import { test } from 'node:test';
import * as assert from 'node:assert';
import { calculatePlan } from '../src/index.js';

/**
 * @typedef {import('../types/src').PaymentPlanParams} PaymentPlanParams
 * @typedef {import('../types/src').PaymentPlanResponse} PaymentPlanResponse
 */
test('calculate payment plan test 0', () => {
  /**
   * @type {PaymentPlanParams}
   */
  const params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date('2022-04-18'),
    disbursementDate: new Date('2022-03-18'),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
    disbursementOnlyOnBusinessDays: false,
  };

  const result = calculatePlan(params);
  const pop = result.pop();
  /**
   * @type {PaymentPlanResponse}
   */
  const expected = {
    installment: 24,
    dueDate: new Date('2024-03-18T03:00:00.000Z'),
    disbursementDate: new Date('2022-03-18T03:00:00.000Z'),
    accumulatedDays: 731,
    daysIndex: 0.445499118983074,
    accumulatedDaysIndex: 16.17294462287348,
    interestRate: 0.0235,
    installmentAmount: 560.19,
    installmentAmountWithoutTAC: 0,
    totalAmount: 13444.56,
    debitService: 4384.599999999999,
    customerDebitServiceAmount: 4384.599999999999,
    customerAmount: 560.19,
    calculationBasisForEffectiveInterestRate: 549.3583333333332,
    merchantDebitServiceAmount: 0,
    merchantTotalAmount: 440,
    settledToMerchant: 8360,
    mdrAmount: 440,
    effectiveInterestRate: 0.0342,
    totalEffectiveCost: 0.037,
    eirYearly: 0.497399,
    tecYearly: 0.546272,
    eirMonthly: 0.0342,
    tecMonthly: 0.037,
    totalIOF: 259.96,
    contractAmount: 9059.96,
    contractAmountWithoutTAC: 0,
    tacAmount: 0,
    IOFPercentage: 0.000082,
    overallIOF: 0.0038,
    preDisbursementAmount: 8799.96,
    paidTotalIOF: 259.92,
    paidContractAmount: 9059.92,
    installments: [
      {
        accumulated_days: 31,
        factor: 0.966292071927668,
        due_date: new Date('2022-04-18T03:00:00.000Z'),
        accumulated_factor: 0.966292071927668,
      },
      {
        accumulated_days: 61,
        factor: 0.93475372892694,
        due_date: new Date('2022-05-18T03:00:00.000Z'),
        accumulated_factor: 1.9010458008546078,
      },
      {
        accumulated_days: 92,
        factor: 0.903245117466927,
        due_date: new Date('2022-06-18T03:00:00.000Z'),
        accumulated_factor: 2.8042909183215348,
      },
      {
        accumulated_days: 122,
        factor: 0.873764533742819,
        due_date: new Date('2022-07-18T03:00:00.000Z'),
        accumulated_factor: 3.6780554520643536,
      },
      {
        accumulated_days: 153,
        factor: 0.844311741687262,
        due_date: new Date('2022-08-18T03:00:00.000Z'),
        accumulated_factor: 4.522367193751616,
      },
      {
        accumulated_days: 184,
        factor: 0.815851742227842,
        due_date: new Date('2022-09-18T03:00:00.000Z'),
        accumulated_factor: 5.338218935979458,
      },
      {
        accumulated_days: 214,
        factor: 0.789223548918967,
        due_date: new Date('2022-10-18T03:00:00.000Z'),
        accumulated_factor: 6.127442484898425,
      },
      {
        accumulated_days: 245,
        factor: 0.762620458299016,
        due_date: new Date('2022-11-18T03:00:00.000Z'),
        accumulated_factor: 6.8900629431974405,
      },
      {
        accumulated_days: 275,
        factor: 0.737729655308957,
        due_date: new Date('2022-12-18T03:00:00.000Z'),
        accumulated_factor: 7.627792598506398,
      },
      {
        accumulated_days: 306,
        factor: 0.712862317150977,
        due_date: new Date('2023-01-18T03:00:00.000Z'),
        accumulated_factor: 8.340654915657375,
      },
      {
        accumulated_days: 337,
        factor: 0.688833205438976,
        due_date: new Date('2023-02-18T03:00:00.000Z'),
        accumulated_factor: 9.029488121096351,
      },
      {
        accumulated_days: 365,
        factor: 0.667826443575455,
        due_date: new Date('2023-03-18T03:00:00.000Z'),
        accumulated_factor: 9.697314564671807,
      },
      {
        accumulated_days: 396,
        factor: 0.645315397850613,
        due_date: new Date('2023-04-18T03:00:00.000Z'),
        accumulated_factor: 10.34262996252242,
      },
      {
        accumulated_days: 426,
        factor: 0.624253258408174,
        due_date: new Date('2023-05-18T03:00:00.000Z'),
        accumulated_factor: 10.966883220930594,
      },
      {
        accumulated_days: 457,
        factor: 0.603210974474832,
        due_date: new Date('2023-06-18T03:00:00.000Z'),
        accumulated_factor: 11.570094195405426,
      },
      {
        accumulated_days: 487,
        factor: 0.583523061091833,
        due_date: new Date('2023-07-18T03:00:00.000Z'),
        accumulated_factor: 12.15361725649726,
      },
      {
        accumulated_days: 518,
        factor: 0.563853707720002,
        due_date: new Date('2023-08-18T03:00:00.000Z'),
        accumulated_factor: 12.717470964217261,
      },
      {
        accumulated_days: 549,
        factor: 0.544847367496859,
        due_date: new Date('2023-09-18T03:00:00.000Z'),
        accumulated_factor: 13.26231833171412,
      },
      {
        accumulated_days: 579,
        factor: 0.527064355860553,
        due_date: new Date('2023-10-18T03:00:00.000Z'),
        accumulated_factor: 13.789382687574673,
      },
      {
        accumulated_days: 610,
        factor: 0.509298108463716,
        due_date: new Date('2023-11-18T03:00:00.000Z'),
        accumulated_factor: 14.298680796038388,
      },
      {
        accumulated_days: 640,
        factor: 0.492675372025128,
        due_date: new Date('2023-12-18T03:00:00.000Z'),
        accumulated_factor: 14.791356168063515,
      },
      {
        accumulated_days: 671,
        factor: 0.476068306021895,
        due_date: new Date('2024-01-18T03:00:00.000Z'),
        accumulated_factor: 15.267424474085411,
      },
      {
        accumulated_days: 702,
        factor: 0.460021029804993,
        due_date: new Date('2024-02-18T03:00:00.000Z'),
        accumulated_factor: 15.727445503890404,
      },
      {
        accumulated_days: 731,
        factor: 0.445499118983074,
        due_date: new Date('2024-03-18T03:00:00.000Z'),
        accumulated_factor: 16.17294462287348,
      },
    ],
  };
  assert.deepEqual(pop, expected);
});

test('calculate payment plan test 1', () => {
  /**
   * @type {PaymentPlanParams}
   */
  const params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date('2022-04-18'),
    disbursementDate: new Date('2022-03-18'),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
    disbursementOnlyOnBusinessDays: true,
  };

  const result = calculatePlan(params);
  const pop = result.pop();
  /**
   * @type {PaymentPlanResponse}
   */
  const expected = {
    installment: 24,
    dueDate: new Date('2024-03-18T03:00:00.000Z'),
    disbursementDate: new Date('2022-03-18T03:00:00.000Z'),
    accumulatedDays: 731,
    daysIndex: 0.561985770761778,
    accumulatedDaysIndex: 18.014166849381613,
    interestRate: 0.0235,
    installmentAmount: 502.69,
    installmentAmountWithoutTAC: 0,
    totalAmount: 12064.56,
    debitService: 3009.0499999999993,
    customerDebitServiceAmount: 3009.0499999999993,
    customerAmount: 502.69,
    calculationBasisForEffectiveInterestRate: 492.04375,
    merchantDebitServiceAmount: 0,
    merchantTotalAmount: 440,
    settledToMerchant: 8360,
    mdrAmount: 440,
    effectiveInterestRate: 0.0242,
    totalEffectiveCost: 0.0268,
    eirYearly: 0.332954,
    tecYearly: 0.373929,
    eirMonthly: 0.0242,
    tecMonthly: 0.0268,
    totalIOF: 255.51,
    contractAmount: 9055.51,
    contractAmountWithoutTAC: 0,
    tacAmount: 0,
    IOFPercentage: 0.000082,
    overallIOF: 0.0038,
    preDisbursementAmount: 8800.03,
    paidTotalIOF: 255.54,
    paidContractAmount: 9055.54,
    installments: [
      {
        accumulated_days: 31,
        factor: 0.977039570089471,
        due_date: new Date('2022-04-18T03:00:00.000Z'),
        accumulated_factor: 0.977039570089471,
      },
      {
        accumulated_days: 61,
        factor: 0.953551014026522,
        due_date: new Date('2022-05-18T03:00:00.000Z'),
        accumulated_factor: 1.930590584115993,
      },
      {
        accumulated_days: 94,
        factor: 0.929598336717808,
        due_date: new Date('2022-06-20T03:00:00.000Z'),
        accumulated_factor: 2.860188920833801,
      },
      {
        accumulated_days: 122,
        factor: 0.909259536351008,
        due_date: new Date('2022-07-18T03:00:00.000Z'),
        accumulated_factor: 3.7694484571848093,
      },
      {
        accumulated_days: 153,
        factor: 0.886419436614634,
        due_date: new Date('2022-08-18T03:00:00.000Z'),
        accumulated_factor: 4.655867893799443,
      },
      {
        accumulated_days: 185,
        factor: 0.86510943723528,
        due_date: new Date('2022-09-19T03:00:00.000Z'),
        accumulated_factor: 5.520977331034723,
      },
      {
        accumulated_days: 214,
        factor: 0.845246152636702,
        due_date: new Date('2022-10-18T03:00:00.000Z'),
        accumulated_factor: 6.366223483671425,
      },
      {
        accumulated_days: 245,
        factor: 0.8240140339113,
        due_date: new Date('2022-11-18T03:00:00.000Z'),
        accumulated_factor: 7.190237517582725,
      },
      {
        accumulated_days: 276,
        factor: 0.805094317440387,
        due_date: new Date('2022-12-19T03:00:00.000Z'),
        accumulated_factor: 7.995331835023112,
      },
      {
        accumulated_days: 306,
        factor: 0.785739417608204,
        due_date: new Date('2023-01-18T03:00:00.000Z'),
        accumulated_factor: 8.781071252631316,
      },
      {
        accumulated_days: 339,
        factor: 0.766002075356075,
        due_date: new Date('2023-02-20T03:00:00.000Z'),
        accumulated_factor: 9.547073327987391,
      },
      {
        accumulated_days: 367,
        factor: 0.749242618420912,
        due_date: new Date('2023-03-20T03:00:00.000Z'),
        accumulated_factor: 10.296315946408303,
      },
      {
        accumulated_days: 396,
        factor: 0.732039685794677,
        due_date: new Date('2023-04-18T03:00:00.000Z'),
        accumulated_factor: 11.02835563220298,
      },
      {
        accumulated_days: 426,
        factor: 0.714441058547147,
        due_date: new Date('2023-05-18T03:00:00.000Z'),
        accumulated_factor: 11.742796690750126,
      },
      {
        accumulated_days: 458,
        factor: 0.697265511751411,
        due_date: new Date('2023-06-19T03:00:00.000Z'),
        accumulated_factor: 12.440062202501537,
      },
      {
        accumulated_days: 487,
        factor: 0.681255995839813,
        due_date: new Date('2023-07-18T03:00:00.000Z'),
        accumulated_factor: 13.12131819834135,
      },
      {
        accumulated_days: 518,
        factor: 0.664143219708338,
        due_date: new Date('2023-08-18T03:00:00.000Z'),
        accumulated_factor: 13.785461418049687,
      },
      {
        accumulated_days: 549,
        factor: 0.648894205861671,
        due_date: new Date('2023-09-18T03:00:00.000Z'),
        accumulated_factor: 14.434355623911358,
      },
      {
        accumulated_days: 579,
        factor: 0.633294440611724,
        due_date: new Date('2023-10-18T03:00:00.000Z'),
        accumulated_factor: 15.067650064523082,
      },
      {
        accumulated_days: 612,
        factor: 0.61738643238328,
        due_date: new Date('2023-11-20T03:00:00.000Z'),
        accumulated_factor: 15.685036496906362,
      },
      {
        accumulated_days: 640,
        factor: 0.603878556022668,
        due_date: new Date('2023-12-18T03:00:00.000Z'),
        accumulated_factor: 16.28891505292903,
      },
      {
        accumulated_days: 671,
        factor: 0.588709458645293,
        due_date: new Date('2024-01-18T03:00:00.000Z'),
        accumulated_factor: 16.877624511574325,
      },
      {
        accumulated_days: 703,
        factor: 0.574556567045507,
        due_date: new Date('2024-02-19T03:00:00.000Z'),
        accumulated_factor: 17.452181078619834,
      },
      {
        accumulated_days: 731,
        factor: 0.561985770761778,
        due_date: new Date('2024-03-18T03:00:00.000Z'),
        accumulated_factor: 18.014166849381613,
      },
    ],
  };
  assert.deepEqual(pop, expected);
});

test('Error: invalid requestedAmount', () => {
  /**
   * @type {PaymentPlanParams}
   */
  const params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: -8800.0,
    firstPaymentDate: new Date('2022-04-18'),
    disbursementDate: new Date('2022-03-18'),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
  };

  assert.throws(() => calculatePlan(params));
});

test('Error: invalid installments', () => {
  /**
   * @type {PaymentPlanParams}
   */
  const params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date('2022-04-18'),
    disbursementDate: new Date('2022-03-18'),
    installments: 0,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
  };

  assert.throws(() => calculatePlan(params));
});
