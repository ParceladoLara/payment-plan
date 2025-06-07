import { test } from "node:test";
import * as assert from "node:assert";
import {
  calculatePaymentPlan,
  Params,
  PaymentPlanResponse,
} from "../../pkg/wasm_payment_plan";

test("calculate payment plan test 0", () => {
  const params: Params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date("2022-04-18"),
    disbursementDate: new Date("2022-03-18"),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
    disbursementOnlyOnBusinessDays: false,
  };

  const result = calculatePaymentPlan(params);
  const pop = result.pop();
  const expected: PaymentPlanResponse = {
    installment: 24,
    dueDate: new Date("2024-03-18T06:00:00.000Z"),
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
    disbursementDate: new Date("2022-03-18T06:00:00.000Z"),
    paidTotalIOF: 259.92,
    paidContractAmount: 9059.92,
    preDisbursementAmount: 8799.96,
    invoices: [
      {
        accumulatedDays: 31,
        factor: 0.966292071927668,
        accumulatedFactor: 0.966292071927668,
        dueDate: new Date("2022-04-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 61,
        factor: 0.93475372892694,
        accumulatedFactor: 1.9010458008546078,
        dueDate: new Date("2022-05-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 92,
        factor: 0.903245117466927,
        accumulatedFactor: 2.8042909183215348,
        dueDate: new Date("2022-06-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 122,
        factor: 0.873764533742819,
        accumulatedFactor: 3.6780554520643536,
        dueDate: new Date("2022-07-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 153,
        factor: 0.844311741687262,
        accumulatedFactor: 4.522367193751616,
        dueDate: new Date("2022-08-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 184,
        factor: 0.815851742227842,
        accumulatedFactor: 5.338218935979458,
        dueDate: new Date("2022-09-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 214,
        factor: 0.789223548918967,
        accumulatedFactor: 6.127442484898425,
        dueDate: new Date("2022-10-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 245,
        factor: 0.762620458299016,
        accumulatedFactor: 6.8900629431974405,
        dueDate: new Date("2022-11-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 275,
        factor: 0.737729655308957,
        accumulatedFactor: 7.627792598506398,
        dueDate: new Date("2022-12-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 306,
        factor: 0.712862317150977,
        accumulatedFactor: 8.340654915657375,
        dueDate: new Date("2023-01-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 337,
        factor: 0.688833205438976,
        accumulatedFactor: 9.029488121096351,
        dueDate: new Date("2023-02-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 365,
        factor: 0.667826443575456,
        accumulatedFactor: 9.697314564671807,
        dueDate: new Date("2023-03-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 396,
        factor: 0.645315397850613,
        accumulatedFactor: 10.34262996252242,
        dueDate: new Date("2023-04-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 426,
        factor: 0.624253258408174,
        accumulatedFactor: 10.966883220930594,
        dueDate: new Date("2023-05-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 457,
        factor: 0.603210974474832,
        accumulatedFactor: 11.570094195405426,
        dueDate: new Date("2023-06-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 487,
        factor: 0.583523061091833,
        accumulatedFactor: 12.15361725649726,
        dueDate: new Date("2023-07-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 518,
        factor: 0.563853707720002,
        accumulatedFactor: 12.717470964217261,
        dueDate: new Date("2023-08-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 549,
        factor: 0.544847367496859,
        accumulatedFactor: 13.26231833171412,
        dueDate: new Date("2023-09-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 579,
        factor: 0.527064355860553,
        accumulatedFactor: 13.789382687574673,
        dueDate: new Date("2023-10-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 610,
        factor: 0.509298108463716,
        accumulatedFactor: 14.298680796038388,
        dueDate: new Date("2023-11-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 640,
        factor: 0.492675372025128,
        accumulatedFactor: 14.791356168063515,
        dueDate: new Date("2023-12-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 671,
        factor: 0.476068306021895,
        accumulatedFactor: 15.267424474085411,
        dueDate: new Date("2024-01-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 702,
        factor: 0.460021029804993,
        accumulatedFactor: 15.727445503890404,
        dueDate: new Date("2024-02-18T06:00:00.000Z"),
      },
      {
        accumulatedDays: 731,
        factor: 0.445499118983074,
        accumulatedFactor: 16.17294462287348,
        dueDate: new Date("2024-03-18T06:00:00.000Z"),
      },
    ],
  };

  assert.equal(pop.installment, expected.installment);
  assert.deepEqual(pop.dueDate, expected.dueDate);
  assert.equal(pop.accumulatedDays, expected.accumulatedDays);
  assert.equal(pop.daysIndex, expected.daysIndex);
  assert.equal(pop.accumulatedDaysIndex, expected.accumulatedDaysIndex);
  assert.equal(pop.interestRate, expected.interestRate);
  assert.equal(pop.installmentAmount, expected.installmentAmount);
  assert.equal(
    pop.installmentAmountWithoutTAC,
    expected.installmentAmountWithoutTAC
  );
  assert.equal(pop.totalAmount, expected.totalAmount);
  assert.equal(pop.debitService, expected.debitService);
  assert.equal(
    pop.customerDebitServiceAmount,
    expected.customerDebitServiceAmount
  );
  assert.equal(pop.customerAmount, expected.customerAmount);
  assert.equal(
    pop.calculationBasisForEffectiveInterestRate,
    expected.calculationBasisForEffectiveInterestRate
  );
  assert.equal(
    pop.merchantDebitServiceAmount,
    expected.merchantDebitServiceAmount
  );
  assert.equal(pop.merchantTotalAmount, expected.merchantTotalAmount);
  assert.equal(pop.settledToMerchant, expected.settledToMerchant);
  assert.equal(pop.mdrAmount, expected.mdrAmount);
  assert.equal(pop.effectiveInterestRate, expected.effectiveInterestRate);
  assert.equal(pop.totalEffectiveCost, expected.totalEffectiveCost);
  assert.equal(pop.eirYearly, expected.eirYearly);
  assert.equal(pop.tecYearly, expected.tecYearly);
  assert.equal(pop.eirMonthly, expected.eirMonthly);
  assert.equal(pop.tecMonthly, expected.tecMonthly);
  assert.equal(pop.totalIOF, expected.totalIOF);
  assert.equal(pop.contractAmount, expected.contractAmount);
  assert.equal(pop.contractAmountWithoutTAC, expected.contractAmountWithoutTAC);
  assert.equal(pop.tacAmount, expected.tacAmount);
  assert.equal(pop.IOFPercentage, expected.IOFPercentage);
  assert.equal(pop.overallIOF, expected.overallIOF);
  assert.deepEqual(pop.disbursementDate, expected.disbursementDate);
  assert.deepEqual(pop.invoices, expected.invoices);
});

test("Error: invalid requestedAmount", () => {
  const params: Params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: -8800.0,
    firstPaymentDate: new Date("2022-04-18"),
    disbursementDate: new Date("2022-03-18"),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
    disbursementOnlyOnBusinessDays: false,
  };

  assert.throws(() => calculatePaymentPlan(params));
});

test("Error: invalid installments", () => {
  const params: Params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date("2022-04-18"),
    disbursementDate: new Date("2022-03-18"),
    installments: 0,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
    disbursementOnlyOnBusinessDays: false,
  };

  assert.throws(() => calculatePaymentPlan(params));
});
