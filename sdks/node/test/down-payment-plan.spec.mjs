import { test } from 'node:test';
import * as assert from 'node:assert';
import { calculateDownPaymentPlan } from '../src/index.js';

test('calculate down payment plan 2 installments', () => {
  /**
   * @type {PaymentPlanParams}
   */
  const planParam = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 1000.0,
    firstPaymentDate: new Date('2022-06-20'),
    disbursementDate: new Date('2022-05-20'),
    installments: 1,
    debitServicePercentage: 0,
    mdr: 0.01,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0355,
  };
  const downPayment = 200.0;
  const minInstallmentAmount = 100.0;
  const installments = 4;

  /**
   * @type {DownPaymentPlanParams}
   */
  const params = {
    params: planParam,
    requestedAmount: downPayment,
    minInstallmentAmount,
    installments,
    firstPaymentDate: new Date('2022-06-20'),
  };

  const result = calculateDownPaymentPlan(params);

  assert.equal(result.length, 2);

  const response = result[0];

  assert.equal(response.installmentAmount, downPayment);

  const plans = response.plans;
  const firstPlan = plans[0];

  // if the first payment is 20/06/2022, the first plan should be 20/07/2022 because we have 1 down payment to pay
  const planDueDate = new Date('2022-07-20T03:00:00.000Z'); //Node is a joke

  assert.deepEqual(firstPlan.dueDate, planDueDate);

  const response2 = result[1];

  assert.equal(response2.installmentAmount, minInstallmentAmount);

  const plans2 = response2.plans;
  const firstPlan2 = plans2[0];
  // if the first payment is 20/06/2022, the first plan should be 20/08/2022 because we have 2 down payments to pay
  const planDueDate2 = new Date('2022-08-20T03:00:00.000Z'); //Node is a joke

  assert.deepEqual(firstPlan2.dueDate, planDueDate2);
});

test('calculate down payment plan 2 installments no max provided', () => {
  /**
   * @type {PaymentPlanParams}
   */
  const planParam = {
    minInstallmentAmount: 0.0,
    requestedAmount: 1000.0,
    firstPaymentDate: new Date('2022-06-20'),
    disbursementDate: new Date('2022-05-20'),
    installments: 1,
    debitServicePercentage: 0,
    mdr: 0.01,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0355,
  };
  const downPayment = 200.0;
  const minInstallmentAmount = 100.0;
  const installments = 4;

  /**
   * @type {DownPaymentPlanParams}
   */
  const params = {
    params: planParam,
    requestedAmount: downPayment,
    minInstallmentAmount,
    installments,
    firstPaymentDate: new Date('2022-06-20'),
  };

  const result = calculateDownPaymentPlan(params);

  assert.equal(result.length, 2);

  const response = result[0];

  assert.equal(response.installmentAmount, downPayment);

  const plans = response.plans;
  const firstPlan = plans[0];

  // if the first payment is 20/06/2022, the first plan should be 20/07/2022 because we have 1 down payment to pay
  const planDueDate = new Date('2022-07-20T03:00:00.000Z'); //Node is a joke

  assert.deepEqual(firstPlan.dueDate, planDueDate);

  const response2 = result[1];

  assert.equal(response2.installmentAmount, minInstallmentAmount);

  const plans2 = response2.plans;
  const firstPlan2 = plans2[0];
  // if the first payment is 20/06/2022, the first plan should be 20/08/2022 because we have 2 down payments to pay
  const planDueDate2 = new Date('2022-08-20T03:00:00.000Z'); //Node is a joke

  assert.deepEqual(firstPlan2.dueDate, planDueDate2);
});
