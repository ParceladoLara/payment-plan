<?php
/* src/internal/Response/FFIInvoice.php */

namespace ParceladoLara\PaymentPlan\Internal\Response;

/**
 * @internal
 * Internal representation of Invoice_t for FFI.
 */
class FFIInvoice
{
  public int $accumulated_days;
  public float $factor;
  public float $accumulated_factor;
  public float $main_iof_tac;
  public float $debit_service;
  public int $due_date_ms;
}
