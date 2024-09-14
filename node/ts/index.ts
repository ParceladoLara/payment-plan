import {
  calculateDownPaymentPlan,
  calculatePlan,
  DownPaymentPlanParams,
  PaymentPlanParams,
} from "./payment-plan";

const plan: PaymentPlanParams = {
  debitServicePercentage: 0,
  firstPaymentDate: new Date("2024-10-10"),
  installments: 24,
  interestRate: 0.03,
  iofOverall: 0.038,
  iofPercentage: 0.0038,
  mdr: 0.02,
  requestedAmount: 7000,
  requestedDate: new Date("2024-10-08"),
  tacPercentage: 0,
  maxTotalAmount: 100000000,
  minInstallmentAmount: 100,
};

const dPlan: DownPaymentPlanParams = {
  params: plan,
  firstPaymentDate: new Date("2024-10-10"),
  installments: 4,
  requestedAmount: 3000,
  minInstallmentAmount: 100,
};

const result = calculatePlan(plan);

console.log(result);

const dResult = calculateDownPaymentPlan(dPlan);

console.log(dResult);
