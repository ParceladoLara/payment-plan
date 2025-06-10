export type PaymentPlanParams = {
    requestedAmount: number;
    firstPaymentDate: Date;
    disbursementDate: Date;
    installments: number;
    debitServicePercentage: number;
    mdr: number;
    tacPercentage: number;
    iofOverall: number;
    iofPercentage: number;
    interestRate: number;
    minInstallmentAmount?: number;
    maxTotalAmount?: number;
    disbursementOnlyOnBusinessDays?: boolean;
};
export type Invoice = {
    accumulatedDays: number;
    factor: number;
    accumulatedFactor: number;
    dueDate: Date;
};
export type PaymentPlanResponse = {
    installment: number;
    dueDate: Date;
    disbursementDate: Date;
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
    preDisbursementAmount: number;
    paidTotalIOF: number;
    paidContractAmount: number;
    invoices: Invoice[];
};
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
/**
 * @typedef {Object} PaymentPlanParams
 * @property {number} requestedAmount
 * @property {Date} firstPaymentDate
 * @property {Date} disbursementDate
 * @property {number} installments
 * @property {number} debitServicePercentage
 * @property {number} mdr
 * @property {number} tacPercentage
 * @property {number} iofOverall
 * @property {number} iofPercentage
 * @property {number} interestRate
 * @property {number} [minInstallmentAmount]
 * @property {number} [maxTotalAmount]
 * @property {boolean} [disbursementOnlyOnBusinessDays]
 */
/**
 * @typedef {Object} Invoice
 * @property {number} accumulatedDays
 * @property {number} factor
 * @property {number} accumulatedFactor
 * @property {Date} dueDate
 */
/**
 * @typedef {Object} PaymentPlanResponse
 * @property {number} installment
 * @property {Date} dueDate
 * @property {Date} disbursementDate
 * @property {number} accumulatedDays
 * @property {number} daysIndex
 * @property {number} accumulatedDaysIndex
 * @property {number} interestRate
 * @property {number} installmentAmount
 * @property {number} installmentAmountWithoutTAC
 * @property {number} totalAmount
 * @property {number} debitService
 * @property {number} customerDebitServiceAmount
 * @property {number} customerAmount
 * @property {number} calculationBasisForEffectiveInterestRate
 * @property {number} merchantDebitServiceAmount
 * @property {number} merchantTotalAmount
 * @property {number} settledToMerchant
 * @property {number} mdrAmount
 * @property {number} effectiveInterestRate
 * @property {number} totalEffectiveCost
 * @property {number} eirYearly
 * @property {number} tecYearly
 * @property {number} eirMonthly
 * @property {number} tecMonthly
 * @property {number} totalIOF
 * @property {number} contractAmount
 * @property {number} contractAmountWithoutTAC
 * @property {number} tacAmount
 * @property {number} IOFPercentage
 * @property {number} overallIOF
 * @property {number} preDisbursementAmount
 * @property {number} paidTotalIOF
 * @property {number} paidContractAmount
 * @property {Invoice[]} invoices
 */
/**
 * @typedef {Object} DownPaymentPlanParams
 * @property {PaymentPlanParams} params
 * @property {number} requestedAmount
 * @property {Date} firstPaymentDate
 * @property {number} installments
 * @property {number} minInstallmentAmount
 */
/**
 * @typedef {Object} DownPaymentPlanResponse
 * @property {number} installmentAmount
 * @property {number} totalAmount
 * @property {number} installmentQuantity
 * @property {Date} firstPaymentDate
 * @property {PaymentPlanResponse[]} plans
 */
/**
 * @param {PaymentPlanParams} params
 * @returns {PaymentPlanResponse[]}
 */
export function calculatePlan(params: PaymentPlanParams): PaymentPlanResponse[];
/**
 * @param {DownPaymentPlanParams} params
 * @returns {DownPaymentPlanResponse[]}
 */
export function calculateDownPaymentPlan(params: DownPaymentPlanParams): DownPaymentPlanResponse[];
/**
 * @param {Date} date
 * @returns {Date}
 */
export function nextDisbursementDate(date: Date): Date;
/**
 * @param {Date} base_date
 * @param {number} days
 * @returns {Date[]}
 */
export function disbursementDateRange(base_date: Date, days: number): Date[];
/**
 * @param {Date} start_date
 * @param {Date} end_date
 * @returns {Date[]}
 */
export function getNonBusinessDaysBetween(start_date: Date, end_date: Date): Date[];
//# sourceMappingURL=index.d.ts.map