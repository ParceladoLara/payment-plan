export type PaymentPlanParams = {
  requestedAmount: number;
  firstPaymentDate: Date;
  requestedDate: Date;
  installments: number;
  debitServicePercentage: number;
  mdr: number;
  tacPercentage: number;
  iofOverall: number;
  iofPercentage: number;
  interestRate: number;
  minInstallmentAmount?: number;
  maxTotalAmount?: number;
};

export type PaymentPlanResponse = {
  installment: number;
  dueDate: Date;
  accumulatedDays: number;
  daysIndex: number;
  accumulatedDaysIndex: number;
  interestRate: number;
  installmentAmount: number;
  installmentAmountWithoutTAC: number;
  totalAmount: number;
  debitService: number;
  customerDebitServiceAmount: number;
  customerAmount: number;
  calculationBasisForEffectiveInterestRate: number;
  merchantDebitServiceAmount: number;
  merchantTotalAmount: number;
  settledToMerchant: number;
  mdrAmount: number;
  effectiveInterestRate: number;
  totalEffectiveCost: number;
  eirYearly: number;
  tecYearly: number;
  eirMonthly: number;
  tecMonthly: number;
  totalIOF: number;
  contractAmount: number;
  contractAmountWithoutTAC: number;
  tacAmount: number;
  IOFPercentage: number;
  overallIOF: number;
};

/*

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct DownPaymentParams {
    pub params: Params,              // The params for the actual payment plan
    pub request_amount: f64,         // The requested amount for the down payment(ex: 1000.0)
    pub min_installment_amount: f64, // The minium installment value for the down payment (ex: 100.0)
    pub first_payment_date: chrono::NaiveDate, // The first payment date for the down payment
    pub installments: u32,           // The max number of installments for the down payment (ex: 12)
}
    */

export type DownPaymentPlanParams = {
  params: PaymentPlanParams;
  requestedAmount: number;
  firstPaymentDate: Date;
  installments: number;
  minInstallmentAmount: number;
};

export type DownPaymentPlanResponse = {
  installmentAmount: number;
  totalAmount: number;
  installmentQuantity: number;
  firstPaymentDate: Date;
  plans: PaymentPlanResponse[];
};

interface PaymentPlanFunctions {
  calculatePlan(params: PaymentPlanParams): PaymentPlanResponse[];
  calculateDownPaymentPlan(
    params: DownPaymentPlanParams,
  ): DownPaymentPlanResponse[];
  calculateReimbursementPlan(params: any): any;
}

import * as funcs from '../index.node';
const { calculatePlan, calculateDownPaymentPlan, calculateReimbursementPlan } =
  funcs as PaymentPlanFunctions;

export { calculatePlan, calculateDownPaymentPlan, calculateReimbursementPlan };
