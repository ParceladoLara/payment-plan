/* tslint:disable */
/* eslint-disable */
export function calculateDownPaymentPlan(p: DownPaymentParams): Array<DownPaymentResponse>;
export function nextDisbursementDate(base_date: Date): Date;
export function calculatePaymentPlan(p: Params): Array<PaymentPlanResponse>;
export function disbursementDateRange(base_date: Date, days: number): Array<Date>;
export function getNonBusinessDaysBetween(start_date: Date, end_date: Date): Array<Date>;
export interface DownPaymentParams {
    params: Params;
    requestedAmount: number;
    minInstallmentAmount: number;
    firstPaymentDate: Date;
    installments: number;
}

export interface Params {
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
    minInstallmentAmount: number;
    maxTotalAmount: number;
    disbursementOnlyOnBusinessDays: boolean;
}

export interface Invoice {
    accumulatedDays: number;
    factor: number;
    accumulatedFactor: number;
    mainIOFTAC: number;
    debitService: number;
    dueDate: Date;
}

export interface PaymentPlanResponse {
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
    disbursementDate: Date;
    paidTotalIOF: number;
    paidContractAmount: number;
    preDisbursementAmount: number;
    invoices: Invoice[];
}

export interface DownPaymentResponse {
    installmentAmount: number;
    totalAmount: number;
    installmentQuantity: number;
    firstPaymentDate: Date;
    plans: PaymentPlanResponse[];
}

