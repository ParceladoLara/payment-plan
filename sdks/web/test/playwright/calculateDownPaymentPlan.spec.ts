import { test, expect } from "@playwright/test";

test.describe("CalculateDownPaymentPlan page", () => {
  test("should render message", async ({ page }) => {
    const expectedText = `

{
  "installmentAmount": 100,
  "totalAmount": 200,
  "installmentQuantity": 2,
  "firstPaymentDate": "2022-06-20T03:00:00.000Z",
  "plans": [
    {
      "installment": 1,
      "dueDate": "2022-08-22T03:00:00.000Z",
      "accumulatedDays": 27,
      "daysIndex": 0.9792033719596,
      "accumulatedDaysIndex": 0.9792033719596,
      "interestRate": 0.0235,
      "installmentAmount": 9041.27,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9041.27,
      "debitService": 188.03000000000043,
      "customerDebitServiceAmount": 188.03000000000043,
      "customerAmount": 9041.27,
      "calculationBasisForEffectiveInterestRate": 8988.03,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.024,
      "totalEffectiveCost": 0.0309,
      "eirYearly": 0.328577,
      "tecYearly": 0.441448,
      "eirMonthly": 0.024,
      "tecMonthly": 0.0309,
      "totalIOF": 53.24,
      "contractAmount": 8853.24,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 53.24,
      "paidContractAmount": 8853.24,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 8833.21886,
          "debitService": 208.05114,
          "dueDate": "2022-08-22T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 2,
      "dueDate": "2022-09-20T03:00:00.000Z",
      "accumulatedDays": 56,
      "daysIndex": 0.956720441569568,
      "accumulatedDaysIndex": 1.935923813529168,
      "interestRate": 0.0235,
      "installmentAmount": 4578.66,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9157.32,
      "debitService": 293.3799999999997,
      "customerDebitServiceAmount": 293.3799999999997,
      "customerAmount": 4578.66,
      "calculationBasisForEffectiveInterestRate": 4546.69,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0242,
      "totalEffectiveCost": 0.0297,
      "eirYearly": 0.332369,
      "tecYearly": 0.420372,
      "eirMonthly": 0.0242,
      "tecMonthly": 0.0297,
      "totalIOF": 63.94,
      "contractAmount": 8863.94,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 63.94,
      "paidContractAmount": 8863.94,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 4370.35741,
          "debitService": 208.30259,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 4473.0608091349995,
          "debitService": 105.59919086500003,
          "dueDate": "2022-09-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 3,
      "dueDate": "2022-10-20T03:00:00.000Z",
      "accumulatedDays": 86,
      "daysIndex": 0.933720368270266,
      "accumulatedDaysIndex": 2.8696441817994343,
      "interestRate": 0.0235,
      "installmentAmount": 3092.72,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9278.16,
      "debitService": 403.15999999999985,
      "customerDebitServiceAmount": 403.15999999999985,
      "customerAmount": 3092.72,
      "calculationBasisForEffectiveInterestRate": 3067.72,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0244,
      "totalEffectiveCost": 0.0291,
      "eirYearly": 0.335106,
      "tecYearly": 0.411289,
      "eirMonthly": 0.0244,
      "tecMonthly": 0.0291,
      "totalIOF": 75,
      "contractAmount": 8875,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 75.01,
      "paidContractAmount": 8875.01,
      "preDisbursementAmount": 8800.01,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 2884.1575,
          "debitService": 208.5625,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 2951.93520125,
          "debitService": 140.78479875000002,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 3021.305678479375,
          "debitService": 71.41432152062502,
          "dueDate": "2022-10-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 4,
      "dueDate": "2022-11-21T03:00:00.000Z",
      "accumulatedDays": 118,
      "daysIndex": 0.911273229087126,
      "accumulatedDaysIndex": 3.7809174108865604,
      "interestRate": 0.0235,
      "installmentAmount": 2350.38,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9401.52,
      "debitService": 514.9100000000004,
      "customerDebitServiceAmount": 514.9100000000004,
      "customerAmount": 2350.38,
      "calculationBasisForEffectiveInterestRate": 2328.7275,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0286,
      "eirYearly": 0.334254,
      "tecYearly": 0.403366,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0286,
      "totalIOF": 86.61,
      "contractAmount": 8886.61,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 86.59,
      "paidContractAmount": 8886.59,
      "preDisbursementAmount": 8799.98,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 2141.544665,
          "debitService": 208.83533500000001,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 2191.8709646275,
          "debitService": 158.50903537250002,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 2243.3799322962464,
          "debitService": 107.00006770375377,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 2296.0993607052083,
          "debitService": 54.28063929479198,
          "dueDate": "2022-11-21T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 5,
      "dueDate": "2022-12-20T03:00:00.000Z",
      "accumulatedDays": 147,
      "daysIndex": 0.89035000398133,
      "accumulatedDaysIndex": 4.6712674148678905,
      "interestRate": 0.0235,
      "installmentAmount": 1904.85,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9524.25,
      "debitService": 626.19,
      "customerDebitServiceAmount": 626.19,
      "customerAmount": 1904.85,
      "calculationBasisForEffectiveInterestRate": 1885.238,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0284,
      "eirYearly": 0.33426,
      "tecYearly": 0.399091,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0284,
      "totalIOF": 98.06,
      "contractAmount": 8898.06,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 98.06,
      "paidContractAmount": 8898.06,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 1695.74559,
          "debitService": 209.10441,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 1735.595611365,
          "debitService": 169.25438863499997,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 1776.3821082320774,
          "debitService": 128.46789176792248,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 1818.1270877755312,
          "debitService": 86.72291222446866,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 1860.8530743382562,
          "debitService": 43.996925661743674,
          "dueDate": "2022-12-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 6,
      "dueDate": "2023-01-20T03:00:00.000Z",
      "accumulatedDays": 178,
      "daysIndex": 0.867984901303578,
      "accumulatedDaysIndex": 5.539252316171469,
      "interestRate": 0.0235,
      "installmentAmount": 1608.47,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9650.82,
      "debitService": 741.0899999999997,
      "customerDebitServiceAmount": 741.0899999999997,
      "customerAmount": 1608.47,
      "calculationBasisForEffectiveInterestRate": 1590.1816666666666,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0244,
      "totalEffectiveCost": 0.0282,
      "eirYearly": 0.334972,
      "tecYearly": 0.396922,
      "eirMonthly": 0.0244,
      "tecMonthly": 0.0282,
      "totalIOF": 109.73,
      "contractAmount": 8909.73,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 109.72,
      "paidContractAmount": 8909.72,
      "preDisbursementAmount": 8799.99,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 1399.091345,
          "debitService": 209.37865499999998,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 1431.9699916075,
          "debitService": 176.5000083925,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 1465.6212864102763,
          "debitService": 142.84871358972376,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 1500.0633866409178,
          "debitService": 108.40661335908226,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 1535.3148762269793,
          "debitService": 73.15512377302069,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 1571.3947758183133,
          "debitService": 37.07522418168667,
          "dueDate": "2023-01-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 7,
      "dueDate": "2023-02-20T03:00:00.000Z",
      "accumulatedDays": 209,
      "daysIndex": 0.8480555948138,
      "accumulatedDaysIndex": 6.387307910985269,
      "interestRate": 0.0235,
      "installmentAmount": 1396.77,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9777.39,
      "debitService": 855.7799999999994,
      "customerDebitServiceAmount": 855.7799999999994,
      "customerAmount": 1396.77,
      "calculationBasisForEffectiveInterestRate": 1379.3971428571426,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0281,
      "eirYearly": 0.334618,
      "tecYearly": 0.394498,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0281,
      "totalIOF": 121.61,
      "contractAmount": 8921.61,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 121.6,
      "paidContractAmount": 8921.6,
      "preDisbursementAmount": 8799.99,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 1187.112165,
          "debitService": 209.657835,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 1215.0093008775,
          "debitService": 181.7606991225,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 1243.5620194481212,
          "debitService": 153.20798055187876,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 1272.7857269051522,
          "debitService": 123.9842730948479,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 1302.6961914874232,
          "debitService": 94.07380851257682,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 1333.3095519873775,
          "debitService": 63.46044801262237,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 1364.642326459081,
          "debitService": 32.127673540919,
          "dueDate": "2023-02-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 8,
      "dueDate": "2023-03-20T03:00:00.000Z",
      "accumulatedDays": 237,
      "daysIndex": 0.829500878479253,
      "accumulatedDaysIndex": 7.2168087894645225,
      "interestRate": 0.0235,
      "installmentAmount": 1237.85,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 9902.8,
      "debitService": 969.4399999999993,
      "customerDebitServiceAmount": 969.4399999999993,
      "customerAmount": 1237.85,
      "calculationBasisForEffectiveInterestRate": 1221.1799999999998,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.028,
      "eirYearly": 0.334424,
      "tecYearly": 0.392791,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.028,
      "totalIOF": 133.36,
      "contractAmount": 8933.36,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 133.33,
      "paidContractAmount": 8933.33,
      "preDisbursementAmount": 8799.97,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 1027.9160399999998,
          "debitService": 209.93396,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 1052.0720669399998,
          "debitService": 185.77793306,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 1076.7957605130898,
          "debitService": 161.05423948691,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 1102.1004608851474,
          "debitService": 135.7495391148524,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 1127.9998217159484,
          "debitService": 109.85017828405145,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 1154.5078175262734,
          "debitService": 83.34218247372667,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 1181.6387512381407,
          "debitService": 56.211248761859245,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 1209.4072618922369,
          "debitService": 28.442738107762942,
          "dueDate": "2023-03-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 9,
      "dueDate": "2023-04-20T03:00:00.000Z",
      "accumulatedDays": 268,
      "daysIndex": 0.808664272385564,
      "accumulatedDaysIndex": 8.025473061850086,
      "interestRate": 0.0235,
      "installmentAmount": 1114.61,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10031.49,
      "debitService": 1086.1899999999998,
      "customerDebitServiceAmount": 1086.1899999999998,
      "customerAmount": 1114.61,
      "calculationBasisForEffectiveInterestRate": 1098.4655555555555,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0279,
      "eirYearly": 0.334603,
      "tecYearly": 0.391895,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0279,
      "totalIOF": 145.3,
      "contractAmount": 8945.3,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 145.27,
      "paidContractAmount": 8945.27,
      "preDisbursementAmount": 8799.97,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 904.39545,
          "debitService": 210.21454999999997,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 925.6487430749999,
          "debitService": 188.961256925,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 947.4014885372624,
          "debitService": 167.20851146273748,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 969.665423517888,
          "debitService": 144.9445764821118,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 992.4525609705585,
          "debitService": 122.15743902944145,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 1015.7751961533666,
          "debitService": 98.83480384663332,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 1039.6459132629707,
          "debitService": 74.96408673702919,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 1064.0775922246505,
          "debitService": 50.532407775349384,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 1089.0834156419298,
          "debitService": 25.526584358070096,
          "dueDate": "2023-04-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 10,
      "dueDate": "2023-05-22T03:00:00.000Z",
      "accumulatedDays": 300,
      "daysIndex": 0.789223548918967,
      "accumulatedDaysIndex": 8.814696610769053,
      "interestRate": 0.0235,
      "installmentAmount": 1016.21,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10162.1,
      "debitService": 1204.5500000000004,
      "customerDebitServiceAmount": 1204.5500000000004,
      "customerAmount": 1016.21,
      "calculationBasisForEffectiveInterestRate": 1000.4550000000002,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0279,
      "eirYearly": 0.334444,
      "tecYearly": 0.390966,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0279,
      "totalIOF": 157.55,
      "contractAmount": 8957.55,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 157.58,
      "paidContractAmount": 8957.58,
      "preDisbursementAmount": 8800.03,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 805.707575,
          "debitService": 210.502425,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 824.6417030125001,
          "debitService": 191.56829698749996,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 844.0207830332938,
          "debitService": 172.18921696670623,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 863.8552714345763,
          "debitService": 152.3547285654238,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 884.1558703132888,
          "debitService": 132.05412968671126,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 904.933533265651,
          "debitService": 111.27646673434899,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 926.1994712973939,
          "debitService": 90.0105287026062,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 947.9651588728826,
          "debitService": 68.24484112711744,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 970.2423401063953,
          "debitService": 45.9676598936047,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 993.0430350988956,
          "debitService": 23.16696490110441,
          "dueDate": "2023-05-22T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 11,
      "dueDate": "2023-06-20T03:00:00.000Z",
      "accumulatedDays": 329,
      "daysIndex": 0.771102636940274,
      "accumulatedDaysIndex": 9.585799247709327,
      "interestRate": 0.0235,
      "installmentAmount": 935.74,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10293.14,
      "debitService": 1323.3699999999994,
      "customerDebitServiceAmount": 1323.3699999999994,
      "customerAmount": 935.74,
      "calculationBasisForEffectiveInterestRate": 920.3063636363636,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0278,
      "eirYearly": 0.334408,
      "tecYearly": 0.390334,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0278,
      "totalIOF": 169.77,
      "contractAmount": 8969.77,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 169.82,
      "paidContractAmount": 8969.82,
      "preDisbursementAmount": 8800.05,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 724.950405,
          "debitService": 210.78959500000002,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 741.9867395174999,
          "debitService": 193.75326048250002,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 759.4234278961612,
          "debitService": 176.31657210383878,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 777.2698784517211,
          "debitService": 158.470121548279,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 795.5357205953364,
          "debitService": 140.20427940466357,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 814.2308100293269,
          "debitService": 121.50918997067313,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 833.365234065016,
          "debitService": 102.37476593498396,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 852.9493170655439,
          "debitService": 82.79068293445609,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 872.9936260165842,
          "debitService": 62.74637398341579,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 893.5089762279739,
          "debitService": 42.23102377202607,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 914.5064371693313,
          "debitService": 21.23356283066868,
          "dueDate": "2023-06-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 12,
      "dueDate": "2023-07-20T03:00:00.000Z",
      "accumulatedDays": 359,
      "daysIndex": 0.752564915365291,
      "accumulatedDaysIndex": 10.338364163074617,
      "interestRate": 0.0235,
      "installmentAmount": 868.81,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10425.72,
      "debitService": 1443.6099999999992,
      "customerDebitServiceAmount": 1443.6099999999992,
      "customerAmount": 868.81,
      "calculationBasisForEffectiveInterestRate": 853.6341666666666,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0278,
      "eirYearly": 0.334526,
      "tecYearly": 0.389973,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0278,
      "totalIOF": 182.11,
      "contractAmount": 8982.11,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 182.07,
      "paidContractAmount": 8982.07,
      "preDisbursementAmount": 8799.96,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 657.730415,
          "debitService": 211.079585,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 673.1870797524999,
          "debitService": 195.62292024750002,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 689.0069761266836,
          "debitService": 179.80302387331628,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 705.1986400656607,
          "debitService": 163.61135993433922,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 721.7708081072037,
          "debitService": 147.0391918927962,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 738.732422097723,
          "debitService": 130.07757790227691,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 756.0926340170195,
          "debitService": 112.71736598298044,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 773.8608109164195,
          "debitService": 94.94918908358048,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 792.0465399729553,
          "debitService": 76.76346002704463,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 810.6596336623197,
          "debitService": 58.15036633768017,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 829.7101350533843,
          "debitService": 39.099864946615654,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 849.2083232271389,
          "debitService": 19.601676772861122,
          "dueDate": "2023-07-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 13,
      "dueDate": "2023-08-21T03:00:00.000Z",
      "accumulatedDays": 391,
      "daysIndex": 0.734472850574151,
      "accumulatedDaysIndex": 11.072837013648769,
      "interestRate": 0.0235,
      "installmentAmount": 812.16,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10558.08,
      "debitService": 1565.1299999999999,
      "customerDebitServiceAmount": 1565.1299999999999,
      "customerAmount": 812.16,
      "calculationBasisForEffectiveInterestRate": 797.3176923076923,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0278,
      "eirYearly": 0.334443,
      "tecYearly": 0.389081,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0278,
      "totalIOF": 192.95,
      "contractAmount": 8992.95,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 192.92,
      "paidContractAmount": 8992.92,
      "preDisbursementAmount": 8799.97,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 600.8256749999999,
          "debitService": 211.334325,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 614.9450783625,
          "debitService": 197.21492163750003,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 629.3962877040187,
          "debitService": 182.76371229598126,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 644.1871004650632,
          "debitService": 167.97289953493683,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 659.325497325992,
          "debitService": 152.83450267400787,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 674.819646513153,
          "debitService": 137.34035348684705,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 690.677908206212,
          "debitService": 121.48209179378794,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 706.908839049058,
          "debitService": 105.25116095094197,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 723.5211967667109,
          "debitService": 88.63880323328911,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 740.5239448907286,
          "debitService": 71.6360551092714,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 757.9262575956607,
          "debitService": 54.233742404339274,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 775.7375246491587,
          "debitService": 36.422475350841246,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 793.967356478414,
          "debitService": 18.192643521586017,
          "dueDate": "2023-08-21T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 14,
      "dueDate": "2023-09-20T03:00:00.000Z",
      "accumulatedDays": 421,
      "daysIndex": 0.716815728738394,
      "accumulatedDaysIndex": 11.789652742387164,
      "interestRate": 0.0235,
      "installmentAmount": 763.57,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10689.98,
      "debitService": 1687.7199999999996,
      "customerDebitServiceAmount": 1687.7199999999996,
      "customerAmount": 763.57,
      "calculationBasisForEffectiveInterestRate": 749.1228571428571,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0277,
      "eirYearly": 0.334467,
      "tecYearly": 0.388031,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0277,
      "totalIOF": 202.26,
      "contractAmount": 9002.26,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 202.23,
      "paidContractAmount": 9002.23,
      "preDisbursementAmount": 8799.97,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 552.0168900000001,
          "debitService": 211.55311,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 564.9892869150001,
          "debitService": 198.58071308499999,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 578.2665351575026,
          "debitService": 185.30346484249748,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 591.8557987337039,
          "debitService": 171.7142012662962,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 605.7644100039458,
          "debitService": 157.80558999605415,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 619.9998736390387,
          "debitService": 143.5701263609614,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 634.569870669556,
          "debitService": 129.000129330444,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 649.4822626302906,
          "debitService": 114.08773736970944,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 664.7450958021025,
          "debitService": 98.82490419789762,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 680.3666055534518,
          "debitService": 83.2033944465482,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 696.355220783958,
          "debitService": 67.21477921604209,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 712.719568472381,
          "debitService": 50.85043152761908,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 729.468478331482,
          "debitService": 34.101521668518124,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 746.6109875722717,
          "debitService": 16.9590124277283,
          "dueDate": "2023-09-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 15,
      "dueDate": "2023-10-20T03:00:00.000Z",
      "accumulatedDays": 451,
      "daysIndex": 0.699583093595752,
      "accumulatedDaysIndex": 12.489235835982916,
      "interestRate": 0.0235,
      "installmentAmount": 721.45,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10821.75,
      "debitService": 1811.41,
      "customerDebitServiceAmount": 1811.41,
      "customerAmount": 721.45,
      "calculationBasisForEffectiveInterestRate": 707.4273333333333,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0276,
      "eirYearly": 0.334561,
      "tecYearly": 0.386921,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0276,
      "totalIOF": 210.34,
      "contractAmount": 9010.34,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 210.36,
      "paidContractAmount": 9010.36,
      "preDisbursementAmount": 8800.02,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 509.7070100000001,
          "debitService": 211.74299,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 521.685124735,
          "debitService": 199.764875265,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 533.9447251662725,
          "debitService": 187.5052748337275,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 546.49242620768,
          "debitService": 174.9575737923201,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 559.3349982235604,
          "debitService": 162.1150017764396,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 572.4793706818141,
          "debitService": 148.97062931818593,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 585.9326358928367,
          "debitService": 135.5173641071633,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 599.7020528363184,
          "debitService": 121.74794716368163,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 613.795051077972,
          "debitService": 107.65494892202815,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 628.2192347783042,
          "debitService": 93.23076522169582,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 642.9823867955944,
          "debitService": 78.46761320440567,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 658.0924728852908,
          "debitService": 63.3575271147092,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 673.5576459980952,
          "debitService": 47.89235400190487,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 689.3862506790504,
          "debitService": 32.06374932094963,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 705.586827570008,
          "debitService": 15.86317242999195,
          "dueDate": "2023-10-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 16,
      "dueDate": "2023-11-20T03:00:00.000Z",
      "accumulatedDays": 482,
      "daysIndex": 0.683520365008656,
      "accumulatedDaysIndex": 13.172756200991572,
      "interestRate": 0.0235,
      "installmentAmount": 684.55,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 10952.8,
      "debitService": 1935.4299999999994,
      "customerDebitServiceAmount": 1935.4299999999994,
      "customerAmount": 684.55,
      "calculationBasisForEffectiveInterestRate": 670.964375,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0275,
      "eirYearly": 0.334501,
      "tecYearly": 0.385546,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0275,
      "totalIOF": 217.37,
      "contractAmount": 9017.37,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 217.41,
      "paidContractAmount": 9017.41,
      "preDisbursementAmount": 8800.04,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 472.641805,
          "debitService": 211.908195,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 483.74888741749993,
          "debitService": 200.80111258250002,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 495.11698627181113,
          "debitService": 189.4330137281888,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 506.7522354491987,
          "debitService": 177.79776455080125,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 518.6609129822549,
          "debitService": 165.88908701774508,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 530.8494444373379,
          "debitService": 153.7005555626621,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 543.3244063816153,
          "debitService": 141.22559361838464,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 556.0925299315833,
          "debitService": 128.45747006841668,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 569.1607043849755,
          "debitService": 115.38929561502448,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 582.5359809380224,
          "debitService": 102.01401906197756,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 596.2255764900659,
          "debitService": 88.32442350993404,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 610.2368775375825,
          "debitService": 74.31312246241748,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 624.5774441597157,
          "debitService": 59.972555840284286,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 639.255014097469,
          "debitService": 45.29498590253097,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 654.2775069287595,
          "debitService": 30.272493071240447,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 669.6530283415854,
          "debitService": 14.896971658414598,
          "dueDate": "2023-11-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 17,
      "dueDate": "2023-12-20T03:00:00.000Z",
      "accumulatedDays": 512,
      "daysIndex": 0.667088168294041,
      "accumulatedDaysIndex": 13.839844369285613,
      "interestRate": 0.0235,
      "installmentAmount": 652,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11084,
      "debitService": 2060.38,
      "customerDebitServiceAmount": 2060.38,
      "customerAmount": 652,
      "calculationBasisForEffectiveInterestRate": 638.8458823529412,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0275,
      "eirYearly": 0.334508,
      "tecYearly": 0.384214,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0275,
      "totalIOF": 223.62,
      "contractAmount": 9023.62,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 223.58,
      "paidContractAmount": 9023.58,
      "preDisbursementAmount": 8799.96,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 439.94493,
          "debitService": 212.05507000000003,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 450.28363585499994,
          "debitService": 201.71636414500003,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 460.8653012975925,
          "debitService": 191.13469870240752,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 471.6956358780859,
          "debitService": 180.3043641219141,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 482.7804833212209,
          "debitService": 169.21951667877906,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 494.1258246792696,
          "debitService": 157.87417532073036,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 505.7377815592325,
          "debitService": 146.26221844076753,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 517.6226194258744,
          "debitService": 134.37738057412557,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 529.7867509823825,
          "debitService": 122.21324901761751,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 542.2367396304685,
          "debitService": 109.76326036953152,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 554.9793030117845,
          "debitService": 97.02069698821552,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 568.0213166325614,
          "debitService": 83.97868336743859,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 581.3698175734266,
          "debitService": 70.6301824265734,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 595.0320082864022,
          "debitService": 56.967991713597876,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 609.0152604811326,
          "debitService": 42.98473951886742,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 623.3271191024392,
          "debitService": 28.672880897560805,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 637.9753064013465,
          "debitService": 14.024693598653487,
          "dueDate": "2023-12-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 18,
      "dueDate": "2024-01-22T03:00:00.000Z",
      "accumulatedDays": 545,
      "daysIndex": 0.65033128020251,
      "accumulatedDaysIndex": 14.490175649488123,
      "interestRate": 0.0235,
      "installmentAmount": 623.12,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11216.16,
      "debitService": 2187,
      "customerDebitServiceAmount": 2187,
      "customerAmount": 623.12,
      "calculationBasisForEffectiveInterestRate": 610.3888888888889,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0274,
      "eirYearly": 0.334452,
      "tecYearly": 0.38283,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0274,
      "totalIOF": 229.16,
      "contractAmount": 9029.16,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 229.12,
      "paidContractAmount": 9029.12,
      "preDisbursementAmount": 8799.96,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 410.93474000000003,
          "debitService": 212.18526,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 420.59170639,
          "debitService": 202.52829361,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 430.475611490165,
          "debitService": 192.64438850983498,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 440.5917883601839,
          "debitService": 182.5282116398161,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 450.94569538664825,
          "debitService": 172.17430461335178,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 461.54291922823444,
          "debitService": 161.57708077176557,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 472.389177830098,
          "debitService": 150.73082216990204,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 483.4903235091052,
          "debitService": 139.62967649089475,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 494.8523461115692,
          "debitService": 128.26765388843077,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 506.4813762451911,
          "debitService": 116.6386237548089,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 518.3836885869531,
          "debitService": 104.7363114130469,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 530.5657052687465,
          "debitService": 92.5542947312535,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 543.0339993425621,
          "debitService": 80.08600065743796,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 555.7952983271123,
          "debitService": 67.32470167288774,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 568.8564878377994,
          "debitService": 54.263512162200605,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 582.2246153019877,
          "debitService": 40.89538469801232,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 595.9068937615843,
          "debitService": 27.21310623841561,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "545",
          "factor": 0.65033128020251,
          "accumulatedFactor": 14.490175649488123,
          "mainIOFTAC": 609.9107057649817,
          "debitService": 13.209294235018376,
          "dueDate": "2024-01-22T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 19,
      "dueDate": "2024-02-20T03:00:00.000Z",
      "accumulatedDays": 574,
      "daysIndex": 0.635399394424795,
      "accumulatedDaysIndex": 15.125575043912917,
      "interestRate": 0.0235,
      "installmentAmount": 597.27,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11348.13,
      "debitService": 2314.0099999999993,
      "customerDebitServiceAmount": 2314.0099999999993,
      "customerAmount": 597.27,
      "calculationBasisForEffectiveInterestRate": 584.947894736842,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0273,
      "eirYearly": 0.334442,
      "tecYearly": 0.381514,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0273,
      "totalIOF": 234.12,
      "contractAmount": 9034.12,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 234.05,
      "paidContractAmount": 9034.05,
      "preDisbursementAmount": 8799.93,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 384.96817999999996,
          "debitService": 212.30182000000002,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 394.01493223,
          "debitService": 203.25506777,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 403.27428313740495,
          "debitService": 193.995716862595,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 412.751228791134,
          "debitService": 184.51877120886599,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 422.45088266772564,
          "debitService": 174.81911733227435,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 432.3784784104172,
          "debitService": 164.8915215895828,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 442.539372653062,
          "debitService": 154.73062734693798,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 452.93904791040893,
          "debitService": 144.33095208959102,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 463.5831155363036,
          "debitService": 133.68688446369643,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 474.4773187514067,
          "debitService": 122.79268124859328,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 485.62753574206477,
          "debitService": 111.64246425793523,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 497.03978283200325,
          "debitService": 100.23021716799671,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 508.7202177285553,
          "debitService": 88.54978227144463,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 520.6751428451764,
          "debitService": 76.59485715482359,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 532.9110087020381,
          "debitService": 64.35899129796194,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 545.4344174065359,
          "debitService": 51.83558259346404,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 558.2521262155896,
          "debitService": 39.01787378441044,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "545",
          "factor": 0.65033128020251,
          "accumulatedFactor": 14.490175649488123,
          "mainIOFTAC": 571.3710511816558,
          "debitService": 25.898948818344085,
          "dueDate": "2024-01-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.635399394424795,
          "accumulatedFactor": 15.125575043912917,
          "mainIOFTAC": 584.7982708844248,
          "debitService": 12.471729115575174,
          "dueDate": "2024-02-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 20,
      "dueDate": "2024-03-20T03:00:00.000Z",
      "accumulatedDays": 603,
      "daysIndex": 0.620810351163912,
      "accumulatedDaysIndex": 15.74638539507683,
      "interestRate": 0.0235,
      "installmentAmount": 574.01,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11480.2,
      "debitService": 2441.6500000000005,
      "customerDebitServiceAmount": 2441.6500000000005,
      "customerAmount": 574.01,
      "calculationBasisForEffectiveInterestRate": 562.0825000000001,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0272,
      "eirYearly": 0.334442,
      "tecYearly": 0.380287,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0272,
      "totalIOF": 238.55,
      "contractAmount": 9038.55,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 238.58,
      "paidContractAmount": 9038.58,
      "preDisbursementAmount": 8800.03,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 361.60407499999997,
          "debitService": 212.405925,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 370.1017707625,
          "debitService": 203.9082292375,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 378.79916237541875,
          "debitService": 195.21083762458125,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 387.70094269124104,
          "debitService": 186.30905730875892,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 396.8119148444853,
          "debitService": 177.19808515551475,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 406.1369948433306,
          "debitService": 167.87300515666934,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 415.6812142221489,
          "debitService": 158.32878577785107,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 425.4497227563694,
          "debitService": 148.5602772436306,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 435.4477912411441,
          "debitService": 138.5622087588559,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 445.68081433531097,
          "debitService": 128.32918566468902,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 456.1543134721908,
          "debitService": 117.8556865278092,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 466.87393983878724,
          "debitService": 107.13606016121273,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 477.84547742499876,
          "debitService": 96.16452257500123,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 489.07484614448623,
          "debitService": 84.93515385551376,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 500.56810502888163,
          "debitService": 73.44189497111833,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 512.3314554970603,
          "debitService": 61.678544502939616,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 524.3712447012413,
          "debitService": 49.638755298758696,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "545",
          "factor": 0.65033128020251,
          "accumulatedFactor": 14.490175649488123,
          "mainIOFTAC": 536.6939689517204,
          "debitService": 37.31603104827953,
          "dueDate": "2024-01-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.635399394424795,
          "accumulatedFactor": 15.125575043912917,
          "mainIOFTAC": 549.3062772220859,
          "debitService": 24.703722777914102,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.620810351163912,
          "accumulatedFactor": 15.74638539507683,
          "mainIOFTAC": 562.214974736805,
          "debitService": 11.795025263195082,
          "dueDate": "2024-03-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 21,
      "dueDate": "2024-04-22T03:00:00.000Z",
      "accumulatedDays": 636,
      "daysIndex": 0.605215936399937,
      "accumulatedDaysIndex": 16.35160133147677,
      "interestRate": 0.0235,
      "installmentAmount": 553.01,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11613.21,
      "debitService": 2570.609999999999,
      "customerDebitServiceAmount": 2570.609999999999,
      "customerAmount": 553.01,
      "calculationBasisForEffectiveInterestRate": 541.457619047619,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0271,
      "eirYearly": 0.334394,
      "tecYearly": 0.379036,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0271,
      "totalIOF": 242.6,
      "contractAmount": 9042.6,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 242.6,
      "paidContractAmount": 9042.6,
      "preDisbursementAmount": 8800,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 340.5089,
          "debitService": 212.5011,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 348.51085915,
          "debitService": 204.49914085,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 356.70086434002496,
          "debitService": 196.309135659975,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 365.08333465201554,
          "debitService": 187.92666534798443,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 373.6627930163379,
          "debitService": 179.34720698366206,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 382.4438686522219,
          "debitService": 170.5661313477781,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 391.4312995655491,
          "debitService": 161.5787004344509,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 400.6299351053395,
          "debitService": 152.3800648946605,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 410.04473858031497,
          "debitService": 142.96526141968502,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 419.68078993695235,
          "debitService": 133.3292100630476,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 429.54328850047074,
          "debitService": 123.46671149952924,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 439.6375557802318,
          "debitService": 113.37244421976816,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 449.9690383410673,
          "debitService": 103.0409616589327,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 460.5433107420823,
          "debitService": 92.46668925791764,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 471.3660785445213,
          "debitService": 81.6439214554787,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 482.44318139031753,
          "debitService": 70.56681860968244,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 493.78059615299003,
          "debitService": 59.22940384700998,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "545",
          "factor": 0.65033128020251,
          "accumulatedFactor": 14.490175649488123,
          "mainIOFTAC": 505.3844401625853,
          "debitService": 47.62555983741472,
          "dueDate": "2024-01-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.635399394424795,
          "accumulatedFactor": 15.125575043912917,
          "mainIOFTAC": 517.260974506406,
          "debitService": 35.74902549359396,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.620810351163912,
          "accumulatedFactor": 15.74638539507683,
          "mainIOFTAC": 529.4166074073065,
          "debitService": 23.593392592693423,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "636",
          "factor": 0.605215936399937,
          "accumulatedFactor": 16.35160133147677,
          "mainIOFTAC": 541.8578976813783,
          "debitService": 11.15210231862172,
          "dueDate": "2024-04-22T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 22,
      "dueDate": "2024-05-20T03:00:00.000Z",
      "accumulatedDays": 664,
      "daysIndex": 0.59197433987051,
      "accumulatedDaysIndex": 16.94357567134728,
      "interestRate": 0.0235,
      "installmentAmount": 533.9,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11745.8,
      "debitService": 2699.5599999999995,
      "customerDebitServiceAmount": 2699.5599999999995,
      "customerAmount": 533.9,
      "calculationBasisForEffectiveInterestRate": 522.7072727272727,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0271,
      "eirYearly": 0.334374,
      "tecYearly": 0.377836,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0271,
      "totalIOF": 246.24,
      "contractAmount": 9046.24,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 246.18,
      "paidContractAmount": 9046.18,
      "preDisbursementAmount": 8799.94,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 321.31336,
          "debitService": 212.58664,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 328.86422396,
          "debitService": 205.03577604,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 336.59253322305995,
          "debitService": 197.30746677694,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 344.5024577538019,
          "debitService": 189.3975422461981,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 352.5982655110162,
          "debitService": 181.30173448898375,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 360.88432475052514,
          "debitService": 173.01567524947487,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 369.3651063821625,
          "debitService": 164.53489361783753,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 378.04518638214324,
          "debitService": 155.8548136178567,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 386.9292482621237,
          "debitService": 146.97075173787633,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 396.02208559628355,
          "debitService": 137.8779144037164,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 405.32860460779625,
          "debitService": 128.57139539220375,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 414.85382681607945,
          "debitService": 119.04617318392053,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 424.6028917462573,
          "debitService": 109.29710825374268,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 434.58105970229434,
          "debitService": 99.31894029770562,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 444.79371460529825,
          "debitService": 89.10628539470171,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 455.24636689852275,
          "debitService": 78.65363310147721,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 465.9446565206381,
          "debitService": 67.95534347936191,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "545",
          "factor": 0.65033128020251,
          "accumulatedFactor": 14.490175649488123,
          "mainIOFTAC": 476.89435594887306,
          "debitService": 57.00564405112693,
          "dueDate": "2024-01-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.635399394424795,
          "accumulatedFactor": 15.125575043912917,
          "mainIOFTAC": 488.10137331367156,
          "debitService": 45.79862668632841,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.620810351163912,
          "accumulatedFactor": 15.74638539507683,
          "mainIOFTAC": 499.57175558654285,
          "debitService": 34.32824441345713,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "636",
          "factor": 0.605215936399937,
          "accumulatedFactor": 16.35160133147677,
          "mainIOFTAC": 511.3116918428266,
          "debitService": 22.588308157173373,
          "dueDate": "2024-04-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "664",
          "factor": 0.59197433987051,
          "accumulatedFactor": 16.94357567134728,
          "mainIOFTAC": 523.3275166011331,
          "debitService": 10.572483398866948,
          "dueDate": "2024-05-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 23,
      "dueDate": "2024-06-20T03:00:00.000Z",
      "accumulatedDays": 695,
      "daysIndex": 0.577104269859171,
      "accumulatedDaysIndex": 17.52067994120645,
      "interestRate": 0.0235,
      "installmentAmount": 516.51,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 11879.73,
      "debitService": 2830.1499999999996,
      "customerDebitServiceAmount": 2830.1499999999996,
      "customerAmount": 516.51,
      "calculationBasisForEffectiveInterestRate": 505.6586956521739,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.027,
      "eirYearly": 0.334389,
      "tecYearly": 0.376769,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.027,
      "totalIOF": 249.58,
      "contractAmount": 9049.58,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 249.61,
      "paidContractAmount": 9049.61,
      "preDisbursementAmount": 8800.03,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 303.84487,
          "debitService": 212.66513,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 310.985224445,
          "debitService": 205.524775555,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 318.2933772194575,
          "debitService": 198.21662278054248,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 325.77327158411475,
          "debitService": 190.73672841588524,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 333.4289434663415,
          "debitService": 183.08105653365854,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 341.2645236378005,
          "debitService": 175.24547636219953,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 349.28423994328875,
          "debitService": 167.2257600567112,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 357.492419581956,
          "debitService": 159.01758041804393,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 365.893491442132,
          "debitService": 150.61650855786797,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 374.4919884910221,
          "debitService": 142.01801150897788,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 383.2925502205611,
          "debitService": 133.21744977943885,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 392.29992515074434,
          "debitService": 124.21007484925568,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 401.5189733917868,
          "debitService": 114.99102660821319,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 410.9546692664938,
          "debitService": 105.5553307335062,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 420.6121039942564,
          "debitService": 95.89789600574359,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 430.49648843812145,
          "debitService": 86.01351156187856,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 440.61315591641727,
          "debitService": 75.8968440835827,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "545",
          "factor": 0.65033128020251,
          "accumulatedFactor": 14.490175649488123,
          "mainIOFTAC": 450.9675650804531,
          "debitService": 65.5424349195469,
          "dueDate": "2024-01-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.635399394424795,
          "accumulatedFactor": 15.125575043912917,
          "mainIOFTAC": 461.5653028598437,
          "debitService": 54.94469714015626,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.620810351163912,
          "accumulatedFactor": 15.74638539507683,
          "mainIOFTAC": 472.4120874770501,
          "debitService": 44.097912522949926,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "636",
          "factor": 0.605215936399937,
          "accumulatedFactor": 16.35160133147677,
          "mainIOFTAC": 483.51377153276076,
          "debitService": 32.99622846723925,
          "dueDate": "2024-04-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "664",
          "factor": 0.59197433987051,
          "accumulatedFactor": 16.94357567134728,
          "mainIOFTAC": 494.8763451637806,
          "debitService": 21.63365483621937,
          "dueDate": "2024-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "695",
          "factor": 0.577104269859171,
          "accumulatedFactor": 17.52067994120645,
          "mainIOFTAC": 506.50593927512944,
          "debitService": 10.004060724870527,
          "dueDate": "2024-06-20T03:00:00.000Z"
        }
      ]
    },
    {
      "installment": 24,
      "dueDate": "2024-07-22T03:00:00.000Z",
      "accumulatedDays": 727,
      "daysIndex": 0.563230373231306,
      "accumulatedDaysIndex": 18.083910314437755,
      "interestRate": 0.0235,
      "installmentAmount": 500.59,
      "installmentAmountWithoutTAC": 0,
      "totalAmount": 12014.16,
      "debitService": 2961.5299999999997,
      "customerDebitServiceAmount": 2961.5299999999997,
      "customerAmount": 500.59,
      "calculationBasisForEffectiveInterestRate": 490.06374999999997,
      "merchantDebitServiceAmount": 0,
      "merchantTotalAmount": 440,
      "settledToMerchant": 8360,
      "mdrAmount": 440,
      "effectiveInterestRate": 0.0243,
      "totalEffectiveCost": 0.0269,
      "eirYearly": 0.334375,
      "tecYearly": 0.375677,
      "eirMonthly": 0.0243,
      "tecMonthly": 0.0269,
      "totalIOF": 252.63,
      "contractAmount": 9052.63,
      "contractAmountWithoutTAC": 0,
      "tacAmount": 0,
      "IOFPercentage": 0.000082,
      "overallIOF": 0.0038,
      "disbursementDate": "2022-07-26T03:00:00.000Z",
      "paidTotalIOF": 252.62,
      "paidContractAmount": 9052.62,
      "preDisbursementAmount": 8799.99,
      "invoices": [
        {
          "accumulatedDays": "27",
          "factor": 0.9792033719596,
          "accumulatedFactor": 0.9792033719596,
          "mainIOFTAC": 287.853195,
          "debitService": 212.73680499999998,
          "dueDate": "2022-08-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "56",
          "factor": 0.956720441569568,
          "accumulatedFactor": 1.935923813529168,
          "mainIOFTAC": 294.6177450825,
          "debitService": 205.97225491749998,
          "dueDate": "2022-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "86",
          "factor": 0.933720368270266,
          "accumulatedFactor": 2.8696441817994343,
          "mainIOFTAC": 301.54126209193873,
          "debitService": 199.04873790806124,
          "dueDate": "2022-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "118",
          "factor": 0.911273229087126,
          "accumulatedFactor": 3.7809174108865604,
          "mainIOFTAC": 308.62748175109925,
          "debitService": 191.9625182489007,
          "dueDate": "2022-11-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "147",
          "factor": 0.89035000398133,
          "accumulatedFactor": 4.6712674148678905,
          "mainIOFTAC": 315.88022757225013,
          "debitService": 184.70977242774984,
          "dueDate": "2022-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "178",
          "factor": 0.867984901303578,
          "accumulatedFactor": 5.539252316171469,
          "mainIOFTAC": 323.303412920198,
          "debitService": 177.28658707980196,
          "dueDate": "2023-01-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "209",
          "factor": 0.8480555948138,
          "accumulatedFactor": 6.387307910985269,
          "mainIOFTAC": 330.90104312382266,
          "debitService": 169.68895687617731,
          "dueDate": "2023-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "237",
          "factor": 0.829500878479253,
          "accumulatedFactor": 7.2168087894645225,
          "mainIOFTAC": 338.6772176372325,
          "debitService": 161.91278236276747,
          "dueDate": "2023-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "268",
          "factor": 0.808664272385564,
          "accumulatedFactor": 8.025473061850086,
          "mainIOFTAC": 346.63613225170747,
          "debitService": 153.9538677482925,
          "dueDate": "2023-04-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "300",
          "factor": 0.789223548918967,
          "accumulatedFactor": 8.814696610769053,
          "mainIOFTAC": 354.78208135962257,
          "debitService": 145.80791864037738,
          "dueDate": "2023-05-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "329",
          "factor": 0.771102636940274,
          "accumulatedFactor": 9.585799247709327,
          "mainIOFTAC": 363.11946027157376,
          "debitService": 137.47053972842625,
          "dueDate": "2023-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "359",
          "factor": 0.752564915365291,
          "accumulatedFactor": 10.338364163074617,
          "mainIOFTAC": 371.65276758795574,
          "debitService": 128.93723241204427,
          "dueDate": "2023-07-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "391",
          "factor": 0.734472850574151,
          "accumulatedFactor": 11.072837013648769,
          "mainIOFTAC": 380.3866076262727,
          "debitService": 120.2033923737273,
          "dueDate": "2023-08-21T03:00:00.000Z"
        },
        {
          "accumulatedDays": "421",
          "factor": 0.716815728738394,
          "accumulatedFactor": 11.789652742387164,
          "mainIOFTAC": 389.3256929054901,
          "debitService": 111.26430709450987,
          "dueDate": "2023-09-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "451",
          "factor": 0.699583093595752,
          "accumulatedFactor": 12.489235835982916,
          "mainIOFTAC": 398.4748466887691,
          "debitService": 102.11515331123087,
          "dueDate": "2023-10-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "482",
          "factor": 0.683520365008656,
          "accumulatedFactor": 13.172756200991572,
          "mainIOFTAC": 407.8390055859552,
          "debitService": 92.7509944140448,
          "dueDate": "2023-11-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "512",
          "factor": 0.667088168294041,
          "accumulatedFactor": 13.839844369285613,
          "mainIOFTAC": 417.4232222172251,
          "debitService": 83.16677778277486,
          "dueDate": "2023-12-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "545",
          "factor": 0.65033128020251,
          "accumulatedFactor": 14.490175649488123,
          "mainIOFTAC": 427.2326679393299,
          "debitService": 73.35733206067007,
          "dueDate": "2024-01-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "574",
          "factor": 0.635399394424795,
          "accumulatedFactor": 15.125575043912917,
          "mainIOFTAC": 437.2726356359042,
          "debitService": 63.317364364095816,
          "dueDate": "2024-02-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "603",
          "factor": 0.620810351163912,
          "accumulatedFactor": 15.74638539507683,
          "mainIOFTAC": 447.5485425733479,
          "debitService": 53.041457426652066,
          "dueDate": "2024-03-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "636",
          "factor": 0.605215936399937,
          "accumulatedFactor": 16.35160133147677,
          "mainIOFTAC": 458.0659333238216,
          "debitService": 42.524066676178386,
          "dueDate": "2024-04-22T03:00:00.000Z"
        },
        {
          "accumulatedDays": "664",
          "factor": 0.59197433987051,
          "accumulatedFactor": 16.94357567134728,
          "mainIOFTAC": 468.8304827569314,
          "debitService": 31.75951724306858,
          "dueDate": "2024-05-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "695",
          "factor": 0.577104269859171,
          "accumulatedFactor": 17.52067994120645,
          "mainIOFTAC": 479.84799910171927,
          "debitService": 20.742000898280693,
          "dueDate": "2024-06-20T03:00:00.000Z"
        },
        {
          "accumulatedDays": "727",
          "factor": 0.563230373231306,
          "accumulatedFactor": 18.083910314437755,
          "mainIOFTAC": 491.1244270806097,
          "debitService": 9.465572919390292,
          "dueDate": "2024-07-22T03:00:00.000Z"
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
