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
    dueDate: new Date("2024-03-18T03:00:00.000Z"),
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
    disbursementDate: new Date("2022-03-18T03:00:00.000Z"),
    paidTotalIOF: 259.92,
    paidContractAmount: 9059.92,
    preDisbursementAmount: 8799.96,
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
