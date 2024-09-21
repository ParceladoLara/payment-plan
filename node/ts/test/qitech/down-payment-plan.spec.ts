import { test } from 'node:test';
import * as assert from 'node:assert';
import {
  calculateDownPaymentPlan,
  DownPaymentPlanParams,
  PaymentPlanParams,
} from 'ts/payment-plan';

test('calculate down payment plan 2 installments', () => {
  const planParam: PaymentPlanParams = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 1000.0,
    firstPaymentDate: new Date('2022-06-20'),
    requestedDate: new Date('2022-05-20'),
    installments: 1,
    debitServicePercentage: 0,
    mdr: 0.01,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.03,
    interestRate: 0.0355,
  };
  const downPayment = 200.0;
  const minInstallmentAmount = 100.0;
  const installments = 4;

  const params: DownPaymentPlanParams = {
    params: planParam,
    requestedAmount: downPayment,
    minInstallmentAmount,
    installments,
    firstPaymentDate: new Date('2022-06-20'),
  };

  assert.throws(() => calculateDownPaymentPlan(params)); //TODO: qitech was not implemented yet
});
