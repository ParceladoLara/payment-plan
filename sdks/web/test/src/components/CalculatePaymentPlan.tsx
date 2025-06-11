import { calculatePaymentPlan, Params } from '@parcelado_lara/payment-plan-wasm'
import React from 'react'

const CalculatePaymentPlan: React.FC = () => {
  const params: Params = {
    maxTotalAmount: Number.MAX_VALUE,
    minInstallmentAmount: 0.0,
    requestedAmount: 8800.0,
    firstPaymentDate: new Date('2022-04-18'),
    disbursementDate: new Date('2022-03-18'),
    installments: 24,
    debitServicePercentage: 0,
    mdr: 0.05,
    tacPercentage: 0.0,
    iofOverall: 0.0038,
    iofPercentage: 0.000082,
    interestRate: 0.0235,
    disbursementOnlyOnBusinessDays: false
  }
  const result = calculatePaymentPlan(params)
  const paymentPlan = result.pop()
  const str = JSON.stringify(paymentPlan, (key, value) =>
    typeof value === 'bigint' ? value.toString() : value,
    2
  )
  return <pre>{str}</pre>
}

export default CalculatePaymentPlan
