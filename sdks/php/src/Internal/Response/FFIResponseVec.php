<?php
/* src/internal/Response/FFIResponseVec.php */

namespace ParceladoLara\PaymentPlan\Internal\Response;

use ParceladoLara\PaymentPlan\Response\Response;

/**
 * @internal
 * Internal representation of Vec_Response_t for FFI.
 */
class FFIResponseVec
{
  /** @var \FFI\CData|FFIResponse  Pointer to FFIResponse (Response_t*) */
  public $ptr;
  /** @var int */
  public int $len;
  /** @var int */
  public int $cap;

  /**
   * Convert the FFIResponseVec to an array of Response objects.
   * @return Response[]
   */
  public function toArray(): array
  {
    $responses = [];

    if ($this->ptr === null || $this->len === 0) {
      return $responses;
    }

    for ($i = 0; $i < $this->len; $i++) {
      /** @var FFIResponse $cResponse */
      $cResponse = $this->ptr[$i];
      $response = new Response();

      $response->installment = $cResponse->installment;
      $response->dueDate = new \DateTimeImmutable('@' . $cResponse->due_date_ms / 1000);
      $response->disbursementDate = new \DateTimeImmutable('@' . $cResponse->disbursement_date_ms / 1000);
      $response->accumulatedDays = $cResponse->accumulated_days;
      $response->daysIndex = $cResponse->days_index;
      $response->accumulatedDaysIndex = $cResponse->accumulated_days_index;
      $response->interestRate = $cResponse->interest_rate;
      $response->installmentAmount = $cResponse->installment_amount;
      $response->installmentAmountWithoutTac = $cResponse->installment_amount_without_tac;
      $response->totalAmount = $cResponse->total_amount;
      $response->debitService = $cResponse->debit_service;
      $response->customerDebitServiceAmount = $cResponse->customer_debit_service_amount;
      $response->customerAmount = $cResponse->customer_amount;
      $response->calculationBasisForEffectiveInterestRate = $cResponse->calculation_basis_for_effective_interest_rate;
      $response->merchantDebitServiceAmount = $cResponse->merchant_debit_service_amount;
      $response->merchantTotalAmount = $cResponse->merchant_total_amount;
      $response->settledToMerchant = $cResponse->settled_to_merchant;
      $response->mdrAmount = $cResponse->mdr_amount;
      $response->effectiveInterestRate = $cResponse->effective_interest_rate;
      $response->totalEffectiveCost = $cResponse->total_effective_cost;
      $response->eirYearly = $cResponse->eir_yearly;
      $response->tecYearly = $cResponse->tec_yearly;
      $response->eirMonthly = $cResponse->eir_monthly;
      $response->tecMonthly = $cResponse->tec_monthly;
      $response->totalIof = $cResponse->total_iof;
      $response->contractAmount = $cResponse->contract_amount;
      $response->contractAmountWithoutTac = $cResponse->contract_amount_without_tac;
      $response->tacAmount = $cResponse->tac_amount;
      $response->iofPercentage = $cResponse->iof_percentage;
      $response->overallIof = $cResponse->overall_iof;
      $response->preDisbursementAmount = $cResponse->pre_disbursement_amount;
      $response->paidTotalIof = $cResponse->paid_total_iof;
      $response->paidContractAmount = $cResponse->paid_contract_amount;

      // Handle invoices
      if ($cResponse->invoices !== null) {
        $ffiInvoiceVec = new FFIInvoiceVec();
        $ffiInvoiceVec->ptr = $cResponse->invoices->ptr;
        $ffiInvoiceVec->len = $cResponse->invoices->len;
        $ffiInvoiceVec->cap = $cResponse->invoices->cap;
        $response->invoices = $ffiInvoiceVec->toArray();
      } else {
        $response->invoices = [];
      }

      $responses[] = $response;
    }
    return $responses;
  }

  public static function fromCData(\FFI\CData $cData): self
  {
    $vec = new self();
    $vec->ptr = $cData->ptr;
    $vec->len = $cData->len;
    $vec->cap = $cData->cap;
    return $vec;
  }
}
