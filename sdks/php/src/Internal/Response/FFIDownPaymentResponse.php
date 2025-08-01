<?php
/* src/internal/Response/FFIDownPaymentResponse.php */

namespace ParceladoLara\PaymentPlan\Internal\Response;

/**
 * @internal
 * Internal representation of DownPaymentResponse_t for FFI.
 */
class FFIDownPaymentResponse
{
  public float $installment_amount;
  public float $total_amount;
  public int $installment_quantity;
  public int $first_payment_date_ms;
  public FFIResponseVec $plans;
}
