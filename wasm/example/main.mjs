// @ts-check

import init, {
  calculatePaymentPlan,
  calculateDownPaymentPlan,
} from "./pkg/wasm_payment_plan.js";

await init();

const params = {
  requestedAmount: 1000,
  firstPaymentDate: new Date(),
  requestedDate: new Date(),
  installments: 12,
  debitServicePercentage: 1,
  mdr: 0.4,
  tacPercentage: 0.03,
  iofOverall: 0.38,
  iofPercentage: 0.0041,
  interestRate: 0.55,
  minInstallmentAmount: 50,
  maxTotalAmount: 10000,
};

const result = calculatePaymentPlan(params);

const dResult = calculateDownPaymentPlan({
  firstPaymentDate: new Date(),
  installments: 12,
  minInstallmentAmount: 50,
  params,
  requestAmount: 1000,
});

console.log(result);

console.log(dResult);
