const funcs = require('../native/index.node');
const __calculatePlan = funcs.calculatePlan;
const __calculateDownPaymentPlan = funcs.calculateDownPaymentPlan;
const __nextDisbursementDate = funcs.nextDisbursementDate;
const __disbursementDateRange = funcs.disbursementDateRange;
const __getNonBusinessDaysBetween = funcs.getNonBusinessDaysBetween;
// @ts-check

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
 * @property {number} mainIOFTAC
 * @property {number} debitService
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
function calculatePlan(params) {
  return __calculatePlan(params);
}

/**
 * @param {DownPaymentPlanParams} params
 * @returns {DownPaymentPlanResponse[]}
 */
function calculateDownPaymentPlan(params) {
  return __calculateDownPaymentPlan(params);
}

/**
 * @param {Date} date
 * @returns {Date}
 */
function nextDisbursementDate(date) {
  return __nextDisbursementDate(date);
}

/**
 * @param {Date} base_date
 * @param {number} days
 * @returns {Date[]}
 */
function disbursementDateRange(base_date, days) {
  return __disbursementDateRange(base_date, days);
}

/**
 * @param {Date} start_date
 * @param {Date} end_date
 * @returns {Date[]}
 */
function getNonBusinessDaysBetween(start_date, end_date) {
  return __getNonBusinessDaysBetween(start_date, end_date);
}

module.exports = {
  calculatePlan,
  calculateDownPaymentPlan,
  nextDisbursementDate,
  disbursementDateRange,
  getNonBusinessDaysBetween,
};
