<?php
/* src/internal/Response/FFIInvoiceVec.php */

namespace ParceladoLara\PaymentPlan\Internal\Response;

use ParceladoLara\PaymentPlan\Response\Invoice;

/**
 * @internal
 * Internal representation of Vec_Invoice_t for FFI.
 */
class FFIInvoiceVec
{
  /** @var \FFI\CData|FFIInvoice Pointer to FFIInvoice (Invoice_t*) */
  public $ptr;
  /** @var int */
  public int $len;
  /** @var int */
  public int $cap;

  /**
   * Convert the FFIInvoiceVec to an array of Invoice objects.
   * @return Invoice[]
   */
  public function toArray(): array
  {
    $invoices = [];
    if ($this->ptr === null || $this->len === 0) {
      return $invoices;
    }
    for ($i = 0; $i < $this->len; $i++) {
      $cInvoice = $this->ptr[$i];
      $invoice = new Invoice();
      $invoice->accumulatedDays = $cInvoice->accumulated_days;
      $invoice->factor = $cInvoice->factor;
      $invoice->accumulatedFactor = $cInvoice->accumulated_factor;
      $invoice->dueDate = new \DateTimeImmutable('@' . $cInvoice->due_date_ms / 1000);
      $invoices[] = $invoice;
    }
    return $invoices;
  }
}
