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
    iofPercentage: 0.03,
    interestRate: 0.0235,
  };

  assert.throws(() => calculatePlan(params)); //TODO: qitech was not implemented yet
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
    iofPercentage: 0.03,
    interestRate: 0.0235,
  };

  assert.throws(() => calculatePlan(params)); //TODO: qitech was not implemented yet
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
    iofPercentage: 0.03,
    interestRate: 0.0235,
  };

  assert.throws(() => calculatePlan(params)); //TODO: qitech was not implemented yet
});
