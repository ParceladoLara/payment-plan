// @ts-check

import init, {
  calculatePaymentPlan,
  calculateDownPaymentPlan,
  nextDisbursementDate,
} from "./pkg/wasm_payment_plan.js";

await init();



const params = {
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
  iofPercentage: 0.000082,
  interestRate: 0.0235,
  disbursementOnlyOnBusinessDays:true,
};

console.log(params);

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

const result = calculatePaymentPlan(params);

console.log(result);


const dResult = calculateDownPaymentPlan({
  firstPaymentDate: new Date(),
  installments: 12,
  minInstallmentAmount: 50,
  params,
  requestedAmount: 1000,
});

console.log(result[0].dueDate.toISOString());
console.log(typeof result[0].dueDate);

console.log(typeof dResult[0].firstPaymentDate); //comes as string see: https://github.com/rustwasm/wasm-bindgen/issues/1519

const r = result.pop();

console.log(r);
console.log(typeof r?.dueDate);


if (r) {
  console.log(
    "expectedContractAmount",
    r.contractAmount,
    expectedContractAmount
  );
  console.log(
    "expectedContractAmountWithoutTac",
    r.contractAmountWithoutTAC,
    expectedContractAmountWithoutTac
  );
  console.log(
    "expectedCustomerAmount",
    r.customerAmount,
    expectedCustomerAmount
  );
  console.log(
    "expectedCustomerDebitServiceAmount",
    r.customerDebitServiceAmount,
    expectedCustomerDebitServiceAmount
  );
  console.log("expectedDebitService", r.debitService, expectedDebitService);
  console.log("expectedEirMonthly", r.eirMonthly, expectedEirMonthly);
  console.log("expectedEirYearly", r.eirYearly, expectedEirYearly);
  console.log(
    "expectedEffectiveInterestRate",
    r.effectiveInterestRate,
    expectedEffectiveInterestRate
  );
  console.log("expectedInstallment", r.installment, expectedInstallment);
  console.log(
    "expectedInstallmentAmount",
    r.installmentAmount,
    expectedInstallmentAmount
  );
  console.log("expectedInterestRate", r.interestRate, expectedInterestRate);
  console.log("expectedMdrAmount", r.mdrAmount, expectedMdrAmount);
  console.log(
    "expectedMerchantDebitServiceAmount",
    r.merchantDebitServiceAmount,
    expectedMerchantDebitServiceAmount
  );
  console.log(
    "expectedMerchantTotalAmount",
    r.merchantTotalAmount,
    expectedMerchantTotalAmount
  );
  console.log(
    "expectedSettledToMerchant",
    r.settledToMerchant,
    expectedSettledToMerchant
  );
  console.log("expectedTecMonthly", r.tecMonthly, expectedTecMonthly);
  console.log("expectedTecYearly", r.tecYearly, expectedTecYearly);
  console.log("expectedTotalAmount", r.totalAmount, expectedTotalAmount);
  console.log("expectedTotalIof", r.totalIOF, expectedTotalIof);
}