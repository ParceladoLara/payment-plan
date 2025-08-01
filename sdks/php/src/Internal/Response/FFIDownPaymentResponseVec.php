<?php
/* src/internal/Response/FFIDownPaymentResponse.php */

namespace ParceladoLara\PaymentPlan\Internal\Response;

use ParceladoLara\PaymentPlan\Response\DownPaymentResponse;

/**
 * @internal
 * Internal representation of DownPaymentResponse_t for FFI.
 */
class FFIDownPaymentResponseVec
{

  /** @var \FFI\CData|FFIDownPaymentResponse  Pointer to FFIDownPaymentResponse (DownPaymentResponse_t*) */
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
      $cResponse = $this->ptr[$i];
      $response = new DownPaymentResponse();

      $response->installmentAmount = $cResponse->installment_amount;
      $response->totalAmount = $cResponse->total_amount;
      $response->installmentQuantity = $cResponse->installment_quantity;
      $response->firstPaymentDate = new \DateTimeImmutable('@' . $cResponse->first_payment_date_ms / 1000);

      if ($cResponse->plans !== null) {
        $plans = $cResponse->plans;
        $respVec = FFIResponseVec::fromCData($plans);
        $response->plans = $respVec->toArray();
      } else {
        $response->plans = [];
      }

      $responses[] = $response;
    }

    return $responses;
  }
}
