import { test } from "node:test";
import * as assert from "node:assert";
import { calculatePaymentPlan, Params } from "../../pkg/wasm_payment_plan";

test("calculate payment plan test 0", () => {
  const expectedContractAmount = 9037.318869753424;
  const expectedContractAmountWithoutTac = 9037.318869753424;
  const expectedCustomerAmount = 499.1987614851067;
  const expectedCustomerDebitServiceAmount = 2943.451405889136;
  const expectedDebitService = 2943.451405889136;
  const expectedEirMonthly = 0.024085088183680048;
  const expectedEirYearly = 0.33055401101326365;
  const expectedEffectiveInterestRate = 0.024085088183680048;
  const expectedInstallment = 24;
  const expectedInstallmentAmount = 499.1987614851067;
  const expectedInterestRate = 0.0235;
  const expectedMdrAmount = 440.0;
  const expectedMerchantDebitServiceAmount = 0.0;
  const expectedMerchantTotalAmount = 440.0;
  const expectedSettledToMerchant = 8360.0;
  const expectedTecMonthly = 0.025868426671143974;
  const expectedTecYearly = 0.3586261331729559;
  const expectedTotalAmount = 11980.77027564256;
  const expectedTotalIof = 237.3188697534247;

  const params: Params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date("2022-04-18"),
    requestedDate: new Date("2022-03-18"),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.03,
    interestRate: 0.0235,
  };

  const result = calculatePaymentPlan(params);

  assert.equal(result.length, 24);

  const response = result.pop();

  assert.equal(response.contractAmount, expectedContractAmount);
  assert.equal(
    response.contractAmountWithoutTAC,
    expectedContractAmountWithoutTac
  );
  assert.equal(response.customerAmount, expectedCustomerAmount);
  assert.equal(
    response.customerDebitServiceAmount,
    expectedCustomerDebitServiceAmount
  );
  assert.equal(response.debitService, expectedDebitService);
  assert.equal(response.eirMonthly, expectedEirMonthly);
  assert.equal(response.eirYearly, expectedEirYearly);
  assert.equal(response.effectiveInterestRate, expectedEffectiveInterestRate);
  assert.equal(response.installment, expectedInstallment);
  assert.equal(response.installmentAmount, expectedInstallmentAmount);
  assert.equal(response.interestRate, expectedInterestRate);
  assert.equal(response.mdrAmount, expectedMdrAmount);
  assert.equal(
    response.merchantDebitServiceAmount,
    expectedMerchantDebitServiceAmount
  );
  assert.equal(response.merchantTotalAmount, expectedMerchantTotalAmount);
  assert.equal(response.settledToMerchant, expectedSettledToMerchant);
  assert.equal(response.tecMonthly, expectedTecMonthly);
  assert.equal(response.tecYearly, expectedTecYearly);
  assert.equal(response.totalAmount, expectedTotalAmount);
  assert.equal(response.totalIOF, expectedTotalIof);
});

test("Error: invalid requestedAmount", () => {
  const params: Params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: -8800.0,
    firstPaymentDate: new Date("2022-04-18"),
    requestedDate: new Date("2022-03-18"),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.03,
    interestRate: 0.0235,
  };

  assert.throws(() => calculatePaymentPlan(params), {
    message: "Requested amount must be greater than 0",
  });
});

test("Error: invalid installments", () => {
  const params: Params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date("2022-04-18"),
    requestedDate: new Date("2022-03-18"),
    installments: 0,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.03,
    interestRate: 0.0235,
  };

  assert.throws(() => calculatePaymentPlan(params), {
    message: "Number of installments must be greater than 0",
  });
});
