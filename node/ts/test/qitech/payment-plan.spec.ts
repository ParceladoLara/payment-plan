import { test } from 'node:test';
import * as assert from 'node:assert';
import { calculatePlan, PaymentPlanParams } from 'ts/payment-plan';

test('calculate payment plan test 0', () => {
  const params: PaymentPlanParams = {
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
  const expected = {
    installment: 24,
    dueDate: new Date('2024-03-18T03:00:00.000Z'),
    accumulatedDays: 731,
    daysIndex: 0.572214390057081,
    accumulatedDaysIndex: 18.17043451706523,
    interestRate: 0.0235,
    installmentAmount: 498.34,
    installmentAmountWithoutTAC: 0,
    totalAmount: 11960.16,
    debitService: 2905.0699999999997,
    customerDebitServiceAmount: 2905.0699999999997,
    customerAmount: 498.34,
    calculationBasisForEffectiveInterestRate: 487.71125,
    merchantDebitServiceAmount: 0,
    merchantTotalAmount: 440,
    settledToMerchant: 8360,
    mdrAmount: 440,
    effectiveInterestRate: 0.0235,
    totalEffectiveCost: 0.0261,
    eirYearly: 0.321453,
    tecYearly: 0.36193,
    eirMonthly: 0.0235,
    tecMonthly: 0.0261,
    totalIOF: 255.09,
    contractAmount: 9055.09,
    contractAmountWithoutTAC: 0,
    tacAmount: 0,
    iofPercentage: 0.000082,
    overallIOF: 0.0038,
  };

  assert.deepEqual(pop, expected);
});

test('Error: invalid requestedAmount', () => {
  const params: PaymentPlanParams = {
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
  const params: PaymentPlanParams = {
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
