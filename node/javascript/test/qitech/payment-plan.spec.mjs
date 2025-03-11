import { test } from 'node:test';
import * as assert from 'node:assert';
import { calculatePlan } from '../../src/index.js';

test('calculate payment plan test 0', () => {
  /**
   * @type {PaymentPlanParams}
   */
  const params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date('2022-04-18'),
    requestedDate: new Date('2022-03-18'),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
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
    requestedDate: new Date('2022-03-18'),
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
    requestedDate: new Date('2022-03-18'),
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
