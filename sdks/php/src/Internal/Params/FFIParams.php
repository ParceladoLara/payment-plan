<?php
/* src/internal/FFIParams.php */

namespace ParceladoLara\PaymentPlan\Internal\Params;

/**
 * @internal
 * Internal representation of Params for FFI (matches C struct layout).
 */
class FFIParams
{
  public float $requested_amount;
  public int $first_payment_date_ms;
  public int $disbursement_date_ms;
  public int $installments;
  public int $debit_service_percentage;
  public float $mdr;
  public float $tac_percentage;
  public float $iof_overall;
  public float $iof_percentage;
  public float $interest_rate;
  public float $min_installment_amount;
  public float $max_total_amount;
  public bool $disbursement_only_on_business_days;

  public function __construct(
    float $requested_amount,
    int $first_payment_date_ms,
    int $disbursement_date_ms,
    int $installments,
    int $debit_service_percentage,
    float $mdr,
    float $tac_percentage,
    float $iof_overall,
    float $iof_percentage,
    float $interest_rate,
    float $min_installment_amount,
    float $max_total_amount,
    bool $disbursement_only_on_business_days
  ) {
    $this->requested_amount = $requested_amount;
    $this->first_payment_date_ms = $first_payment_date_ms;
    $this->disbursement_date_ms = $disbursement_date_ms;
    $this->installments = $installments;
    $this->debit_service_percentage = $debit_service_percentage;
    $this->mdr = $mdr;
    $this->tac_percentage = $tac_percentage;
    $this->iof_overall = $iof_overall;
    $this->iof_percentage = $iof_percentage;
    $this->interest_rate = $interest_rate;
    $this->min_installment_amount = $min_installment_amount;
    $this->max_total_amount = $max_total_amount;
    $this->disbursement_only_on_business_days = $disbursement_only_on_business_days;
  }

  /**
   * Create an FFIParams from a public Params object.
   */
  public static function fromParams(\ParceladoLara\PaymentPlan\Params\Params $params): self
  {
    return new self(
      $params->requestedAmount,
      $params->firstPaymentDate->getTimestamp() * 1000,
      $params->disbursementDate->getTimestamp() * 1000,
      $params->installments,
      $params->debitServicePercentage,
      $params->mdr,
      $params->tacPercentage,
      $params->iofOverall,
      $params->iofPercentage,
      $params->interestRate,
      $params->minInstallmentAmount,
      $params->maxTotalAmount,
      $params->disbursementOnlyOnBusinessDays
    );
  }

  public function toCData(\FFI\CData $data): \FFI\CData
  {
    $data->requested_amount = $this->requested_amount;
    $data->first_payment_date_ms = $this->first_payment_date_ms;
    $data->disbursement_date_ms = $this->disbursement_date_ms;
    $data->installments = $this->installments;
    $data->debit_service_percentage = $this->debit_service_percentage;
    $data->mdr = $this->mdr;
    $data->tac_percentage = $this->tac_percentage;
    $data->iof_overall = $this->iof_overall;
    $data->iof_percentage = $this->iof_percentage;
    $data->interest_rate = $this->interest_rate;
    $data->min_installment_amount = $this->min_installment_amount;
    $data->max_total_amount = $this->max_total_amount;
    $data->disbursement_only_on_business_days = $this->disbursement_only_on_business_days;

    return $data;
  }
}
