import { test, expect } from "@playwright/test";

test.describe("CalculateDownPaymentPlan page", () => {
  test("should render message", async ({ page }) => {
    const expectedText = `{
  "installmentAmount": 100,
  "totalAmount": 200,
  "installmentQuantity": 2,
  "firstPaymentDate": "2022-06-20T03:00:00.000Z",
  "plans": [
    {
      "installment": 1,
      "dueDate": "2022-08-20T03:00:00.000Z",
      "accumulatedDays": 25,
      "daysIndex": 0.972726300214712,
      "accumulatedDaysIndex": 0.972726300214712,
      "interestRate": 0.0235,
      "installmentAmount": 9099.98,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9099.98,
      "debitService": 248.18999999999957,
      "customerDebitServiceAmount": 248.18999999999957,
      "customerAmount": 9099.98,
      "calculationBasisForEffectiveInterestRate": 9048.189999999999,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0416,
      "eirYearly": 0.4974,
      "tecYearly": 0.631336,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0416,
      "totalIOF": 51.79,
      "contractAmount": 8851.79,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 51.79,
      "paidContractAmount": 8851.79,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 2,
      "dueDate": "2022-09-20T03:00:00.000Z",
      "accumulatedDays": 56,
      "daysIndex": 0.939937712053009,
      "accumulatedDaysIndex": 1.912664012267721,
      "interestRate": 0.0235,
      "installmentAmount": 4633.97,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9267.94,
      "debitService": 404.7100000000005,
      "customerDebitServiceAmount": 404.7100000000005,
      "customerAmount": 4633.97,
      "calculationBasisForEffectiveInterestRate": 4602.3550000000005,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0398,
      "eirYearly": 0.497387,
      "tecYearly": 0.597931,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0398,
      "totalIOF": 63.23,
      "contractAmount": 8863.23,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 63.23,
      "paidContractAmount": 8863.23,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 3,
      "dueDate": "2022-10-20T03:00:00.000Z",
      "accumulatedDays": 86,
      "daysIndex": 0.909259536351008,
      "accumulatedDaysIndex": 2.821923548618729,
      "interestRate": 0.0235,
      "installmentAmount": 3144.87,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9434.61,
      "debitService": 560.0200000000006,
      "customerDebitServiceAmount": 560.0200000000006,
      "customerAmount": 3144.87,
      "calculationBasisForEffectiveInterestRate": 3120.0066666666667,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0391,
      "eirYearly": 0.497394,
      "tecYearly": 0.583763,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0391,
      "totalIOF": 74.59,
      "contractAmount": 8874.59,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 74.58,
      "paidContractAmount": 8874.58,
      "preDisbursementAmount": 8799.99,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 4,
      "dueDate": "2022-11-20T03:00:00.000Z",
      "accumulatedDays": 117,
      "daysIndex": 0.878610281300606,
      "accumulatedDaysIndex": 3.700533829919335,
      "interestRate": 0.0235,
      "installmentAmount": 2401.34,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9605.36,
      "debitService": 719.1200000000006,
      "customerDebitServiceAmount": 719.1200000000006,
      "customerAmount": 2401.34,
      "calculationBasisForEffectiveInterestRate": 2379.78,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0386,
      "eirYearly": 0.497411,
      "tecYearly": 0.575942,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0386,
      "totalIOF": 86.24,
      "contractAmount": 8886.24,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 86.24,
      "paidContractAmount": 8886.24,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 5,
      "dueDate": "2022-12-20T03:00:00.000Z",
      "accumulatedDays": 147,
      "daysIndex": 0.849933742166485,
      "accumulatedDaysIndex": 4.55046757208582,
      "interestRate": 0.0235,
      "installmentAmount": 1955.39,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9776.95,
      "debitService": 879.0100000000007,
      "customerDebitServiceAmount": 879.0100000000007,
      "customerAmount": 1955.39,
      "calculationBasisForEffectiveInterestRate": 1935.8020000000001,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0384,
      "eirYearly": 0.497404,
      "tecYearly": 0.57115,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0384,
      "totalIOF": 97.94,
      "contractAmount": 8897.94,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 97.94,
      "paidContractAmount": 8897.94,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 6,
      "dueDate": "2023-01-20T03:00:00.000Z",
      "accumulatedDays": 178,
      "daysIndex": 0.821284236719289,
      "accumulatedDaysIndex": 5.371751808805109,
      "interestRate": 0.0235,
      "installmentAmount": 1658.65,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9951.9,
      "debitService": 1042.0299999999997,
      "customerDebitServiceAmount": 1042.0299999999997,
      "customerAmount": 1658.65,
      "calculationBasisForEffectiveInterestRate": 1640.3383333333331,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0382,
      "eirYearly": 0.497397,
      "tecYearly": 0.567973,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0382,
      "totalIOF": 109.87,
      "contractAmount": 8909.87,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 109.86,
      "paidContractAmount": 8909.86,
      "preDisbursementAmount": 8799.99,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 7,
      "dueDate": "2023-02-20T03:00:00.000Z",
      "accumulatedDays": 209,
      "daysIndex": 0.793600446741016,
      "accumulatedDaysIndex": 6.165352255546125,
      "interestRate": 0.0235,
      "installmentAmount": 1447.12,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10129.84,
      "debitService": 1207.8500000000001,
      "customerDebitServiceAmount": 1207.8500000000001,
      "customerAmount": 1447.12,
      "calculationBasisForEffectiveInterestRate": 1429.6928571428573,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0381,
      "eirYearly": 0.497391,
      "tecYearly": 0.565811,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0381,
      "totalIOF": 121.99,
      "contractAmount": 8921.99,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 122,
      "paidContractAmount": 8922,
      "preDisbursementAmount": 8800.01,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 8,
      "dueDate": "2023-03-20T03:00:00.000Z",
      "accumulatedDays": 237,
      "daysIndex": 0.769398687203526,
      "accumulatedDaysIndex": 6.934750942749651,
      "interestRate": 0.0235,
      "installmentAmount": 1288.3,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10306.4,
      "debitService": 1372.3599999999997,
      "customerDebitServiceAmount": 1372.3599999999997,
      "customerAmount": 1288.3,
      "calculationBasisForEffectiveInterestRate": 1271.545,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.038,
      "eirYearly": 0.497391,
      "tecYearly": 0.564307,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.038,
      "totalIOF": 134.04,
      "contractAmount": 8934.04,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 134.04,
      "paidContractAmount": 8934.04,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 9,
      "dueDate": "2023-04-20T03:00:00.000Z",
      "accumulatedDays": 268,
      "daysIndex": 0.743463851596323,
      "accumulatedDaysIndex": 7.678214794345974,
      "interestRate": 0.0235,
      "installmentAmount": 1165.15,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10486.35,
      "debitService": 1540.0600000000004,
      "customerDebitServiceAmount": 1540.0600000000004,
      "customerAmount": 1165.15,
      "calculationBasisForEffectiveInterestRate": 1148.8955555555558,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0379,
      "eirYearly": 0.497396,
      "tecYearly": 0.563229,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0379,
      "totalIOF": 146.29,
      "contractAmount": 8946.29,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 146.27,
      "paidContractAmount": 8946.27,
      "preDisbursementAmount": 8799.98,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 10,
      "dueDate": "2023-05-20T03:00:00.000Z",
      "accumulatedDays": 298,
      "daysIndex": 0.719198291894988,
      "accumulatedDaysIndex": 8.397413086240963,
      "interestRate": 0.0235,
      "installmentAmount": 1066.84,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10668.4,
      "debitService": 1709.6799999999996,
      "customerDebitServiceAmount": 1709.6799999999996,
      "customerAmount": 1066.84,
      "calculationBasisForEffectiveInterestRate": 1050.968,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0379,
      "eirYearly": 0.497389,
      "tecYearly": 0.5625,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0379,
      "totalIOF": 158.72,
      "contractAmount": 8958.72,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 158.7,
      "paidContractAmount": 8958.7,
      "preDisbursementAmount": 8799.98,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 11,
      "dueDate": "2023-06-20T03:00:00.000Z",
      "accumulatedDays": 329,
      "daysIndex": 0.694955607602048,
      "accumulatedDaysIndex": 9.09236869384301,
      "interestRate": 0.0235,
      "installmentAmount": 986.69,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10853.59,
      "debitService": 1882.2500000000002,
      "customerDebitServiceAmount": 1882.2500000000002,
      "customerAmount": 986.69,
      "calculationBasisForEffectiveInterestRate": 971.1136363636364,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0379,
      "eirYearly": 0.497379,
      "tecYearly": 0.562019,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0379,
      "totalIOF": 171.34,
      "contractAmount": 8971.34,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 171.35,
      "paidContractAmount": 8971.35,
      "preDisbursementAmount": 8800.01,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 12,
      "dueDate": "2023-07-20T03:00:00.000Z",
      "accumulatedDays": 359,
      "daysIndex": 0.672273285186726,
      "accumulatedDaysIndex": 9.764641979029737,
      "interestRate": 0.0235,
      "installmentAmount": 920.06,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11040.72,
      "debitService": 2056.6199999999994,
      "customerDebitServiceAmount": 2056.6199999999994,
      "customerAmount": 920.06,
      "calculationBasisForEffectiveInterestRate": 904.7183333333332,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0378,
      "eirYearly": 0.497393,
      "tecYearly": 0.561694,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0378,
      "totalIOF": 184.1,
      "contractAmount": 8984.1,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 184.06,
      "paidContractAmount": 8984.06,
      "preDisbursementAmount": 8799.96,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 13,
      "dueDate": "2023-08-20T03:00:00.000Z",
      "accumulatedDays": 390,
      "daysIndex": 0.649612345644702,
      "accumulatedDaysIndex": 10.414254324674438,
      "interestRate": 0.0235,
      "installmentAmount": 863.75,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11228.75,
      "debitService": 2233.43,
      "customerDebitServiceAmount": 2233.43,
      "customerAmount": 863.75,
      "calculationBasisForEffectiveInterestRate": 848.7253846153847,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0378,
      "eirYearly": 0.497409,
      "tecYearly": 0.560976,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0378,
      "totalIOF": 195.32,
      "contractAmount": 8995.32,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 195.31,
      "paidContractAmount": 8995.31,
      "preDisbursementAmount": 8799.99,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 14,
      "dueDate": "2023-09-20T03:00:00.000Z",
      "accumulatedDays": 421,
      "daysIndex": 0.627715259422812,
      "accumulatedDaysIndex": 11.04196958409725,
      "interestRate": 0.0235,
      "installmentAmount": 815.52,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11417.28,
      "debitService": 2412.350000000001,
      "customerDebitServiceAmount": 2412.350000000001,
      "customerAmount": 815.52,
      "calculationBasisForEffectiveInterestRate": 800.8821428571429,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0377,
      "eirYearly": 0.497397,
      "tecYearly": 0.55989,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0377,
      "totalIOF": 204.93,
      "contractAmount": 9004.93,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 204.95,
      "paidContractAmount": 9004.95,
      "preDisbursementAmount": 8800.02,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 15,
      "dueDate": "2023-10-20T03:00:00.000Z",
      "accumulatedDays": 451,
      "daysIndex": 0.607227562448361,
      "accumulatedDaysIndex": 11.649197146545612,
      "interestRate": 0.0235,
      "installmentAmount": 773.72,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11605.8,
      "debitService": 2592.5499999999993,
      "customerDebitServiceAmount": 2592.5499999999993,
      "customerAmount": 773.72,
      "calculationBasisForEffectiveInterestRate": 759.5033333333333,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0377,
      "eirYearly": 0.497405,
      "tecYearly": 0.558588,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0377,
      "totalIOF": 213.25,
      "contractAmount": 9013.25,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 213.22,
      "paidContractAmount": 9013.22,
      "preDisbursementAmount": 8799.97,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 16,
      "dueDate": "2023-11-20T03:00:00.000Z",
      "accumulatedDays": 482,
      "daysIndex": 0.586759179449814,
      "accumulatedDaysIndex": 12.235956325995426,
      "interestRate": 0.0235,
      "installmentAmount": 737.21,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11795.36,
      "debitService": 2774.8300000000004,
      "customerDebitServiceAmount": 2774.8300000000004,
      "customerAmount": 737.21,
      "calculationBasisForEffectiveInterestRate": 723.426875,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0376,
      "eirYearly": 0.497389,
      "tecYearly": 0.557188,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0376,
      "totalIOF": 220.53,
      "contractAmount": 9020.53,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 220.47,
      "paidContractAmount": 9020.47,
      "preDisbursementAmount": 8799.94,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 17,
      "dueDate": "2023-12-20T03:00:00.000Z",
      "accumulatedDays": 512,
      "daysIndex": 0.567608228305822,
      "accumulatedDaysIndex": 12.803564554301248,
      "interestRate": 0.0235,
      "installmentAmount": 705.03,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11985.51,
      "debitService": 2958.5600000000004,
      "customerDebitServiceAmount": 2958.5600000000004,
      "customerAmount": 705.03,
      "calculationBasisForEffectiveInterestRate": 691.6800000000001,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0375,
      "eirYearly": 0.4974,
      "tecYearly": 0.555763,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0375,
      "totalIOF": 226.95,
      "contractAmount": 9026.95,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 226.9,
      "paidContractAmount": 9026.9,
      "preDisbursementAmount": 8799.95,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 18,
      "dueDate": "2024-01-20T03:00:00.000Z",
      "accumulatedDays": 543,
      "daysIndex": 0.548475330972825,
      "accumulatedDaysIndex": 13.352039885274074,
      "interestRate": 0.0235,
      "installmentAmount": 676.5,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 12177,
      "debitService": 3144.37,
      "customerDebitServiceAmount": 3144.37,
      "customerAmount": 676.5,
      "calculationBasisForEffectiveInterestRate": 663.576111111111,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0374,
      "eirYearly": 0.49741,
      "tecYearly": 0.554346,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0374,
      "totalIOF": 232.63,
      "contractAmount": 9032.63,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 232.65,
      "paidContractAmount": 9032.65,
      "preDisbursementAmount": 8800.02,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "543",
          "factor": 0.548475330972825,
          "accumulatedFactor": 13.352039885274074,
          "dueDate": "2024-01-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 19,
      "dueDate": "2024-02-20T03:00:00.000Z",
      "accumulatedDays": 574,
      "daysIndex": 0.529987363966945,
      "accumulatedDaysIndex": 13.88202724924102,
      "interestRate": 0.0235,
      "installmentAmount": 651.04,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 12369.76,
      "debitService": 3332.05,
      "customerDebitServiceAmount": 3332.05,
      "customerAmount": 651.04,
      "calculationBasisForEffectiveInterestRate": 638.528947368421,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0374,
      "eirYearly": 0.497385,
      "tecYearly": 0.552935,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0374,
      "totalIOF": 237.71,
      "contractAmount": 9037.71,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 237.76,
      "paidContractAmount": 9037.76,
      "preDisbursementAmount": 8800.05,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "543",
          "factor": 0.548475330972825,
          "accumulatedFactor": 13.352039885274074,
          "dueDate": "2024-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.529987363966945,
          "accumulatedFactor": 13.88202724924102,
          "dueDate": "2024-02-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 20,
      "dueDate": "2024-03-20T03:00:00.000Z",
      "accumulatedDays": 603,
      "daysIndex": 0.51325676093444,
      "accumulatedDaysIndex": 14.395284010175459,
      "interestRate": 0.0235,
      "installmentAmount": 628.14,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 12562.8,
      "debitService": 3520.539999999999,
      "customerDebitServiceAmount": 3520.539999999999,
      "customerAmount": 628.14,
      "calculationBasisForEffectiveInterestRate": 616.0269999999999,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0373,
      "eirYearly": 0.497391,
      "tecYearly": 0.551551,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0373,
      "totalIOF": 242.26,
      "contractAmount": 9042.26,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 242.25,
      "paidContractAmount": 9042.25,
      "preDisbursementAmount": 8799.99,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "543",
          "factor": 0.548475330972825,
          "accumulatedFactor": 13.352039885274074,
          "dueDate": "2024-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.529987363966945,
          "accumulatedFactor": 13.88202724924102,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.51325676093444,
          "accumulatedFactor": 14.395284010175459,
          "dueDate": "2024-03-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 21,
      "dueDate": "2024-04-20T03:00:00.000Z",
      "accumulatedDays": 634,
      "daysIndex": 0.495955938954224,
      "accumulatedDaysIndex": 14.891239949129682,
      "interestRate": 0.0235,
      "installmentAmount": 607.5,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 12757.5,
      "debitService": 3711.13,
      "customerDebitServiceAmount": 3711.13,
      "customerAmount": 607.5,
      "calculationBasisForEffectiveInterestRate": 595.7680952380953,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0372,
      "eirYearly": 0.49739,
      "tecYearly": 0.550238,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0372,
      "totalIOF": 246.37,
      "contractAmount": 9046.37,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 246.43,
      "paidContractAmount": 9046.43,
      "preDisbursementAmount": 8800.06,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "543",
          "factor": 0.548475330972825,
          "accumulatedFactor": 13.352039885274074,
          "dueDate": "2024-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.529987363966945,
          "accumulatedFactor": 13.88202724924102,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.51325676093444,
          "accumulatedFactor": 14.395284010175459,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "634",
          "factor": 0.495955938954224,
          "accumulatedFactor": 14.891239949129682,
          "dueDate": "2024-04-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 22,
      "dueDate": "2024-05-20T03:00:00.000Z",
      "accumulatedDays": 664,
      "daysIndex": 0.47976867118043,
      "accumulatedDaysIndex": 15.371008620310112,
      "interestRate": 0.0235,
      "installmentAmount": 588.78,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 12953.16,
      "debitService": 3903.0699999999997,
      "customerDebitServiceAmount": 3903.0699999999997,
      "customerAmount": 588.78,
      "calculationBasisForEffectiveInterestRate": 577.4122727272727,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0371,
      "eirYearly": 0.497407,
      "tecYearly": 0.548962,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0371,
      "totalIOF": 250.09,
      "contractAmount": 9050.09,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 250.14,
      "paidContractAmount": 9050.14,
      "preDisbursementAmount": 8800.05,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "543",
          "factor": 0.548475330972825,
          "accumulatedFactor": 13.352039885274074,
          "dueDate": "2024-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.529987363966945,
          "accumulatedFactor": 13.88202724924102,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.51325676093444,
          "accumulatedFactor": 14.395284010175459,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "634",
          "factor": 0.495955938954224,
          "accumulatedFactor": 14.891239949129682,
          "dueDate": "2024-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "664",
          "factor": 0.47976867118043,
          "accumulatedFactor": 15.371008620310112,
          "dueDate": "2024-05-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 23,
      "dueDate": "2024-06-20T03:00:00.000Z",
      "accumulatedDays": 695,
      "daysIndex": 0.463596663320922,
      "accumulatedDaysIndex": 15.834605283631035,
      "interestRate": 0.0235,
      "installmentAmount": 571.75,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 13150.25,
      "debitService": 4096.75,
      "customerDebitServiceAmount": 4096.75,
      "customerAmount": 571.75,
      "calculationBasisForEffectiveInterestRate": 560.7282608695652,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.0371,
      "eirYearly": 0.49738,
      "tecYearly": 0.547721,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.0371,
      "totalIOF": 253.5,
      "contractAmount": 9053.5,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 253.44,
      "paidContractAmount": 9053.44,
      "preDisbursementAmount": 8799.94,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "543",
          "factor": 0.548475330972825,
          "accumulatedFactor": 13.352039885274074,
          "dueDate": "2024-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.529987363966945,
          "accumulatedFactor": 13.88202724924102,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.51325676093444,
          "accumulatedFactor": 14.395284010175459,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "634",
          "factor": 0.495955938954224,
          "accumulatedFactor": 14.891239949129682,
          "dueDate": "2024-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "664",
          "factor": 0.47976867118043,
          "accumulatedFactor": 15.371008620310112,
          "dueDate": "2024-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "695",
          "factor": 0.463596663320922,
          "accumulatedFactor": 15.834605283631035,
          "dueDate": "2024-06-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 24,
      "dueDate": "2024-07-20T03:00:00.000Z",
      "accumulatedDays": 725,
      "daysIndex": 0.44846555440823,
      "accumulatedDaysIndex": 16.283070838039265,
      "interestRate": 0.0235,
      "installmentAmount": 556.2,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 13348.8,
      "debitService": 4292.219999999999,
      "customerDebitServiceAmount": 4292.219999999999,
      "customerAmount": 556.2,
      "calculationBasisForEffectiveInterestRate": 545.5091666666666,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0342,
      "totalEffectiveCost": 0.037,
      "eirYearly": 0.497399,
      "tecYearly": 0.546577,
      "eirMonthly": 0.0342,
      "tecMonthly": 0.037,
      "totalIOF": 256.58,
      "contractAmount": 9056.58,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 256.64,
      "paidContractAmount": 9056.64,
      "preDisbursementAmount": 8800.06,
      "invoices": [
        {
          "accumulatedDays": "25",
          "factor": 0.972726300214712,
          "accumulatedFactor": 0.972726300214712,
          "dueDate": "2022-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.939937712053009,
          "accumulatedFactor": 1.912664012267721,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.909259536351008,
          "accumulatedFactor": 2.821923548618729,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "117",
          "factor": 0.878610281300606,
          "accumulatedFactor": 3.700533829919335,
          "dueDate": "2022-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.849933742166485,
          "accumulatedFactor": 4.55046757208582,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.821284236719289,
          "accumulatedFactor": 5.371751808805109,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.793600446741016,
          "accumulatedFactor": 6.165352255546125,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.769398687203526,
          "accumulatedFactor": 6.934750942749651,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.743463851596323,
          "accumulatedFactor": 7.678214794345974,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "298",
          "factor": 0.719198291894988,
          "accumulatedFactor": 8.397413086240963,
          "dueDate": "2023-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.694955607602048,
          "accumulatedFactor": 9.09236869384301,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.672273285186726,
          "accumulatedFactor": 9.764641979029737,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "390",
          "factor": 0.649612345644702,
          "accumulatedFactor": 10.414254324674438,
          "dueDate": "2023-08-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.627715259422812,
          "accumulatedFactor": 11.04196958409725,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.607227562448361,
          "accumulatedFactor": 11.649197146545612,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.586759179449814,
          "accumulatedFactor": 12.235956325995426,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.567608228305822,
          "accumulatedFactor": 12.803564554301248,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "543",
          "factor": 0.548475330972825,
          "accumulatedFactor": 13.352039885274074,
          "dueDate": "2024-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.529987363966945,
          "accumulatedFactor": 13.88202724924102,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.51325676093444,
          "accumulatedFactor": 14.395284010175459,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "634",
          "factor": 0.495955938954224,
          "accumulatedFactor": 14.891239949129682,
          "dueDate": "2024-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "664",
          "factor": 0.47976867118043,
          "accumulatedFactor": 15.371008620310112,
          "dueDate": "2024-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "695",
          "factor": 0.463596663320922,
          "accumulatedFactor": 15.834605283631035,
          "dueDate": "2024-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "725",
          "factor": 0.44846555440823,
          "accumulatedFactor": 16.283070838039265,
          "dueDate": "2024-07-20T03:00:00.000Z"
        }
      ]
    }
  ]
}`;
    await page.goto("/");
    await page.click('button:has-text("Calculate Down Payment Plan")');
    await expect(page.locator("pre")).toContainText(expectedText.trim());
  });
});
