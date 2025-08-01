<?php
/* src/internal/FFIDownPaymentParams.php */

namespace ParceladoLara\PaymentPlan\Internal\Params;

/**
 * @internal
 * Internal representation of DownPaymentParams for FFI (matches C struct layout).
 */
class FFIDownPaymentParams
{
  public float $requested_amount;
  public float $min_installment_amount;
  public int $first_payment_date_ms;
  public int $installments;
  public FFIParams $params;

  public function __construct(
    float $requested_amount,
    float $min_installment_amount,
    int $first_payment_date_ms,
    int $installments,
    FFIParams $params
  ) {
    $this->requested_amount = $requested_amount;
    $this->min_installment_amount = $min_installment_amount;
    $this->first_payment_date_ms = $first_payment_date_ms;
    $this->installments = $installments;
    $this->params = $params;
  }

  /**
   * Create an FFIDownPaymentParams from a public DownPaymentParams object.
   */
  public static function fromParams(\ParceladoLara\PaymentPlan\Params\DownPaymentParams $params): self
  {
    return new self(
      $params->requestedAmount,
      $params->minInstallmentAmount,
      $params->firstPaymentDate->getTimestamp() * 1000, // Convert to milliseconds
      $params->installments,
      FFIParams::fromParams($params->params)
    );
  }

  public function toCData(\FFI\CData $data): \FFI\CData
  {
    $data->requested_amount = $this->requested_amount;
    $data->min_installment_amount = $this->min_installment_amount;
    $data->first_payment_date_ms = $this->first_payment_date_ms;
    $data->installments = $this->installments;
    $data->params = $this->params->toCData($data->params);

    return $data;
  }
}
